// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::Error::*;
use crate::ExternC::*;
use crate::TargetMachine::*;
use crate::ExternC::*;
use crate::Target::*;
use crate::ExternC::*;
use crate::Types::*;
use crate::DataTypes::*;
use crate::ExternC::*;
use crate::Types::*;
use crate::Types::*;
pub type LLVMOrcJITTargetAddress = std::ffi::c_ulong;
pub type LLVMOrcExecutorAddress = std::ffi::c_ulong;

pub const LLVMJITSymbolGenericFlagsNone: std::ffi::c_uint = 0;
pub const LLVMJITSymbolGenericFlagsExported: std::ffi::c_uint = 1;
pub const LLVMJITSymbolGenericFlagsWeak: std::ffi::c_uint = 2;
pub const LLVMJITSymbolGenericFlagsCallable: std::ffi::c_uint = 4;
pub const LLVMJITSymbolGenericFlagsMaterializationSideEffectsOnly: std::ffi::c_uint = 8;
pub type LLVMJITSymbolGenericFlags = std::ffi::c_uint;
pub type LLVMJITSymbolTargetFlags = std::ffi::c_uchar;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_0 {
  pub GenericFlags: std::ffi::c_uchar,
  pub TargetFlags: std::ffi::c_uchar,
}
pub type LLVMJITSymbolFlags = Struct__anon_0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_1 {
  pub Address: std::ffi::c_ulong,
  pub Flags: LLVMJITSymbolFlags,
}
pub type LLVMJITEvaluatedSymbol = Struct__anon_1;
pub type LLVMOrcExecutionSessionRef = *mut u8;
pub type LLVMOrcErrorReporterFunction = *mut extern fn (*mut std::ffi::c_void, LLVMErrorRef) -> ();
pub type LLVMOrcSymbolStringPoolRef = *mut u8;
pub type LLVMOrcSymbolStringPoolEntryRef = *mut u8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_2 {
  pub Name: LLVMOrcSymbolStringPoolEntryRef,
  pub Flags: LLVMJITSymbolFlags,
}
pub type LLVMOrcCSymbolFlagsMapPair = Struct__anon_2;
pub type LLVMOrcCSymbolFlagsMapPairs = *mut LLVMOrcCSymbolFlagsMapPair;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_3 {
  pub Name: LLVMOrcSymbolStringPoolEntryRef,
  pub Sym: LLVMJITEvaluatedSymbol,
}
pub type LLVMOrcCSymbolMapPair = Struct__anon_3;
pub type LLVMOrcCSymbolMapPairs = *mut LLVMOrcCSymbolMapPair;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_4 {
  pub Name: LLVMOrcSymbolStringPoolEntryRef,
  pub Flags: LLVMJITSymbolFlags,
}
pub type LLVMOrcCSymbolAliasMapEntry = Struct__anon_4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_5 {
  pub Name: LLVMOrcSymbolStringPoolEntryRef,
  pub Entry: LLVMOrcCSymbolAliasMapEntry,
}
pub type LLVMOrcCSymbolAliasMapPair = Struct__anon_5;
pub type LLVMOrcCSymbolAliasMapPairs = *mut LLVMOrcCSymbolAliasMapPair;
pub type LLVMOrcJITDylibRef = *mut u8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_6 {
  pub Symbols: *mut LLVMOrcSymbolStringPoolEntryRef,
  pub Length: std::ffi::c_ulong,
}
pub type LLVMOrcCSymbolsList = Struct__anon_6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_7 {
  pub JD: LLVMOrcJITDylibRef,
  pub Names: LLVMOrcCSymbolsList,
}
pub type LLVMOrcCDependenceMapPair = Struct__anon_7;
pub type LLVMOrcCDependenceMapPairs = *mut LLVMOrcCDependenceMapPair;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_8 {
  pub Symbols: LLVMOrcCSymbolsList,
  pub Dependencies: LLVMOrcCDependenceMapPairs,
  pub NumDependencies: std::ffi::c_ulong,
}
pub type LLVMOrcCSymbolDependenceGroup = Struct__anon_8;

pub const LLVMOrcLookupKindStatic: std::ffi::c_uint = 0;
pub const LLVMOrcLookupKindDLSym: std::ffi::c_uint = 1;
pub type LLVMOrcLookupKind = std::ffi::c_uint;

pub const LLVMOrcJITDylibLookupFlagsMatchExportedSymbolsOnly: std::ffi::c_uint = 0;
pub const LLVMOrcJITDylibLookupFlagsMatchAllSymbols: std::ffi::c_uint = 1;
pub type LLVMOrcJITDylibLookupFlags = std::ffi::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_9 {
  pub JD: LLVMOrcJITDylibRef,
  pub JDLookupFlags: std::ffi::c_uint,
}
pub type LLVMOrcCJITDylibSearchOrderElement = Struct__anon_9;
pub type LLVMOrcCJITDylibSearchOrder = *mut LLVMOrcCJITDylibSearchOrderElement;

