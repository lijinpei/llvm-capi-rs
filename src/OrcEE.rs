// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::Error::*;
use crate::ExternC::*;
use crate::ExecutionEngine::*;
use crate::ExternC::*;
use crate::Target::*;
use crate::ExternC::*;
use crate::Types::*;
use crate::DataTypes::*;
use crate::ExternC::*;
use crate::TargetMachine::*;
use crate::ExternC::*;
use crate::Target::*;
use crate::Types::*;
use crate::Types::*;
use crate::Orc::*;
use crate::Error::*;
use crate::TargetMachine::*;
use crate::Types::*;
use crate::TargetMachine::*;
use crate::Types::*;
pub type LLVMMemoryManagerCreateContextCallback = *mut extern fn (*mut std::ffi::c_void) -> *mut std::ffi::c_void;
pub type LLVMMemoryManagerNotifyTerminatingCallback = *mut extern fn (*mut std::ffi::c_void) -> ();

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

  pub fn LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager(ES: LLVMOrcExecutionSessionRef) -> LLVMOrcObjectLayerRef;

  pub fn LLVMOrcCreateRTDyldObjectLinkingLayerWithMCJITMemoryManagerLikeCallbacks(ES: LLVMOrcExecutionSessionRef, CreateContextCtx: *mut std::ffi::c_void, CreateContext: LLVMMemoryManagerCreateContextCallback, NotifyTerminating: LLVMMemoryManagerNotifyTerminatingCallback, AllocateCodeSection: LLVMMemoryManagerAllocateCodeSectionCallback, AllocateDataSection: LLVMMemoryManagerAllocateDataSectionCallback, FinalizeMemory: LLVMMemoryManagerFinalizeMemoryCallback, Destroy: LLVMMemoryManagerDestroyCallback) -> LLVMOrcObjectLayerRef;

  pub fn LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener(RTDyldObjLinkingLayer: LLVMOrcObjectLayerRef, Listener: LLVMJITEventListenerRef) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcObjectLayerRef> {
  pub unsafe fn LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager<T0_>(ES_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcExecutionSessionRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::OrcEE::LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager(Into::<LLVMOrcExecutionSessionRef>::into(ES_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcObjectLayerRef> {
  pub unsafe fn LLVMOrcCreateRTDyldObjectLinkingLayerWithMCJITMemoryManagerLikeCallbacks<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_>(ES_:  T0_, CreateContextCtx_:  T1_, CreateContext_:  T2_, NotifyTerminating_:  T3_, AllocateCodeSection_:  T4_, AllocateDataSection_:  T5_, FinalizeMemory_:  T6_, Destroy_:  T7_)-> Tret_
  where
     T0_: Into<LLVMOrcExecutionSessionRef>,  T1_: Into<*mut std::ffi::c_void>,  T2_: Into<LLVMMemoryManagerCreateContextCallback>,  T3_: Into<LLVMMemoryManagerNotifyTerminatingCallback>,  T4_: Into<LLVMMemoryManagerAllocateCodeSectionCallback>,  T5_: Into<LLVMMemoryManagerAllocateDataSectionCallback>,  T6_: Into<LLVMMemoryManagerFinalizeMemoryCallback>,  T7_: Into<LLVMMemoryManagerDestroyCallback>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::OrcEE::LLVMOrcCreateRTDyldObjectLinkingLayerWithMCJITMemoryManagerLikeCallbacks(Into::<LLVMOrcExecutionSessionRef>::into(ES_), Into::<*mut std::ffi::c_void>::into(CreateContextCtx_), Into::<LLVMMemoryManagerCreateContextCallback>::into(CreateContext_), Into::<LLVMMemoryManagerNotifyTerminatingCallback>::into(NotifyTerminating_), Into::<LLVMMemoryManagerAllocateCodeSectionCallback>::into(AllocateCodeSection_), Into::<LLVMMemoryManagerAllocateDataSectionCallback>::into(AllocateDataSection_), Into::<LLVMMemoryManagerFinalizeMemoryCallback>::into(FinalizeMemory_), Into::<LLVMMemoryManagerDestroyCallback>::into(Destroy_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener<T0_, T1_>(RTDyldObjLinkingLayer_:  T0_, Listener_:  T1_)
  where
     T0_: Into<LLVMOrcObjectLayerRef>,  T1_: Into<LLVMJITEventListenerRef>
  {
    unsafe {
      crate::OrcEE::LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener(Into::<LLVMOrcObjectLayerRef>::into(RTDyldObjLinkingLayer_), Into::<LLVMJITEventListenerRef>::into(Listener_))
    }
  }
}

