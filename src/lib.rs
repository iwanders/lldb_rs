/// The raw bindings as produced.
pub mod api;

/// Some wrappers to make things a lot more convenient.
pub mod wrappers;

/// Re-export autocxx, consumers will likely want to use `autocxx::prelude::*`.
pub use autocxx;

#[cfg(test)]
mod test {
    use super::*;
    use api::ffi::lldb;
    use autocxx::prelude::*;

    #[test]
    fn try_version() {
        unsafe {
            let v = std::ffi::CStr::from_ptr(lldb::SBDebugger::GetVersionString());
            println!("Version: {v:?}");
        }
    }

    #[test]
    fn try_debugger() {
        lldb::SBDebugger::Initialize();
        let mut dbg = lldb::SBDebugger::Create().within_box();
        dbg.as_mut().SetAsync(true);
        assert_eq!(true, dbg.as_mut().GetAsync());
        dbg.as_mut().SetAsync(false);
        assert_eq!(false, dbg.as_mut().GetAsync());
        dbg.as_mut().SetAsync(true);
        assert_eq!(true, dbg.as_mut().GetAsync());

        unsafe {
            let mut z = dbg.as_mut().GetDummyTarget().within_box();
            let triple = std::ffi::CStr::from_ptr(z.as_mut().GetTriple());
            println!("{:?}", triple);
            assert_eq!("x86_64-pc-linux-gnu", triple.to_str().expect("ascii only"));
        }
    }
}
