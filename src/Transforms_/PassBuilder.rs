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
pub type LLVMPassBuilderOptionsRef = *mut u8;

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

  pub fn LLVMRunPasses(M: LLVMModuleRef, Passes: *const std::ffi::c_char, TM: LLVMTargetMachineRef, Options: LLVMPassBuilderOptionsRef) -> LLVMErrorRef;

  pub fn LLVMRunPassesOnFunction(F: LLVMValueRef, Passes: *const std::ffi::c_char, TM: LLVMTargetMachineRef, Options: LLVMPassBuilderOptionsRef) -> LLVMErrorRef;

  pub fn LLVMCreatePassBuilderOptions() -> LLVMPassBuilderOptionsRef;

  pub fn LLVMPassBuilderOptionsSetVerifyEach(Options: LLVMPassBuilderOptionsRef, VerifyEach: std::ffi::c_int) -> ();

  pub fn LLVMPassBuilderOptionsSetDebugLogging(Options: LLVMPassBuilderOptionsRef, DebugLogging: std::ffi::c_int) -> ();

  pub fn LLVMPassBuilderOptionsSetAAPipeline(Options: LLVMPassBuilderOptionsRef, AAPipeline: *const std::ffi::c_char) -> ();

  pub fn LLVMPassBuilderOptionsSetLoopInterleaving(Options: LLVMPassBuilderOptionsRef, LoopInterleaving: std::ffi::c_int) -> ();

  pub fn LLVMPassBuilderOptionsSetLoopVectorization(Options: LLVMPassBuilderOptionsRef, LoopVectorization: std::ffi::c_int) -> ();

  pub fn LLVMPassBuilderOptionsSetSLPVectorization(Options: LLVMPassBuilderOptionsRef, SLPVectorization: std::ffi::c_int) -> ();

  pub fn LLVMPassBuilderOptionsSetLoopUnrolling(Options: LLVMPassBuilderOptionsRef, LoopUnrolling: std::ffi::c_int) -> ();

  pub fn LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll(Options: LLVMPassBuilderOptionsRef, ForgetAllSCEVInLoopUnroll: std::ffi::c_int) -> ();

  pub fn LLVMPassBuilderOptionsSetLicmMssaOptCap(Options: LLVMPassBuilderOptionsRef, LicmMssaOptCap: std::ffi::c_uint) -> ();

  pub fn LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap(Options: LLVMPassBuilderOptionsRef, LicmMssaNoAccForPromotionCap: std::ffi::c_uint) -> ();

  pub fn LLVMPassBuilderOptionsSetCallGraphProfile(Options: LLVMPassBuilderOptionsRef, CallGraphProfile: std::ffi::c_int) -> ();

  pub fn LLVMPassBuilderOptionsSetMergeFunctions(Options: LLVMPassBuilderOptionsRef, MergeFunctions: std::ffi::c_int) -> ();

  pub fn LLVMPassBuilderOptionsSetInlinerThreshold(Options: LLVMPassBuilderOptionsRef, Threshold: std::ffi::c_int) -> ();

  pub fn LLVMDisposePassBuilderOptions(Options: LLVMPassBuilderOptionsRef) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMRunPasses<T0_, T1_, T2_, T3_>(M_:  T0_, Passes_:  T1_, TM_:  T2_, Options_:  T3_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<LLVMTargetMachineRef>,  T3_: Into<LLVMPassBuilderOptionsRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms_::PassBuilder::LLVMRunPasses(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Passes_), Into::<LLVMTargetMachineRef>::into(TM_), Into::<LLVMPassBuilderOptionsRef>::into(Options_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMRunPassesOnFunction<T0_, T1_, T2_, T3_>(F_:  T0_, Passes_:  T1_, TM_:  T2_, Options_:  T3_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<LLVMTargetMachineRef>,  T3_: Into<LLVMPassBuilderOptionsRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms_::PassBuilder::LLVMRunPassesOnFunction(Into::<LLVMValueRef>::into(F_), Into::<*const std::ffi::c_char>::into(Passes_), Into::<LLVMTargetMachineRef>::into(TM_), Into::<LLVMPassBuilderOptionsRef>::into(Options_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMPassBuilderOptionsRef> {
  pub unsafe fn LLVMCreatePassBuilderOptions()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms_::PassBuilder::LLVMCreatePassBuilderOptions()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetVerifyEach<T0_, T1_>(Options_:  T0_, VerifyEach_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetVerifyEach(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_int>::into(VerifyEach_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetDebugLogging<T0_, T1_>(Options_:  T0_, DebugLogging_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetDebugLogging(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_int>::into(DebugLogging_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetAAPipeline<T0_, T1_>(Options_:  T0_, AAPipeline_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetAAPipeline(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<*const std::ffi::c_char>::into(AAPipeline_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetLoopInterleaving<T0_, T1_>(Options_:  T0_, LoopInterleaving_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetLoopInterleaving(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_int>::into(LoopInterleaving_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetLoopVectorization<T0_, T1_>(Options_:  T0_, LoopVectorization_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetLoopVectorization(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_int>::into(LoopVectorization_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetSLPVectorization<T0_, T1_>(Options_:  T0_, SLPVectorization_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetSLPVectorization(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_int>::into(SLPVectorization_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetLoopUnrolling<T0_, T1_>(Options_:  T0_, LoopUnrolling_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetLoopUnrolling(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_int>::into(LoopUnrolling_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll<T0_, T1_>(Options_:  T0_, ForgetAllSCEVInLoopUnroll_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_int>::into(ForgetAllSCEVInLoopUnroll_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetLicmMssaOptCap<T0_, T1_>(Options_:  T0_, LicmMssaOptCap_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetLicmMssaOptCap(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_uint>::into(LicmMssaOptCap_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap<T0_, T1_>(Options_:  T0_, LicmMssaNoAccForPromotionCap_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_uint>::into(LicmMssaNoAccForPromotionCap_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetCallGraphProfile<T0_, T1_>(Options_:  T0_, CallGraphProfile_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetCallGraphProfile(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_int>::into(CallGraphProfile_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetMergeFunctions<T0_, T1_>(Options_:  T0_, MergeFunctions_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetMergeFunctions(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_int>::into(MergeFunctions_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPassBuilderOptionsSetInlinerThreshold<T0_, T1_>(Options_:  T0_, Threshold_:  T1_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMPassBuilderOptionsSetInlinerThreshold(Into::<LLVMPassBuilderOptionsRef>::into(Options_), Into::<std::ffi::c_int>::into(Threshold_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposePassBuilderOptions<T0_>(Options_:  T0_)
  where
     T0_: Into<LLVMPassBuilderOptionsRef>
  {
    unsafe {
      crate::Transforms_::PassBuilder::LLVMDisposePassBuilderOptions(Into::<LLVMPassBuilderOptionsRef>::into(Options_))
    }
  }
}

