#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_rust_codeblocks)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)] // something returns an u128, ignore it.

use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "lldb_api.h"
    safety!(unsafe) // see details of unsafety policies described in the 'safety' section of the book
    generate!("lldb::SBDebugger")
    name!(internal_ffi)
    instantiable!("lldb::SBDebugger")
}

pub mod ffi
{
    pub use super::internal_ffi::*;
}
