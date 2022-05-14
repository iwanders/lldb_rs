#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_rust_codeblocks)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)] // something returns an u128, ignore it.
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    pub use root::*;
}

use bindings::lldb;

// Singleton to ensure we call initialize once before we create the first debugger.
use std::sync::Once;
static START: Once = Once::new();

// https://lldb.llvm.org/python_api/lldb.SBDebugger.html
// https://github.com/llvm/llvm-project/blob/llvmorg-13.0.1/lldb/include/lldb/API/SBDebugger.h
pub struct SBDebugger {
    dbg: lldb::SBDebugger,
}

impl SBDebugger {
    pub fn new() -> Self {
        START.call_once(|| unsafe {
            lldb::SBDebugger::Initialize();
        });
        SBDebugger {
            dbg: unsafe { lldb::SBDebugger::Create() },
        }
    }

    pub fn set_async(&mut self, state: bool) {
        unsafe {
            self.dbg.SetAsync(state);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_version() {
        unsafe {
            let v = std::ffi::CStr::from_ptr(bindings::lldb::SBDebugger::GetVersionString());
            println!("Version: {v:?}");
        }
    }

    #[test]
    fn try_debugger() {
        unsafe {
            bindings::lldb::SBDebugger::Initialize();
            let mut _dbg = bindings::lldb::SBDebugger::Create();
        }
    }
}
