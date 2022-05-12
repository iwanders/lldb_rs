#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn try_version()
    {
        unsafe {

            let v = std::ffi::CStr::from_ptr(lldb_SBDebugger::GetVersionString());
            println!("Version: {v:?}");
        }
    }
    #[test]
    fn try_debugger()
    {
        unsafe {
            lldb_SBDebugger::Initialize();
            let mut dbg = lldb_SBDebugger::Create();
        }
    }
}