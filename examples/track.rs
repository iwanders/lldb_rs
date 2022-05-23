#![allow(non_upper_case_globals)]
// #![allow(non_snake_case)]

mod util;
use util::*;

use lldb::wrappers::*;

use lldb::api::ffi::lldb as bindings;
use lldb::autocxx::prelude::*;

// Helpers to set breakpoints more easily.
type BreakPoints = std::collections::HashMap<u64, UniquePtr<bindings::SBBreakpoint>>;
#[derive(Copy, Clone, Debug)]
pub enum BP {
    OneShot,
    Enabled,
    Disabled,
}

struct BPProperties<'a> {
    pub bp: &'a mut UniquePtr<bindings::SBBreakpoint>,
}
impl<'a> BPProperties<'a> {
    pub fn set(&mut self, flag: BP) -> &mut Self {
        match flag {
            BP::OneShot => {
                self.bp.pin_mut().SetOneShot(true);
            }
            BP::Enabled => {
                self.bp.pin_mut().SetEnabled(true);
            }
            BP::Disabled => {
                self.bp.pin_mut().SetEnabled(false);
            }
        }
        self
    }
}
trait BPSetter<'a> {
    fn set(&'a mut self, bp: u64) -> BPProperties<'a>;
}

impl<'a> BPSetter<'a> for BreakPoints {
    fn set(&'a mut self, bp: u64) -> BPProperties<'a> {
        BPProperties {
            bp: self.get_mut(&bp).unwrap(),
        }
    }
}

// Actually handlers for a breakpoint.
type CallbackRet = Result<(), Box<dyn std::error::Error>>;
type BreakCallback = Box<dyn Fn(&mut CallBackInfo) -> CallbackRet>;

const pc_allocation_entry: u64 = 0x6ff6cd50; // program counter for this breakpoint.
pub fn break_allocation_entry(info: &mut CallBackInfo) -> CallbackRet {
    // Ensure we capture the return, enable that breakpoint.
    info.bp.set(pc_allocation_return).set(BP::Enabled);

    // https://github.com/llvm/llvm-project/blob/llvmorg-13.0.1/lldb/include/lldb/API/SBFrame.h
    let mut sp_mut = info.sp.borrow_mut();
    let mut thread = sp_mut.process().thread(0);
    let mut frame = thread.frame(0);
    let process = sp_mut.process();

    // Obtain all the information we want to have.
    let size = frame.find_register("edx").get_value_usize()?;
    // let esp = frame.find_register("esp");
    let mut path_ptr = frame.evaluate_expression("(((uint32_t*)$esp)[1])");
    let path = process.read_cstring_from_memory(path_ptr.get_value_unsigned()?, 64)?;
    let linenr = frame
        .evaluate_expression("((const uint32_t)((uint32_t*)$esp)[2])")
        .get_value_u64()?;
    let path = path.into_string()?;

    // Print this part of the allocation.
    print!("Alloc size: 0x{:0>4x} ({path: >60}:{linenr: <5?}) ", size);

    // If we already have something pending, we missed the return?
    if info.data.pending_alloc.is_some() {
        panic!(
            "Already had pending allocation: {:?}",
            info.data.pending_alloc
        );
    }

    // Finally, add the pending allocation to the vector.
    let alloc = Allocation {
        size,
        path,
        linenr,
        address: 0,
        ..Default::default()
    };
    info.data.pending_alloc = Some(alloc);

    // Continue the process
    sp_mut.start()?;
    Ok(())
}

const pc_allocation_return: u64 = 0x6ff6cd8a;
pub fn break_allocation_return(info: &mut CallBackInfo) -> CallbackRet {
    // Disable this breakpoint, now that we have caught the return
    info.bp.set(pc_allocation_return).set(BP::Disabled);

    let mut sp_mut = info.sp.borrow_mut();
    let mut thread = sp_mut.process().thread(0);
    let process = sp_mut.process();
    let mut frame = thread.frame(0);

    // Collect more information about the return.
    let mut path_ptr = frame.evaluate_expression("(((uint32_t*)$esp)[3])");
    let linenr = frame
        .evaluate_expression("((const uint32_t)((uint32_t*)$esp)[4])")
        .get_value_u64()?;
    let path = process.read_cstring_from_memory(path_ptr.get_value_unsigned()?, 64)?;
    let path = path.into_string()?;
    let return_ptr = frame.find_register("eax").get_value_unsigned()?;

    // Print the second half of the allocation.
    println!("-> 0x{return_ptr:0>8X}  ({path:>50}:{linenr: <5?})");

    // If we didn't have a pending one, we must have missed something.
    if info.data.pending_alloc.is_none() {
        panic!("No pending alloc to return from.");
    }

    // Check if the return information matches the pending information.
    let mut alloc = info.data.pending_alloc.take().unwrap();
    if alloc.linenr != linenr || alloc.path != path {
        panic!("Pending alloc differs, we have ({path:>30}:{linenr: <5?}) got {alloc:?}.");
    }

    // Finalize the allocation.
    alloc.address = return_ptr;
    info.data.allocations.push(alloc);

    sp_mut.start()?;
    Ok(())
}

use std::cell::RefCell;
use std::rc::Rc;

type SharedProcess = Rc<RefCell<TargetProcess>>;

/// Struct passed to the callbacks, to write to data and manipulate the shared process and
/// breakpoints.
pub struct CallBackInfo<'a> {
    sp: &'a SharedProcess,
    bp: &'a mut BreakPoints,
    data: &'a mut Data,
}

