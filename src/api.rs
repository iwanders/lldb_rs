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
    dbg: bindings::lldb::SBDebugger,
}

impl SBDebugger {
    pub fn new() -> Self {
        START.call_once(|| unsafe {
            println!("Initialize.");
            bindings::lldb::SBDebugger::Initialize();
        });
        SBDebugger {
            dbg: unsafe { bindings::lldb::SBDebugger::Create() },
        }
    }
}

// Sugar to dereference to get the unsafe thing.
impl std::ops::Deref for SBDebugger {
    type Target = bindings::lldb::SBDebugger;

    fn deref(&self) -> &Self::Target {
        &self.dbg
    }
}
impl std::ops::DerefMut for SBDebugger {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dbg
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
            let mut dbg = bindings::lldb::SBDebugger::Create();
            dbg.SetAsync(true);
            assert_eq!(true, dbg.GetAsync());
        }
    }
}
