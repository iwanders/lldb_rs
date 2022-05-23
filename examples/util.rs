// Very crufty utility functions.
#![allow(dead_code)]

use lldb;
use lldb::api::ffi::lldb as bindings;
use lldb::autocxx::prelude::*;
use lldb::wrappers::*;

pub type BError = Box<dyn std::error::Error>;
#[derive(Debug)]
struct TextError {
    m: String,
}

impl TextError {
    fn new(msg: &str) -> TextError {
        TextError { m: msg.to_string() }
    }
}
impl std::error::Error for TextError {
    fn description(&self) -> &str {
        &self.m
    }
}
impl std::fmt::Display for TextError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.m)
    }
}

///---
use std::boxed::Box;
use std::pin::Pin;

/// Helper to work with lldb
pub struct TargetProcess {
    process: UniquePtr<bindings::SBProcess>,
    target: UniquePtr<bindings::SBTarget>,
    listener: UniquePtr<bindings::SBListener>,
}

impl TargetProcess {
    /// Attach to a process by name.
    pub fn attach_to_name(
        dbg: &mut Pin<&mut bindings::SBDebugger>,
        name: &str,
    ) -> Result<TargetProcess, Box<dyn std::error::Error>> {
        let mut listener = dbg.as_mut().GetListener().within_unique_ptr();
        let mut err = bindings::SBError::new().wrap();
        let mut target = dbg.as_mut().GetDummyTarget().within_unique_ptr();

        let wait_for: bool = false;
        let z = std::ffi::CString::new(name).expect("ascii string");
        let process = unsafe {
            target
                .as_mut()
                .unwrap()
                .AttachToProcessWithName(listener.pin_mut(), z.as_ptr(), wait_for, err.pin_mut())
                .within_unique_ptr()
        };
        if err.is_fail() {
            return Err(Box::new(TextError::new(&format!(
                "Failed to attach (wrong pid/name? or lldb-server address hardcoded? symlink to \
                 ./target/debug/lldb-server-13.0.1 \
                 or maybe target/debug/examples/lldb-server-13.0.1. When in doubt use \
                 strace to figure out where it reports 'ENOENT (No such file or directory)' \
                 or run echo 0 > /proc/sys/kernel/yama/ptrace_scope to allow non root \
                 : {err:}"
            ))));
            // stat("<snip>tracker/target/debug/lldb-server-13.0.1", 0x7ffd5c09d368) = -1 ENOENT (No such file or directory)
            // Thanks lldb.
        }
        Ok(TargetProcess {
            listener,
            target,
            process,
        })
    }

    /// Block on waiting for an event.
    pub fn wait_for_event(&mut self) -> Option<Wrapped<bindings::SBEvent>> {
        let mut event = bindings::SBEvent::new().wrap();
        // https://github.com/llvm/llvm-project/blob/llvmorg-13.0.1/lldb/source/API/SBListener.cpp#L142-L165
        // returns true only if there was an event to get, always populates the event.
        let res = self.listener.pin_mut().WaitForEvent(10, event.pin_mut());
        if res {
            // println!("event: {event:?}");
            return Some(event);
        }

        None
    }

    /// Stop the process.
    pub fn stop(&mut self) -> Result<(), BError> {
        let err = self.process.pin_mut().Stop().wrap();
        if err.is_fail() {
            return Err(Box::new(err));
        }
        let e = self.wait_for_event().expect("must get event");
        let event_type = e.event_type();
        if event_type == bindings::StateType::eStateStopped {
            return Ok(());
        } else {
            return Err(Box::new(TextError::new(&format!(
                "Type was not expected, got {event_type:?}"
            ))));
        }
    }

    /// Continue the process.
    pub fn start(&mut self) -> Result<(), BError> {
        let err = self.process.pin_mut().Continue().wrap();
        if err.is_fail() {
            return Err(Box::new(err));
        }
        let e = self.wait_for_event().expect("must get event");
        let event_type = e.event_type();
        if event_type == bindings::StateType::eStateRunning {
            return Ok(());
        } else {
            return Err(Box::new(TextError::new(&format!(
                "Type was not expected, got {event_type:?}"
            ))));
        }
    }

    /// Mutable borrow the target.
    pub fn target(&mut self) -> &mut UniquePtr<bindings::SBTarget> {
        &mut self.target
    }

    /// Mutable borrow the process.
    pub fn process(&mut self) -> &mut UniquePtr<bindings::SBProcess> {
        &mut self.process
    }
}

// Singleton to ensure we call initialize once before we create the first debugger.
use std::sync::Once;
static START: Once = Once::new();

pub struct ProcessDebugger {
    dbg: Pin<Box<bindings::SBDebugger>>,
}

impl ProcessDebugger {
    pub fn new() -> Self {
        START.call_once(|| {
            bindings::SBDebugger::Initialize();
        });

        let res = bindings::SBDebugger::Create().within_box();

        ProcessDebugger { dbg: res }
    }

    pub fn dbg_ref(&self) -> Pin<&bindings::SBDebugger> {
        self.dbg.as_ref()
    }
    pub fn dbg_mut(&mut self) -> Pin<&mut bindings::SBDebugger> {
        self.dbg.as_mut()
    }

    pub fn attach_to_name(
        &mut self,
        name: &str,
    ) -> Result<TargetProcess, Box<dyn std::error::Error>> {
        TargetProcess::attach_to_name(&mut self.dbg_mut(), name)
    }
}
