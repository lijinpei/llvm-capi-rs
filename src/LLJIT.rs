// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::Error::*;
use crate::ExternC::*;
use crate::Orc::*;
use crate::Error::*;
use crate::TargetMachine::*;
use crate::ExternC::*;
use crate::Target::*;
use crate::ExternC::*;
use crate::Types::*;
use crate::DataTypes::*;
use crate::ExternC::*;
use crate::Types::*;
use crate::Types::*;
use crate::TargetMachine::*;
use crate::Types::*;
pub type LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction = *mut extern fn (*mut std::ffi::c_void, LLVMOrcExecutionSessionRef, *const std::ffi::c_char) -> LLVMOrcObjectLayerRef;
pub type LLVMOrcLLJITBuilderRef = *mut u8;
pub type LLVMOrcLLJITRef = *mut u8;

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

  pub fn LLVMOrcCreateLLJITBuilder() -> LLVMOrcLLJITBuilderRef;

  pub fn LLVMOrcDisposeLLJITBuilder(Builder: LLVMOrcLLJITBuilderRef) -> ();

  pub fn LLVMOrcLLJITBuilderSetJITTargetMachineBuilder(Builder: LLVMOrcLLJITBuilderRef, JTMB: LLVMOrcJITTargetMachineBuilderRef) -> ();

  pub fn LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator(Builder: LLVMOrcLLJITBuilderRef, F: LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction, Ctx: *mut std::ffi::c_void) -> ();

  pub fn LLVMOrcCreateLLJIT(Result: *mut LLVMOrcLLJITRef, Builder: LLVMOrcLLJITBuilderRef) -> LLVMErrorRef;

  pub fn LLVMOrcDisposeLLJIT(J: LLVMOrcLLJITRef) -> LLVMErrorRef;

  pub fn LLVMOrcLLJITGetExecutionSession(J: LLVMOrcLLJITRef) -> LLVMOrcExecutionSessionRef;

  pub fn LLVMOrcLLJITGetMainJITDylib(J: LLVMOrcLLJITRef) -> LLVMOrcJITDylibRef;

  pub fn LLVMOrcLLJITGetTripleString(J: LLVMOrcLLJITRef) -> *const std::ffi::c_char;

  pub fn LLVMOrcLLJITGetGlobalPrefix(J: LLVMOrcLLJITRef) -> std::ffi::c_char;

  pub fn LLVMOrcLLJITMangleAndIntern(J: LLVMOrcLLJITRef, UnmangledName: *const std::ffi::c_char) -> LLVMOrcSymbolStringPoolEntryRef;

  pub fn LLVMOrcLLJITAddObjectFile(J: LLVMOrcLLJITRef, JD: LLVMOrcJITDylibRef, ObjBuffer: LLVMMemoryBufferRef) -> LLVMErrorRef;

  pub fn LLVMOrcLLJITAddObjectFileWithRT(J: LLVMOrcLLJITRef, RT: LLVMOrcResourceTrackerRef, ObjBuffer: LLVMMemoryBufferRef) -> LLVMErrorRef;

  pub fn LLVMOrcLLJITAddLLVMIRModule(J: LLVMOrcLLJITRef, JD: LLVMOrcJITDylibRef, TSM: LLVMOrcThreadSafeModuleRef) -> LLVMErrorRef;

  pub fn LLVMOrcLLJITAddLLVMIRModuleWithRT(J: LLVMOrcLLJITRef, JD: LLVMOrcResourceTrackerRef, TSM: LLVMOrcThreadSafeModuleRef) -> LLVMErrorRef;

  pub fn LLVMOrcLLJITLookup(J: LLVMOrcLLJITRef, Result: *mut std::ffi::c_ulong, Name: *const std::ffi::c_char) -> LLVMErrorRef;

  pub fn LLVMOrcLLJITGetObjLinkingLayer(J: LLVMOrcLLJITRef) -> LLVMOrcObjectLayerRef;

  pub fn LLVMOrcLLJITGetObjTransformLayer(J: LLVMOrcLLJITRef) -> LLVMOrcObjectTransformLayerRef;

  pub fn LLVMOrcLLJITGetIRTransformLayer(J: LLVMOrcLLJITRef) -> LLVMOrcIRTransformLayerRef;

  pub fn LLVMOrcLLJITGetDataLayoutStr(J: LLVMOrcLLJITRef) -> *const std::ffi::c_char;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcLLJITBuilderRef> {
  pub unsafe fn LLVMOrcCreateLLJITBuilder()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcCreateLLJITBuilder()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcDisposeLLJITBuilder<T0_>(Builder_:  T0_)
  where
     T0_: Into<LLVMOrcLLJITBuilderRef>
  {
    unsafe {
      crate::LLJIT::LLVMOrcDisposeLLJITBuilder(Into::<LLVMOrcLLJITBuilderRef>::into(Builder_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcLLJITBuilderSetJITTargetMachineBuilder<T0_, T1_>(Builder_:  T0_, JTMB_:  T1_)
  where
     T0_: Into<LLVMOrcLLJITBuilderRef>,  T1_: Into<LLVMOrcJITTargetMachineBuilderRef>
  {
    unsafe {
      crate::LLJIT::LLVMOrcLLJITBuilderSetJITTargetMachineBuilder(Into::<LLVMOrcLLJITBuilderRef>::into(Builder_), Into::<LLVMOrcJITTargetMachineBuilderRef>::into(JTMB_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator<T0_, T1_, T2_>(Builder_:  T0_, F_:  T1_, Ctx_:  T2_)
  where
     T0_: Into<LLVMOrcLLJITBuilderRef>,  T1_: Into<LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::LLJIT::LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator(Into::<LLVMOrcLLJITBuilderRef>::into(Builder_), Into::<LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction>::into(F_), Into::<*mut std::ffi::c_void>::into(Ctx_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcCreateLLJIT<T0_, T1_>(Result_:  T0_, Builder_:  T1_)-> Tret_
  where
     T0_: Into<*mut LLVMOrcLLJITRef>,  T1_: Into<LLVMOrcLLJITBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcCreateLLJIT(Into::<*mut LLVMOrcLLJITRef>::into(Result_), Into::<LLVMOrcLLJITBuilderRef>::into(Builder_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcDisposeLLJIT<T0_>(J_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcDisposeLLJIT(Into::<LLVMOrcLLJITRef>::into(J_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcExecutionSessionRef> {
  pub unsafe fn LLVMOrcLLJITGetExecutionSession<T0_>(J_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITGetExecutionSession(Into::<LLVMOrcLLJITRef>::into(J_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcJITDylibRef> {
  pub unsafe fn LLVMOrcLLJITGetMainJITDylib<T0_>(J_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITGetMainJITDylib(Into::<LLVMOrcLLJITRef>::into(J_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMOrcLLJITGetTripleString<T0_>(J_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITGetTripleString(Into::<LLVMOrcLLJITRef>::into(J_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_char> {
  pub unsafe fn LLVMOrcLLJITGetGlobalPrefix<T0_>(J_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITGetGlobalPrefix(Into::<LLVMOrcLLJITRef>::into(J_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcSymbolStringPoolEntryRef> {
  pub unsafe fn LLVMOrcLLJITMangleAndIntern<T0_, T1_>(J_:  T0_, UnmangledName_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITMangleAndIntern(Into::<LLVMOrcLLJITRef>::into(J_), Into::<*const std::ffi::c_char>::into(UnmangledName_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcLLJITAddObjectFile<T0_, T1_, T2_>(J_:  T0_, JD_:  T1_, ObjBuffer_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>,  T1_: Into<LLVMOrcJITDylibRef>,  T2_: Into<LLVMMemoryBufferRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITAddObjectFile(Into::<LLVMOrcLLJITRef>::into(J_), Into::<LLVMOrcJITDylibRef>::into(JD_), Into::<LLVMMemoryBufferRef>::into(ObjBuffer_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcLLJITAddObjectFileWithRT<T0_, T1_, T2_>(J_:  T0_, RT_:  T1_, ObjBuffer_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>,  T1_: Into<LLVMOrcResourceTrackerRef>,  T2_: Into<LLVMMemoryBufferRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITAddObjectFileWithRT(Into::<LLVMOrcLLJITRef>::into(J_), Into::<LLVMOrcResourceTrackerRef>::into(RT_), Into::<LLVMMemoryBufferRef>::into(ObjBuffer_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcLLJITAddLLVMIRModule<T0_, T1_, T2_>(J_:  T0_, JD_:  T1_, TSM_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>,  T1_: Into<LLVMOrcJITDylibRef>,  T2_: Into<LLVMOrcThreadSafeModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITAddLLVMIRModule(Into::<LLVMOrcLLJITRef>::into(J_), Into::<LLVMOrcJITDylibRef>::into(JD_), Into::<LLVMOrcThreadSafeModuleRef>::into(TSM_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcLLJITAddLLVMIRModuleWithRT<T0_, T1_, T2_>(J_:  T0_, JD_:  T1_, TSM_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>,  T1_: Into<LLVMOrcResourceTrackerRef>,  T2_: Into<LLVMOrcThreadSafeModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITAddLLVMIRModuleWithRT(Into::<LLVMOrcLLJITRef>::into(J_), Into::<LLVMOrcResourceTrackerRef>::into(JD_), Into::<LLVMOrcThreadSafeModuleRef>::into(TSM_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMOrcLLJITLookup<T0_, T1_, T2_>(J_:  T0_, Result_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>,  T1_: Into<*mut std::ffi::c_ulong>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITLookup(Into::<LLVMOrcLLJITRef>::into(J_), Into::<*mut std::ffi::c_ulong>::into(Result_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcObjectLayerRef> {
  pub unsafe fn LLVMOrcLLJITGetObjLinkingLayer<T0_>(J_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITGetObjLinkingLayer(Into::<LLVMOrcLLJITRef>::into(J_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcObjectTransformLayerRef> {
  pub unsafe fn LLVMOrcLLJITGetObjTransformLayer<T0_>(J_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITGetObjTransformLayer(Into::<LLVMOrcLLJITRef>::into(J_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOrcIRTransformLayerRef> {
  pub unsafe fn LLVMOrcLLJITGetIRTransformLayer<T0_>(J_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITGetIRTransformLayer(Into::<LLVMOrcLLJITRef>::into(J_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMOrcLLJITGetDataLayoutStr<T0_>(J_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOrcLLJITRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::LLJIT::LLVMOrcLLJITGetDataLayoutStr(Into::<LLVMOrcLLJITRef>::into(J_))
      }
    )
  }
}

