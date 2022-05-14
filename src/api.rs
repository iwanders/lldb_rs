use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "lldb_api.h"
    safety!(unsafe) // see details of unsafety policies described in the 'safety' section of the book
    generate!("lldb::SBDebugger")
    name!(internal_ffi)
}

pub mod ffi
{
    pub use super::internal_ffi::*;
}