pub const LLVMOrcSymbolLookupFlagsRequiredSymbol: std::ffi::c_uint = 0;
pub const LLVMOrcSymbolLookupFlagsWeaklyReferencedSymbol: std::ffi::c_uint = 1;
pub type LLVMOrcSymbolLookupFlags = std::ffi::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_10 {
  pub Name: LLVMOrcSymbolStringPoolEntryRef,
  pub LookupFlags: std::ffi::c_uint,
}
pub type LLVMOrcCLookupSetElement = Struct__anon_10;
pub type LLVMOrcCLookupSet = *mut LLVMOrcCLookupSetElement;
pub type LLVMOrcMaterializationUnitRef = *mut u8;
pub type LLVMOrcMaterializationResponsibilityRef = *mut u8;
pub type LLVMOrcMaterializationUnitMaterializeFunction = *mut extern fn (*mut std::ffi::c_void, LLVMOrcMaterializationResponsibilityRef) -> ();
pub type LLVMOrcMaterializationUnitDiscardFunction = *mut extern fn (*mut std::ffi::c_void, LLVMOrcJITDylibRef, LLVMOrcSymbolStringPoolEntryRef) -> ();
pub type LLVMOrcMaterializationUnitDestroyFunction = *mut extern fn (*mut std::ffi::c_void) -> ();
pub type LLVMOrcResourceTrackerRef = *mut u8;
pub type LLVMOrcDefinitionGeneratorRef = *mut u8;
pub type LLVMOrcLookupStateRef = *mut u8;
pub type LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction = *mut extern fn (LLVMOrcDefinitionGeneratorRef, *mut std::ffi::c_void, *mut LLVMOrcLookupStateRef, std::ffi::c_uint, LLVMOrcJITDylibRef, std::ffi::c_uint, LLVMOrcCLookupSet, std::ffi::c_ulong) -> LLVMErrorRef;
pub type LLVMOrcDisposeCAPIDefinitionGeneratorFunction = *mut extern fn (*mut std::ffi::c_void) -> ();
pub type LLVMOrcSymbolPredicate = *mut extern fn (*mut std::ffi::c_void, LLVMOrcSymbolStringPoolEntryRef) -> std::ffi::c_int;
pub type LLVMOrcThreadSafeContextRef = *mut u8;
pub type LLVMOrcThreadSafeModuleRef = *mut u8;
pub type LLVMOrcGenericIRModuleOperationFunction = *mut extern fn (*mut std::ffi::c_void, LLVMModuleRef) -> LLVMErrorRef;
pub type LLVMOrcJITTargetMachineBuilderRef = *mut u8;
pub type LLVMOrcObjectLayerRef = *mut u8;
pub type LLVMOrcObjectLinkingLayerRef = *mut u8;
pub type LLVMOrcIRTransformLayerRef = *mut u8;
pub type LLVMOrcIRTransformLayerTransformFunction = *mut extern fn (*mut std::ffi::c_void, *mut LLVMOrcThreadSafeModuleRef, LLVMOrcMaterializationResponsibilityRef) -> LLVMErrorRef;
pub type LLVMOrcObjectTransformLayerRef = *mut u8;
pub type LLVMOrcObjectTransformLayerTransformFunction = *mut extern fn (*mut std::ffi::c_void, *mut LLVMMemoryBufferRef) -> LLVMErrorRef;
pub type LLVMOrcIndirectStubsManagerRef = *mut u8;
pub type LLVMOrcLazyCallThroughManagerRef = *mut u8;
pub type LLVMOrcDumpObjectsRef = *mut u8;
pub type LLVMOrcExecutionSessionLookupHandleResultFunction = *mut extern fn (LLVMErrorRef, LLVMOrcCSymbolMapPairs, std::ffi::c_ulong, *mut std::ffi::c_void) -> ();

