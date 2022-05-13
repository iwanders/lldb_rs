#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_rust_codeblocks)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn try_version()
    {
        unsafe {

            let v = std::ffi::CStr::from_ptr(root::lldb::SBDebugger::GetVersionString());
            println!("Version: {v:?}");
        }
    }
    #[test]
    fn try_debugger()
    {
        unsafe {
            root::lldb::SBDebugger::Initialize();
            let mut dbg = root::lldb::SBDebugger::Create();
        }
    }
}
