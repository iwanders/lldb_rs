#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_rust_codeblocks)]

use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "lldb_api.h"
    safety!(unsafe) // see details of unsafety policies described in the 'safety' section of the book
    generate!("lldb::SBAddress")
    generate!("lldb::SBBlock")
    generate!("lldb::SBBreakpoint")
    generate!("lldb::SBBreakpointLocation")
    generate!("lldb::SBBreakpointName")
    generate!("lldb::SBBreakpointNameImpl")
    generate!("lldb::SBBroadcaster")
    generate!("lldb::SBCommand")
    generate!("lldb::SBCommandInterpreter")
    generate!("lldb::SBCommandInterpreterRunOptions")
    generate!("lldb::SBCommandInterpreterRunResult")
    generate!("lldb::SBCommandPluginInterface")
    generate!("lldb::SBCommandReturnObject")
    generate!("lldb::SBCommunication")
    generate!("lldb::SBCompileUnit")
    generate!("lldb::SBData")
    generate!("lldb::SBDebugger")
    generate!("lldb::SBDeclaration")
    generate!("lldb::SBEnvironment")
    generate!("lldb::SBError")
    generate!("lldb::SBEvent")
    generate!("lldb::SBEventList")
    generate!("lldb::SBExecutionContext")
    generate!("lldb::SBExpressionOptions")
    generate!("lldb::SBFile")
    generate!("lldb::SBFileSpec")
    generate!("lldb::SBFileSpecList")
    generate!("lldb::SBFrame")
    generate!("lldb::SBFunction")
    generate!("lldb::SBHostOS")
    generate!("lldb::SBInstruction")
    generate!("lldb::SBInstructionList")
    generate!("lldb::SBLanguageRuntime")
    generate!("lldb::SBLaunchInfo")
    generate!("lldb::SBLineEntry")
    generate!("lldb::SBListener")
    generate!("lldb::SBMemoryRegionInfo")
    generate!("lldb::SBMemoryRegionInfoList")
    generate!("lldb::SBModule")
    generate!("lldb::SBModuleSpec")
    generate!("lldb::SBModuleSpecList")
    generate!("lldb::SBProcess")
    generate!("lldb::SBProcessInfo")
    generate!("lldb::SBQueue")
    generate!("lldb::SBQueueItem")
    generate!("lldb::SBSection")
    generate!("lldb::SBSourceManager")
    generate!("lldb::SBStream")
    generate!("lldb::SBStringList")
    generate!("lldb::SBStructuredData")
    generate!("lldb::SBSymbol")
    generate!("lldb::SBSymbolContext")
    generate!("lldb::SBSymbolContextList")
    generate!("lldb::SBTarget")
    generate!("lldb::SBThread")
    generate!("lldb::SBThreadCollection")
    generate!("lldb::SBThreadPlan")
    generate!("lldb::SBTrace")
    generate!("lldb::SBType")
    generate!("lldb::SBTypeCategory")
    generate!("lldb::SBTypeEnumMember")
    generate!("lldb::SBTypeEnumMemberList")
    generate!("lldb::SBTypeFilter")
    // generate!("lldb::SBTypeFormat")
    generate!("lldb::SBTypeMemberFunction")
    generate!("lldb::SBTypeNameSpecifier")
    generate!("lldb::SBTypeSummary")
    generate!("lldb::SBTypeSummaryOptions")
    generate!("lldb::SBTypeSynthetic")
    generate!("lldb::SBTypeList")
    generate!("lldb::SBValue")
    generate!("lldb::SBValueList")
    generate!("lldb::SBVariablesOptions")
    generate!("lldb::SBWatchpoint")
    generate!("lldb::SBUnixSignals")
    // generate_ns!("lldb") // breaks on SBTypeFormat::Type.
    name!(internal_ffi)
}

pub mod ffi {
    pub use super::internal_ffi::*;
}