#[link(name = "LLVM")]
extern {

  pub fn LLVMInitializeX86TargetInfo() -> ();

  pub fn LLVMInitializeNVPTXTargetInfo() -> ();

  pub fn LLVMInitializeAMDGPUTargetInfo() -> ();

  pub fn LLVMInitializeX86Target() -> ();

  pub fn LLVMInitializeNVPTXTarget() -> ();

  pub fn LLVMInitializeAMDGPUTarget() -> ();

  pub fn LLVMInitializeX86TargetMC() -> ();

  pub fn LLVMInitializeNVPTXTargetMC() -> ();

  pub fn LLVMInitializeAMDGPUTargetMC() -> ();

  pub fn LLVMInitializeX86AsmPrinter() -> ();

  pub fn LLVMInitializeNVPTXAsmPrinter() -> ();

  pub fn LLVMInitializeAMDGPUAsmPrinter() -> ();

  pub fn LLVMInitializeX86AsmParser() -> ();

  pub fn LLVMInitializeAMDGPUAsmParser() -> ();

  pub fn LLVMInitializeX86Disassembler() -> ();

  pub fn LLVMInitializeAMDGPUDisassembler() -> ();

  pub fn LLVMOrcExecutionSessionSetErrorReporter(ES: LLVMOrcExecutionSessionRef, ReportError: LLVMOrcErrorReporterFunction, Ctx: *mut std::ffi::c_void) -> ();

  pub fn LLVMOrcExecutionSessionGetSymbolStringPool(ES: LLVMOrcExecutionSessionRef) -> LLVMOrcSymbolStringPoolRef;

  pub fn LLVMOrcSymbolStringPoolClearDeadEntries(SSP: LLVMOrcSymbolStringPoolRef) -> ();

  pub fn LLVMOrcExecutionSessionIntern(ES: LLVMOrcExecutionSessionRef, Name: *const std::ffi::c_char) -> LLVMOrcSymbolStringPoolEntryRef;

  pub fn LLVMOrcExecutionSessionLookup(ES: LLVMOrcExecutionSessionRef, K: std::ffi::c_uint, SearchOrder: LLVMOrcCJITDylibSearchOrder, SearchOrderSize: std::ffi::c_ulong, Symbols: LLVMOrcCLookupSet, SymbolsSize: std::ffi::c_ulong, HandleResult: LLVMOrcExecutionSessionLookupHandleResultFunction, Ctx: *mut std::ffi::c_void) -> ();

  pub fn LLVMOrcRetainSymbolStringPoolEntry(S: LLVMOrcSymbolStringPoolEntryRef) -> ();

  pub fn LLVMOrcReleaseSymbolStringPoolEntry(S: LLVMOrcSymbolStringPoolEntryRef) -> ();

  pub fn LLVMOrcSymbolStringPoolEntryStr(S: LLVMOrcSymbolStringPoolEntryRef) -> *const std::ffi::c_char;

  pub fn LLVMOrcReleaseResourceTracker(RT: LLVMOrcResourceTrackerRef) -> ();

  pub fn LLVMOrcResourceTrackerTransferTo(SrcRT: LLVMOrcResourceTrackerRef, DstRT: LLVMOrcResourceTrackerRef) -> ();

  pub fn LLVMOrcResourceTrackerRemove(RT: LLVMOrcResourceTrackerRef) -> LLVMErrorRef;

  pub fn LLVMOrcDisposeDefinitionGenerator(DG: LLVMOrcDefinitionGeneratorRef) -> ();

  pub fn LLVMOrcDisposeMaterializationUnit(MU: LLVMOrcMaterializationUnitRef) -> ();

  pub fn LLVMOrcCreateCustomMaterializationUnit(Name: *const std::ffi::c_char, Ctx: *mut std::ffi::c_void, Syms: LLVMOrcCSymbolFlagsMapPairs, NumSyms: std::ffi::c_ulong, InitSym: LLVMOrcSymbolStringPoolEntryRef, Materialize: LLVMOrcMaterializationUnitMaterializeFunction, Discard: LLVMOrcMaterializationUnitDiscardFunction, Destroy: LLVMOrcMaterializationUnitDestroyFunction) -> LLVMOrcMaterializationUnitRef;

  pub fn LLVMOrcAbsoluteSymbols(Syms: LLVMOrcCSymbolMapPairs, NumPairs: std::ffi::c_ulong) -> LLVMOrcMaterializationUnitRef;

  pub fn LLVMOrcLazyReexports(LCTM: LLVMOrcLazyCallThroughManagerRef, ISM: LLVMOrcIndirectStubsManagerRef, SourceRef: LLVMOrcJITDylibRef, CallableAliases: LLVMOrcCSymbolAliasMapPairs, NumPairs: std::ffi::c_ulong) -> LLVMOrcMaterializationUnitRef;

  pub fn LLVMOrcDisposeMaterializationResponsibility(MR: LLVMOrcMaterializationResponsibilityRef) -> ();

  pub fn LLVMOrcMaterializationResponsibilityGetTargetDylib(MR: LLVMOrcMaterializationResponsibilityRef) -> LLVMOrcJITDylibRef;

  pub fn LLVMOrcMaterializationResponsibilityGetExecutionSession(MR: LLVMOrcMaterializationResponsibilityRef) -> LLVMOrcExecutionSessionRef;

  pub fn LLVMOrcMaterializationResponsibilityGetSymbols(MR: LLVMOrcMaterializationResponsibilityRef, NumPairs: *mut std::ffi::c_ulong) -> LLVMOrcCSymbolFlagsMapPairs;

  pub fn LLVMOrcDisposeCSymbolFlagsMap(Pairs: LLVMOrcCSymbolFlagsMapPairs) -> ();

  pub fn LLVMOrcMaterializationResponsibilityGetInitializerSymbol(MR: LLVMOrcMaterializationResponsibilityRef) -> LLVMOrcSymbolStringPoolEntryRef;

  pub fn LLVMOrcMaterializationResponsibilityGetRequestedSymbols(MR: LLVMOrcMaterializationResponsibilityRef, NumSymbols: *mut std::ffi::c_ulong) -> *mut LLVMOrcSymbolStringPoolEntryRef;

  pub fn LLVMOrcDisposeSymbols(Symbols: *mut LLVMOrcSymbolStringPoolEntryRef) -> ();

  pub fn LLVMOrcMaterializationResponsibilityNotifyResolved(MR: LLVMOrcMaterializationResponsibilityRef, Symbols: LLVMOrcCSymbolMapPairs, NumPairs: std::ffi::c_ulong) -> LLVMErrorRef;

  pub fn LLVMOrcMaterializationResponsibilityNotifyEmitted(MR: LLVMOrcMaterializationResponsibilityRef, SymbolDepGroups: *mut LLVMOrcCSymbolDependenceGroup, NumSymbolDepGroups: std::ffi::c_ulong) -> LLVMErrorRef;

  pub fn LLVMOrcMaterializationResponsibilityDefineMaterializing(MR: LLVMOrcMaterializationResponsibilityRef, Pairs: LLVMOrcCSymbolFlagsMapPairs, NumPairs: std::ffi::c_ulong) -> LLVMErrorRef;

  pub fn LLVMOrcMaterializationResponsibilityFailMaterialization(MR: LLVMOrcMaterializationResponsibilityRef) -> ();

  pub fn LLVMOrcMaterializationResponsibilityReplace(MR: LLVMOrcMaterializationResponsibilityRef, MU: LLVMOrcMaterializationUnitRef) -> LLVMErrorRef;

  pub fn LLVMOrcMaterializationResponsibilityDelegate(MR: LLVMOrcMaterializationResponsibilityRef, Symbols: *mut LLVMOrcSymbolStringPoolEntryRef, NumSymbols: std::ffi::c_ulong, Result: *mut LLVMOrcMaterializationResponsibilityRef) -> LLVMErrorRef;

  pub fn LLVMOrcExecutionSessionCreateBareJITDylib(ES: LLVMOrcExecutionSessionRef, Name: *const std::ffi::c_char) -> LLVMOrcJITDylibRef;

  pub fn LLVMOrcExecutionSessionCreateJITDylib(ES: LLVMOrcExecutionSessionRef, Result: *mut LLVMOrcJITDylibRef, Name: *const std::ffi::c_char) -> LLVMErrorRef;

  pub fn LLVMOrcExecutionSessionGetJITDylibByName(ES: LLVMOrcExecutionSessionRef, Name: *const std::ffi::c_char) -> LLVMOrcJITDylibRef;

  pub fn LLVMOrcJITDylibCreateResourceTracker(JD: LLVMOrcJITDylibRef) -> LLVMOrcResourceTrackerRef;

  pub fn LLVMOrcJITDylibGetDefaultResourceTracker(JD: LLVMOrcJITDylibRef) -> LLVMOrcResourceTrackerRef;

  pub fn LLVMOrcJITDylibDefine(JD: LLVMOrcJITDylibRef, MU: LLVMOrcMaterializationUnitRef) -> LLVMErrorRef;

  pub fn LLVMOrcJITDylibClear(JD: LLVMOrcJITDylibRef) -> LLVMErrorRef;

  pub fn LLVMOrcJITDylibAddGenerator(JD: LLVMOrcJITDylibRef, DG: LLVMOrcDefinitionGeneratorRef) -> ();

  pub fn LLVMOrcCreateCustomCAPIDefinitionGenerator(F: LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction, Ctx: *mut std::ffi::c_void, Dispose: LLVMOrcDisposeCAPIDefinitionGeneratorFunction) -> LLVMOrcDefinitionGeneratorRef;

  pub fn LLVMOrcLookupStateContinueLookup(S: LLVMOrcLookupStateRef, Err: LLVMErrorRef) -> ();

  pub fn LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess(Result: *mut LLVMOrcDefinitionGeneratorRef, GlobalPrefx: std::ffi::c_char, Filter: LLVMOrcSymbolPredicate, FilterCtx: *mut std::ffi::c_void) -> LLVMErrorRef;

  pub fn LLVMOrcCreateDynamicLibrarySearchGeneratorForPath(Result: *mut LLVMOrcDefinitionGeneratorRef, FileName: *const std::ffi::c_char, GlobalPrefix: std::ffi::c_char, Filter: LLVMOrcSymbolPredicate, FilterCtx: *mut std::ffi::c_void) -> LLVMErrorRef;

  pub fn LLVMOrcCreateStaticLibrarySearchGeneratorForPath(Result: *mut LLVMOrcDefinitionGeneratorRef, ObjLayer: LLVMOrcObjectLayerRef, FileName: *const std::ffi::c_char, TargetTriple: *const std::ffi::c_char) -> LLVMErrorRef;

  pub fn LLVMOrcCreateNewThreadSafeContext() -> LLVMOrcThreadSafeContextRef;

  pub fn LLVMOrcThreadSafeContextGetContext(TSCtx: LLVMOrcThreadSafeContextRef) -> LLVMContextRef;

  pub fn LLVMOrcDisposeThreadSafeContext(TSCtx: LLVMOrcThreadSafeContextRef) -> ();

  pub fn LLVMOrcCreateNewThreadSafeModule(M: LLVMModuleRef, TSCtx: LLVMOrcThreadSafeContextRef) -> LLVMOrcThreadSafeModuleRef;

  pub fn LLVMOrcDisposeThreadSafeModule(TSM: LLVMOrcThreadSafeModuleRef) -> ();

  pub fn LLVMOrcThreadSafeModuleWithModuleDo(TSM: LLVMOrcThreadSafeModuleRef, F: LLVMOrcGenericIRModuleOperationFunction, Ctx: *mut std::ffi::c_void) -> LLVMErrorRef;

  pub fn LLVMOrcJITTargetMachineBuilderDetectHost(Result: *mut LLVMOrcJITTargetMachineBuilderRef) -> LLVMErrorRef;

  pub fn LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine(TM: LLVMTargetMachineRef) -> LLVMOrcJITTargetMachineBuilderRef;

  pub fn LLVMOrcDisposeJITTargetMachineBuilder(JTMB: LLVMOrcJITTargetMachineBuilderRef) -> ();

  pub fn LLVMOrcJITTargetMachineBuilderGetTargetTriple(JTMB: LLVMOrcJITTargetMachineBuilderRef) -> *mut std::ffi::c_char;

  pub fn LLVMOrcJITTargetMachineBuilderSetTargetTriple(JTMB: LLVMOrcJITTargetMachineBuilderRef, TargetTriple: *const std::ffi::c_char) -> ();

  pub fn LLVMOrcObjectLayerAddObjectFile(ObjLayer: LLVMOrcObjectLayerRef, JD: LLVMOrcJITDylibRef, ObjBuffer: LLVMMemoryBufferRef) -> LLVMErrorRef;

  pub fn LLVMOrcObjectLayerAddObjectFileWithRT(ObjLayer: LLVMOrcObjectLayerRef, RT: LLVMOrcResourceTrackerRef, ObjBuffer: LLVMMemoryBufferRef) -> LLVMErrorRef;

  pub fn LLVMOrcObjectLayerEmit(ObjLayer: LLVMOrcObjectLayerRef, R: LLVMOrcMaterializationResponsibilityRef, ObjBuffer: LLVMMemoryBufferRef) -> ();

  pub fn LLVMOrcDisposeObjectLayer(ObjLayer: LLVMOrcObjectLayerRef) -> ();

  pub fn LLVMOrcIRTransformLayerEmit(IRTransformLayer: LLVMOrcIRTransformLayerRef, MR: LLVMOrcMaterializationResponsibilityRef, TSM: LLVMOrcThreadSafeModuleRef) -> ();

  pub fn LLVMOrcIRTransformLayerSetTransform(IRTransformLayer: LLVMOrcIRTransformLayerRef, TransformFunction: LLVMOrcIRTransformLayerTransformFunction, Ctx: *mut std::ffi::c_void) -> ();

  pub fn LLVMOrcObjectTransformLayerSetTransform(ObjTransformLayer: LLVMOrcObjectTransformLayerRef, TransformFunction: LLVMOrcObjectTransformLayerTransformFunction, Ctx: *mut std::ffi::c_void) -> ();

  pub fn LLVMOrcCreateLocalIndirectStubsManager(TargetTriple: *const std::ffi::c_char) -> LLVMOrcIndirectStubsManagerRef;

  pub fn LLVMOrcDisposeIndirectStubsManager(ISM: LLVMOrcIndirectStubsManagerRef) -> ();

  pub fn LLVMOrcCreateLocalLazyCallThroughManager(TargetTriple: *const std::ffi::c_char, ES: LLVMOrcExecutionSessionRef, ErrorHandlerAddr: std::ffi::c_ulong, LCTM: *mut LLVMOrcLazyCallThroughManagerRef) -> LLVMErrorRef;

  pub fn LLVMOrcDisposeLazyCallThroughManager(LCTM: LLVMOrcLazyCallThroughManagerRef) -> ();

  pub fn LLVMOrcCreateDumpObjects(DumpDir: *const std::ffi::c_char, IdentifierOverride: *const std::ffi::c_char) -> LLVMOrcDumpObjectsRef;

  pub fn LLVMOrcDisposeDumpObjects(DumpObjects: LLVMOrcDumpObjectsRef) -> ();

  pub fn LLVMOrcDumpObjects_CallOperator(DumpObjects: LLVMOrcDumpObjectsRef, ObjBuffer: *mut LLVMMemoryBufferRef) -> LLVMErrorRef;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl FFIVoid_ {
  pub unsafe fn LLVMOrcExecutionSessionSetErrorReporter<T0_, T1_, T2_>(ES_:  T0_, ReportError_:  T1_, Ctx_:  T2_)
  where
     T0_: Into<LLVMOrcExecutionSessionRef>,  T1_: Into<LLVMOrcErrorReporterFunction>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::Orc::LLVMOrcExecutionSessionSetErrorReporter(Into::<LLVMOrcExecutionSessionRef>::into(ES_), Into::<LLVMOrcErrorReporterFunction>::into(ReportError_), Into::<*mut std::ffi::c_void>::into(Ctx_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcSymbolStringPoolRef> {
  pub unsafe fn LLVMOrcExecutionSessionGetSymbolStringPool<T0_>(ES_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcExecutionSessionRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcExecutionSessionGetSymbolStringPool(Into::<LLVMOrcExecutionSessionRef>::into(ES_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcSymbolStringPoolClearDeadEntries<T0_>(SSP_:  T0_)
  where
     T0_: Into<LLVMOrcSymbolStringPoolRef>
  {
    unsafe {
      crate::Orc::LLVMOrcSymbolStringPoolClearDeadEntries(Into::<LLVMOrcSymbolStringPoolRef>::into(SSP_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcSymbolStringPoolEntryRef> {
  pub unsafe fn LLVMOrcExecutionSessionIntern<T0_, T1_>(ES_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOrcExecutionSessionRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcExecutionSessionIntern(Into::<LLVMOrcExecutionSessionRef>::into(ES_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcExecutionSessionLookup<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_>(ES_:  T0_, K_:  T1_, SearchOrder_:  T2_, SearchOrderSize_:  T3_, Symbols_:  T4_, SymbolsSize_:  T5_, HandleResult_:  T6_, Ctx_:  T7_)
  where
     T0_: Into<LLVMOrcExecutionSessionRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMOrcCJITDylibSearchOrder>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMOrcCLookupSet>,  T5_: Into<std::ffi::c_ulong>,  T6_: Into<LLVMOrcExecutionSessionLookupHandleResultFunction>,  T7_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::Orc::LLVMOrcExecutionSessionLookup(Into::<LLVMOrcExecutionSessionRef>::into(ES_), Into::<std::ffi::c_uint>::into(K_), Into::<LLVMOrcCJITDylibSearchOrder>::into(SearchOrder_), Into::<std::ffi::c_ulong>::into(SearchOrderSize_), Into::<LLVMOrcCLookupSet>::into(Symbols_), Into::<std::ffi::c_ulong>::into(SymbolsSize_), Into::<LLVMOrcExecutionSessionLookupHandleResultFunction>::into(HandleResult_), Into::<*mut std::ffi::c_void>::into(Ctx_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcRetainSymbolStringPoolEntry<T0_>(S_:  T0_)
  where
     T0_: Into<LLVMOrcSymbolStringPoolEntryRef>
  {
    unsafe {
      crate::Orc::LLVMOrcRetainSymbolStringPoolEntry(Into::<LLVMOrcSymbolStringPoolEntryRef>::into(S_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcReleaseSymbolStringPoolEntry<T0_>(S_:  T0_)
  where
     T0_: Into<LLVMOrcSymbolStringPoolEntryRef>
  {
    unsafe {
      crate::Orc::LLVMOrcReleaseSymbolStringPoolEntry(Into::<LLVMOrcSymbolStringPoolEntryRef>::into(S_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMOrcSymbolStringPoolEntryStr<T0_>(S_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcSymbolStringPoolEntryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcSymbolStringPoolEntryStr(Into::<LLVMOrcSymbolStringPoolEntryRef>::into(S_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcReleaseResourceTracker<T0_>(RT_:  T0_)
  where
     T0_: Into<LLVMOrcResourceTrackerRef>
  {
    unsafe {
      crate::Orc::LLVMOrcReleaseResourceTracker(Into::<LLVMOrcResourceTrackerRef>::into(RT_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcResourceTrackerTransferTo<T0_, T1_>(SrcRT_:  T0_, DstRT_:  T1_)
  where
     T0_: Into<LLVMOrcResourceTrackerRef>,  T1_: Into<LLVMOrcResourceTrackerRef>
  {
    unsafe {
      crate::Orc::LLVMOrcResourceTrackerTransferTo(Into::<LLVMOrcResourceTrackerRef>::into(SrcRT_), Into::<LLVMOrcResourceTrackerRef>::into(DstRT_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcResourceTrackerRemove<T0_>(RT_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcResourceTrackerRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcResourceTrackerRemove(Into::<LLVMOrcResourceTrackerRef>::into(RT_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeDefinitionGenerator<T0_>(DG_:  T0_)
  where
     T0_: Into<LLVMOrcDefinitionGeneratorRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeDefinitionGenerator(Into::<LLVMOrcDefinitionGeneratorRef>::into(DG_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeMaterializationUnit<T0_>(MU_:  T0_)
  where
     T0_: Into<LLVMOrcMaterializationUnitRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeMaterializationUnit(Into::<LLVMOrcMaterializationUnitRef>::into(MU_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcMaterializationUnitRef> {
  pub unsafe fn LLVMOrcCreateCustomMaterializationUnit<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_>(Name_:  T0_, Ctx_:  T1_, Syms_:  T2_, NumSyms_:  T3_, InitSym_:  T4_, Materialize_:  T5_, Discard_:  T6_, Destroy_:  T7_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<*mut std::ffi::c_void>,  T2_: Into<LLVMOrcCSymbolFlagsMapPairs>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMOrcSymbolStringPoolEntryRef>,  T5_: Into<LLVMOrcMaterializationUnitMaterializeFunction>,  T6_: Into<LLVMOrcMaterializationUnitDiscardFunction>,  T7_: Into<LLVMOrcMaterializationUnitDestroyFunction>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcCreateCustomMaterializationUnit(Into::<*const std::ffi::c_char>::into(Name_), Into::<*mut std::ffi::c_void>::into(Ctx_), Into::<LLVMOrcCSymbolFlagsMapPairs>::into(Syms_), Into::<std::ffi::c_ulong>::into(NumSyms_), Into::<LLVMOrcSymbolStringPoolEntryRef>::into(InitSym_), Into::<LLVMOrcMaterializationUnitMaterializeFunction>::into(Materialize_), Into::<LLVMOrcMaterializationUnitDiscardFunction>::into(Discard_), Into::<LLVMOrcMaterializationUnitDestroyFunction>::into(Destroy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcMaterializationUnitRef> {
  pub unsafe fn LLVMOrcAbsoluteSymbols<T0_, T1_>(Syms_:  T0_, NumPairs_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOrcCSymbolMapPairs>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcAbsoluteSymbols(Into::<LLVMOrcCSymbolMapPairs>::into(Syms_), Into::<std::ffi::c_ulong>::into(NumPairs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcMaterializationUnitRef> {
  pub unsafe fn LLVMOrcLazyReexports<T0_, T1_, T2_, T3_, T4_>(LCTM_:  T0_, ISM_:  T1_, SourceRef_:  T2_, CallableAliases_:  T3_, NumPairs_:  T4_)-> Tret_
  where
     T0_: Into<LLVMOrcLazyCallThroughManagerRef>,  T1_: Into<LLVMOrcIndirectStubsManagerRef>,  T2_: Into<LLVMOrcJITDylibRef>,  T3_: Into<LLVMOrcCSymbolAliasMapPairs>,  T4_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcLazyReexports(Into::<LLVMOrcLazyCallThroughManagerRef>::into(LCTM_), Into::<LLVMOrcIndirectStubsManagerRef>::into(ISM_), Into::<LLVMOrcJITDylibRef>::into(SourceRef_), Into::<LLVMOrcCSymbolAliasMapPairs>::into(CallableAliases_), Into::<std::ffi::c_ulong>::into(NumPairs_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeMaterializationResponsibility<T0_>(MR_:  T0_)
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeMaterializationResponsibility(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcJITDylibRef> {
  pub unsafe fn LLVMOrcMaterializationResponsibilityGetTargetDylib<T0_>(MR_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcMaterializationResponsibilityGetTargetDylib(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcExecutionSessionRef> {
  pub unsafe fn LLVMOrcMaterializationResponsibilityGetExecutionSession<T0_>(MR_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcMaterializationResponsibilityGetExecutionSession(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcCSymbolFlagsMapPairs> {
  pub unsafe fn LLVMOrcMaterializationResponsibilityGetSymbols<T0_, T1_>(MR_:  T0_, NumPairs_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcMaterializationResponsibilityGetSymbols(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_), Into::<*mut std::ffi::c_ulong>::into(NumPairs_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeCSymbolFlagsMap<T0_>(Pairs_:  T0_)
  where
     T0_: Into<LLVMOrcCSymbolFlagsMapPairs>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeCSymbolFlagsMap(Into::<LLVMOrcCSymbolFlagsMapPairs>::into(Pairs_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcSymbolStringPoolEntryRef> {
  pub unsafe fn LLVMOrcMaterializationResponsibilityGetInitializerSymbol<T0_>(MR_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcMaterializationResponsibilityGetInitializerSymbol(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut LLVMOrcSymbolStringPoolEntryRef> {
  pub unsafe fn LLVMOrcMaterializationResponsibilityGetRequestedSymbols<T0_, T1_>(MR_:  T0_, NumSymbols_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcMaterializationResponsibilityGetRequestedSymbols(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_), Into::<*mut std::ffi::c_ulong>::into(NumSymbols_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeSymbols<T0_>(Symbols_:  T0_)
  where
     T0_: Into<*mut LLVMOrcSymbolStringPoolEntryRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeSymbols(Into::<*mut LLVMOrcSymbolStringPoolEntryRef>::into(Symbols_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcMaterializationResponsibilityNotifyResolved<T0_, T1_, T2_>(MR_:  T0_, Symbols_:  T1_, NumPairs_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>,  T1_: Into<LLVMOrcCSymbolMapPairs>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcMaterializationResponsibilityNotifyResolved(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_), Into::<LLVMOrcCSymbolMapPairs>::into(Symbols_), Into::<std::ffi::c_ulong>::into(NumPairs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcMaterializationResponsibilityNotifyEmitted<T0_, T1_, T2_>(MR_:  T0_, SymbolDepGroups_:  T1_, NumSymbolDepGroups_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>,  T1_: Into<*mut LLVMOrcCSymbolDependenceGroup>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcMaterializationResponsibilityNotifyEmitted(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_), Into::<*mut LLVMOrcCSymbolDependenceGroup>::into(SymbolDepGroups_), Into::<std::ffi::c_ulong>::into(NumSymbolDepGroups_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcMaterializationResponsibilityDefineMaterializing<T0_, T1_, T2_>(MR_:  T0_, Pairs_:  T1_, NumPairs_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>,  T1_: Into<LLVMOrcCSymbolFlagsMapPairs>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcMaterializationResponsibilityDefineMaterializing(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_), Into::<LLVMOrcCSymbolFlagsMapPairs>::into(Pairs_), Into::<std::ffi::c_ulong>::into(NumPairs_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcMaterializationResponsibilityFailMaterialization<T0_>(MR_:  T0_)
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>
  {
    unsafe {
      crate::Orc::LLVMOrcMaterializationResponsibilityFailMaterialization(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcMaterializationResponsibilityReplace<T0_, T1_>(MR_:  T0_, MU_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>,  T1_: Into<LLVMOrcMaterializationUnitRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcMaterializationResponsibilityReplace(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_), Into::<LLVMOrcMaterializationUnitRef>::into(MU_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcMaterializationResponsibilityDelegate<T0_, T1_, T2_, T3_>(MR_:  T0_, Symbols_:  T1_, NumSymbols_:  T2_, Result_:  T3_)-> Tret_
  where
     T0_: Into<LLVMOrcMaterializationResponsibilityRef>,  T1_: Into<*mut LLVMOrcSymbolStringPoolEntryRef>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<*mut LLVMOrcMaterializationResponsibilityRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcMaterializationResponsibilityDelegate(Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_), Into::<*mut LLVMOrcSymbolStringPoolEntryRef>::into(Symbols_), Into::<std::ffi::c_ulong>::into(NumSymbols_), Into::<*mut LLVMOrcMaterializationResponsibilityRef>::into(Result_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcJITDylibRef> {
  pub unsafe fn LLVMOrcExecutionSessionCreateBareJITDylib<T0_, T1_>(ES_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOrcExecutionSessionRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcExecutionSessionCreateBareJITDylib(Into::<LLVMOrcExecutionSessionRef>::into(ES_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcExecutionSessionCreateJITDylib<T0_, T1_, T2_>(ES_:  T0_, Result_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcExecutionSessionRef>,  T1_: Into<*mut LLVMOrcJITDylibRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcExecutionSessionCreateJITDylib(Into::<LLVMOrcExecutionSessionRef>::into(ES_), Into::<*mut LLVMOrcJITDylibRef>::into(Result_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcJITDylibRef> {
  pub unsafe fn LLVMOrcExecutionSessionGetJITDylibByName<T0_, T1_>(ES_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOrcExecutionSessionRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcExecutionSessionGetJITDylibByName(Into::<LLVMOrcExecutionSessionRef>::into(ES_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcResourceTrackerRef> {
  pub unsafe fn LLVMOrcJITDylibCreateResourceTracker<T0_>(JD_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcJITDylibRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcJITDylibCreateResourceTracker(Into::<LLVMOrcJITDylibRef>::into(JD_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcResourceTrackerRef> {
  pub unsafe fn LLVMOrcJITDylibGetDefaultResourceTracker<T0_>(JD_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcJITDylibRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcJITDylibGetDefaultResourceTracker(Into::<LLVMOrcJITDylibRef>::into(JD_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcJITDylibDefine<T0_, T1_>(JD_:  T0_, MU_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOrcJITDylibRef>,  T1_: Into<LLVMOrcMaterializationUnitRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcJITDylibDefine(Into::<LLVMOrcJITDylibRef>::into(JD_), Into::<LLVMOrcMaterializationUnitRef>::into(MU_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcJITDylibClear<T0_>(JD_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcJITDylibRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcJITDylibClear(Into::<LLVMOrcJITDylibRef>::into(JD_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcJITDylibAddGenerator<T0_, T1_>(JD_:  T0_, DG_:  T1_)
  where
     T0_: Into<LLVMOrcJITDylibRef>,  T1_: Into<LLVMOrcDefinitionGeneratorRef>
  {
    unsafe {
      crate::Orc::LLVMOrcJITDylibAddGenerator(Into::<LLVMOrcJITDylibRef>::into(JD_), Into::<LLVMOrcDefinitionGeneratorRef>::into(DG_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcDefinitionGeneratorRef> {
  pub unsafe fn LLVMOrcCreateCustomCAPIDefinitionGenerator<T0_, T1_, T2_>(F_:  T0_, Ctx_:  T1_, Dispose_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction>,  T1_: Into<*mut std::ffi::c_void>,  T2_: Into<LLVMOrcDisposeCAPIDefinitionGeneratorFunction>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcCreateCustomCAPIDefinitionGenerator(Into::<LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction>::into(F_), Into::<*mut std::ffi::c_void>::into(Ctx_), Into::<LLVMOrcDisposeCAPIDefinitionGeneratorFunction>::into(Dispose_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcLookupStateContinueLookup<T0_, T1_>(S_:  T0_, Err_:  T1_)
  where
     T0_: Into<LLVMOrcLookupStateRef>,  T1_: Into<LLVMErrorRef>
  {
    unsafe {
      crate::Orc::LLVMOrcLookupStateContinueLookup(Into::<LLVMOrcLookupStateRef>::into(S_), Into::<LLVMErrorRef>::into(Err_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess<T0_, T1_, T2_, T3_>(Result_:  T0_, GlobalPrefx_:  T1_, Filter_:  T2_, FilterCtx_:  T3_)-> Tret_
  where
     T0_: Into<*mut LLVMOrcDefinitionGeneratorRef>,  T1_: Into<std::ffi::c_char>,  T2_: Into<LLVMOrcSymbolPredicate>,  T3_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess(Into::<*mut LLVMOrcDefinitionGeneratorRef>::into(Result_), Into::<std::ffi::c_char>::into(GlobalPrefx_), Into::<LLVMOrcSymbolPredicate>::into(Filter_), Into::<*mut std::ffi::c_void>::into(FilterCtx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcCreateDynamicLibrarySearchGeneratorForPath<T0_, T1_, T2_, T3_, T4_>(Result_:  T0_, FileName_:  T1_, GlobalPrefix_:  T2_, Filter_:  T3_, FilterCtx_:  T4_)-> Tret_
  where
     T0_: Into<*mut LLVMOrcDefinitionGeneratorRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_char>,  T3_: Into<LLVMOrcSymbolPredicate>,  T4_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcCreateDynamicLibrarySearchGeneratorForPath(Into::<*mut LLVMOrcDefinitionGeneratorRef>::into(Result_), Into::<*const std::ffi::c_char>::into(FileName_), Into::<std::ffi::c_char>::into(GlobalPrefix_), Into::<LLVMOrcSymbolPredicate>::into(Filter_), Into::<*mut std::ffi::c_void>::into(FilterCtx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcCreateStaticLibrarySearchGeneratorForPath<T0_, T1_, T2_, T3_>(Result_:  T0_, ObjLayer_:  T1_, FileName_:  T2_, TargetTriple_:  T3_)-> Tret_
  where
     T0_: Into<*mut LLVMOrcDefinitionGeneratorRef>,  T1_: Into<LLVMOrcObjectLayerRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcCreateStaticLibrarySearchGeneratorForPath(Into::<*mut LLVMOrcDefinitionGeneratorRef>::into(Result_), Into::<LLVMOrcObjectLayerRef>::into(ObjLayer_), Into::<*const std::ffi::c_char>::into(FileName_), Into::<*const std::ffi::c_char>::into(TargetTriple_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcThreadSafeContextRef> {
  pub unsafe fn LLVMOrcCreateNewThreadSafeContext()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcCreateNewThreadSafeContext()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMContextRef> {
  pub unsafe fn LLVMOrcThreadSafeContextGetContext<T0_>(TSCtx_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcThreadSafeContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcThreadSafeContextGetContext(Into::<LLVMOrcThreadSafeContextRef>::into(TSCtx_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeThreadSafeContext<T0_>(TSCtx_:  T0_)
  where
     T0_: Into<LLVMOrcThreadSafeContextRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeThreadSafeContext(Into::<LLVMOrcThreadSafeContextRef>::into(TSCtx_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcThreadSafeModuleRef> {
  pub unsafe fn LLVMOrcCreateNewThreadSafeModule<T0_, T1_>(M_:  T0_, TSCtx_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<LLVMOrcThreadSafeContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcCreateNewThreadSafeModule(Into::<LLVMModuleRef>::into(M_), Into::<LLVMOrcThreadSafeContextRef>::into(TSCtx_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeThreadSafeModule<T0_>(TSM_:  T0_)
  where
     T0_: Into<LLVMOrcThreadSafeModuleRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeThreadSafeModule(Into::<LLVMOrcThreadSafeModuleRef>::into(TSM_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcThreadSafeModuleWithModuleDo<T0_, T1_, T2_>(TSM_:  T0_, F_:  T1_, Ctx_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcThreadSafeModuleRef>,  T1_: Into<LLVMOrcGenericIRModuleOperationFunction>,  T2_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcThreadSafeModuleWithModuleDo(Into::<LLVMOrcThreadSafeModuleRef>::into(TSM_), Into::<LLVMOrcGenericIRModuleOperationFunction>::into(F_), Into::<*mut std::ffi::c_void>::into(Ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcJITTargetMachineBuilderDetectHost<T0_>(Result_:  T0_)-> Tret_
  where
     T0_: Into<*mut LLVMOrcJITTargetMachineBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcJITTargetMachineBuilderDetectHost(Into::<*mut LLVMOrcJITTargetMachineBuilderRef>::into(Result_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcJITTargetMachineBuilderRef> {
  pub unsafe fn LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine<T0_>(TM_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetMachineRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine(Into::<LLVMTargetMachineRef>::into(TM_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeJITTargetMachineBuilder<T0_>(JTMB_:  T0_)
  where
     T0_: Into<LLVMOrcJITTargetMachineBuilderRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeJITTargetMachineBuilder(Into::<LLVMOrcJITTargetMachineBuilderRef>::into(JTMB_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMOrcJITTargetMachineBuilderGetTargetTriple<T0_>(JTMB_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcJITTargetMachineBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcJITTargetMachineBuilderGetTargetTriple(Into::<LLVMOrcJITTargetMachineBuilderRef>::into(JTMB_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcJITTargetMachineBuilderSetTargetTriple<T0_, T1_>(JTMB_:  T0_, TargetTriple_:  T1_)
  where
     T0_: Into<LLVMOrcJITTargetMachineBuilderRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Orc::LLVMOrcJITTargetMachineBuilderSetTargetTriple(Into::<LLVMOrcJITTargetMachineBuilderRef>::into(JTMB_), Into::<*const std::ffi::c_char>::into(TargetTriple_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcObjectLayerAddObjectFile<T0_, T1_, T2_>(ObjLayer_:  T0_, JD_:  T1_, ObjBuffer_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcObjectLayerRef>,  T1_: Into<LLVMOrcJITDylibRef>,  T2_: Into<LLVMMemoryBufferRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcObjectLayerAddObjectFile(Into::<LLVMOrcObjectLayerRef>::into(ObjLayer_), Into::<LLVMOrcJITDylibRef>::into(JD_), Into::<LLVMMemoryBufferRef>::into(ObjBuffer_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcObjectLayerAddObjectFileWithRT<T0_, T1_, T2_>(ObjLayer_:  T0_, RT_:  T1_, ObjBuffer_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcObjectLayerRef>,  T1_: Into<LLVMOrcResourceTrackerRef>,  T2_: Into<LLVMMemoryBufferRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcObjectLayerAddObjectFileWithRT(Into::<LLVMOrcObjectLayerRef>::into(ObjLayer_), Into::<LLVMOrcResourceTrackerRef>::into(RT_), Into::<LLVMMemoryBufferRef>::into(ObjBuffer_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcObjectLayerEmit<T0_, T1_, T2_>(ObjLayer_:  T0_, R_:  T1_, ObjBuffer_:  T2_)
  where
     T0_: Into<LLVMOrcObjectLayerRef>,  T1_: Into<LLVMOrcMaterializationResponsibilityRef>,  T2_: Into<LLVMMemoryBufferRef>
  {
    unsafe {
      crate::Orc::LLVMOrcObjectLayerEmit(Into::<LLVMOrcObjectLayerRef>::into(ObjLayer_), Into::<LLVMOrcMaterializationResponsibilityRef>::into(R_), Into::<LLVMMemoryBufferRef>::into(ObjBuffer_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeObjectLayer<T0_>(ObjLayer_:  T0_)
  where
     T0_: Into<LLVMOrcObjectLayerRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeObjectLayer(Into::<LLVMOrcObjectLayerRef>::into(ObjLayer_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcIRTransformLayerEmit<T0_, T1_, T2_>(IRTransformLayer_:  T0_, MR_:  T1_, TSM_:  T2_)
  where
     T0_: Into<LLVMOrcIRTransformLayerRef>,  T1_: Into<LLVMOrcMaterializationResponsibilityRef>,  T2_: Into<LLVMOrcThreadSafeModuleRef>
  {
    unsafe {
      crate::Orc::LLVMOrcIRTransformLayerEmit(Into::<LLVMOrcIRTransformLayerRef>::into(IRTransformLayer_), Into::<LLVMOrcMaterializationResponsibilityRef>::into(MR_), Into::<LLVMOrcThreadSafeModuleRef>::into(TSM_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcIRTransformLayerSetTransform<T0_, T1_, T2_>(IRTransformLayer_:  T0_, TransformFunction_:  T1_, Ctx_:  T2_)
  where
     T0_: Into<LLVMOrcIRTransformLayerRef>,  T1_: Into<LLVMOrcIRTransformLayerTransformFunction>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::Orc::LLVMOrcIRTransformLayerSetTransform(Into::<LLVMOrcIRTransformLayerRef>::into(IRTransformLayer_), Into::<LLVMOrcIRTransformLayerTransformFunction>::into(TransformFunction_), Into::<*mut std::ffi::c_void>::into(Ctx_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcObjectTransformLayerSetTransform<T0_, T1_, T2_>(ObjTransformLayer_:  T0_, TransformFunction_:  T1_, Ctx_:  T2_)
  where
     T0_: Into<LLVMOrcObjectTransformLayerRef>,  T1_: Into<LLVMOrcObjectTransformLayerTransformFunction>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::Orc::LLVMOrcObjectTransformLayerSetTransform(Into::<LLVMOrcObjectTransformLayerRef>::into(ObjTransformLayer_), Into::<LLVMOrcObjectTransformLayerTransformFunction>::into(TransformFunction_), Into::<*mut std::ffi::c_void>::into(Ctx_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcIndirectStubsManagerRef> {
  pub unsafe fn LLVMOrcCreateLocalIndirectStubsManager<T0_>(TargetTriple_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcCreateLocalIndirectStubsManager(Into::<*const std::ffi::c_char>::into(TargetTriple_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeIndirectStubsManager<T0_>(ISM_:  T0_)
  where
     T0_: Into<LLVMOrcIndirectStubsManagerRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeIndirectStubsManager(Into::<LLVMOrcIndirectStubsManagerRef>::into(ISM_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcCreateLocalLazyCallThroughManager<T0_, T1_, T2_, T3_>(TargetTriple_:  T0_, ES_:  T1_, ErrorHandlerAddr_:  T2_, LCTM_:  T3_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<LLVMOrcExecutionSessionRef>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<*mut LLVMOrcLazyCallThroughManagerRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcCreateLocalLazyCallThroughManager(Into::<*const std::ffi::c_char>::into(TargetTriple_), Into::<LLVMOrcExecutionSessionRef>::into(ES_), Into::<std::ffi::c_ulong>::into(ErrorHandlerAddr_), Into::<*mut LLVMOrcLazyCallThroughManagerRef>::into(LCTM_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeLazyCallThroughManager<T0_>(LCTM_:  T0_)
  where
     T0_: Into<LLVMOrcLazyCallThroughManagerRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeLazyCallThroughManager(Into::<LLVMOrcLazyCallThroughManagerRef>::into(LCTM_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcDumpObjectsRef> {
  pub unsafe fn LLVMOrcCreateDumpObjects<T0_, T1_>(DumpDir_:  T0_, IdentifierOverride_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcCreateDumpObjects(Into::<*const std::ffi::c_char>::into(DumpDir_), Into::<*const std::ffi::c_char>::into(IdentifierOverride_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeDumpObjects<T0_>(DumpObjects_:  T0_)
  where
     T0_: Into<LLVMOrcDumpObjectsRef>
  {
    unsafe {
      crate::Orc::LLVMOrcDisposeDumpObjects(Into::<LLVMOrcDumpObjectsRef>::into(DumpObjects_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcDumpObjects_CallOperator<T0_, T1_>(DumpObjects_:  T0_, ObjBuffer_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOrcDumpObjectsRef>,  T1_: Into<*mut LLVMMemoryBufferRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Orc::LLVMOrcDumpObjects_CallOperator(Into::<LLVMOrcDumpObjectsRef>::into(DumpObjects_), Into::<*mut LLVMMemoryBufferRef>::into(ObjBuffer_))
      }
    )
  }
}