/// Struct to represent an allocation.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Allocation {
    size: usize,
    path: String,
    linenr: u64,
    address: u64,
    data: Option<Vec<u8>>,
}

/// Struct to hold data the handlers will collect.
#[derive(Default)]
pub struct Data {
    allocations: Vec<Allocation>,
    pending_alloc: Option<Allocation>,
}

/// Struct to track things in a program.
struct ProgramTracker {
    tp: SharedProcess,
    bp: RefCell<BreakPoints>,
    callbacks: std::collections::HashMap<u64, BreakCallback>,
    data: Data,
}

impl ProgramTracker {
    /// Create a new program tracker using the given process.
    pub fn new(tp: TargetProcess) -> ProgramTracker {
        ProgramTracker {
            tp: Rc::new(RefCell::new(tp)),
            bp: RefCell::new(std::collections::HashMap::new()),
            callbacks: std::collections::HashMap::new(),
            data: Default::default(),
        }
    }

    /// Helper to create a breakpoint.
    fn break_instruction<'a>(&'a mut self, address: u64, cb: BreakCallback) {
        let z = self
            .tp
            .borrow_mut()
            .target()
            .pin_mut()
            .BreakpointCreateByAddress(address)
            .within_unique_ptr();
        self.bp.borrow_mut().insert(address, z);
        // Always set as disabled.
        self.bp.borrow_mut().set(address).set(BP::Disabled);
        self.callbacks.insert(address, cb);
    }

    /// Method to enable the appropriate breakpoints for tracking allocations.
    pub fn register_bp_track_allocations(&mut self) {
        self.break_instruction(
            pc_allocation_entry,
            Box::new(|z: _| break_allocation_entry(z)),
        );
        self.bp
            .borrow_mut()
            .set(pc_allocation_entry)
            .set(BP::Enabled);

        self.break_instruction(
            pc_allocation_return,
            Box::new(|z: _| break_allocation_return(z)),
        );
        // self.bp.borrow_mut().set(pc_allocation_return).set(BP::OneShot);
    }

    /// Go into a loop, starting the program and waiting for stop events, calling the appropriate
    /// handler based on the program counter.
    pub fn go(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.tp.borrow_mut().start()?;
        loop {
            let event = self.tp.borrow_mut().wait_for_event();
            if let Some(_event) = event {
                // println!("{:?}", z);
                // If we stopped, do things.
                // find the program counter, and then dispatch.
                let pc;
                {
                    let mut thread = self.tp.borrow_mut().process().thread(0);
                    let mut frame = thread.frame(0);
                    pc = frame.pin_mut().GetPC();
                }

                // obtain the callback from the map.
                let callback_from_map = self.callbacks.get(&pc);

                // Create the struct with borrows to be passed to the callback.
                let mut cb_info = CallBackInfo {
                    sp: &self.tp,
                    bp: &mut self.bp.borrow_mut(),
                    data: &mut self.data,
                };

                if callback_from_map.is_none() {
                    panic!("No callback for this instruction address");
                    // If we had watchpoints, we would handle them here.
                } else {
                    let cb = callback_from_map.unwrap();
                    (cb)(&mut cb_info)?;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pd = ProcessDebugger::new();
    pd.dbg_mut().SetAsync(true);

    // Attach to the wine process
    let mut p = pd.attach_to_name("wine")?;

    // Print the number of supported hardware watchpoints, this is super limited... :(
    let supported = p.process().get_num_supported_hardware_watchpoints()?;
    println!("Supported watchpoints: {supported}");

    // FInally, create the tracker start tracking allocations and resume the process.
    let mut prog = ProgramTracker::new(p);
    prog.register_bp_track_allocations();
    prog.go()?;
    println!("exit");
    Ok(())
}
