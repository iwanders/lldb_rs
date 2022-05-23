# lldb_rs

Rust crate that uses autocxx to generate bindings for lldb-13, some convenience wrappers are
available, but it does not provide full API coverage.

An example on how to use this can be found in the examples directory, this is a simplified version
of what I used to track allocations and more in a 32 bits binary running in Wine.
