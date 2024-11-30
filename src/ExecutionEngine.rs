// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
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
pub type LLVMGenericValueRef = *mut u8;
pub type LLVMExecutionEngineRef = *mut u8;
pub type LLVMMCJITMemoryManagerRef = *mut u8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructLLVMMCJITCompilerOptions {
  pub OptLevel: std::ffi::c_uint,
  pub CodeModel: std::ffi::c_uint,
  pub NoFramePointerElim: std::ffi::c_int,
  pub EnableFastISel: std::ffi::c_int,
  pub MCJMM: LLVMMCJITMemoryManagerRef,
}
pub type LLVMMemoryManagerAllocateCodeSectionCallback = *mut extern fn (*mut std::ffi::c_void, std::ffi::c_ulong, std::ffi::c_uint, std::ffi::c_uint, *const std::ffi::c_char) -> *mut std::ffi::c_uchar;
pub type LLVMMemoryManagerAllocateDataSectionCallback = *mut extern fn (*mut std::ffi::c_void, std::ffi::c_ulong, std::ffi::c_uint, std::ffi::c_uint, *const std::ffi::c_char, std::ffi::c_int) -> *mut std::ffi::c_uchar;
pub type LLVMMemoryManagerFinalizeMemoryCallback = *mut extern fn (*mut std::ffi::c_void, *mut *mut std::ffi::c_char) -> std::ffi::c_int;
pub type LLVMMemoryManagerDestroyCallback = *mut extern fn (*mut std::ffi::c_void) -> ();

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

  pub fn LLVMLinkInMCJIT() -> ();

  pub fn LLVMLinkInInterpreter() -> ();

  pub fn LLVMCreateGenericValueOfInt(Ty: LLVMTypeRef, N: std::ffi::c_ulonglong, IsSigned: std::ffi::c_int) -> LLVMGenericValueRef;

  pub fn LLVMCreateGenericValueOfPointer(P: *mut std::ffi::c_void) -> LLVMGenericValueRef;

  pub fn LLVMCreateGenericValueOfFloat(Ty: LLVMTypeRef, N: f64) -> LLVMGenericValueRef;

  pub fn LLVMGenericValueIntWidth(GenValRef: LLVMGenericValueRef) -> std::ffi::c_uint;

  pub fn LLVMGenericValueToInt(GenVal: LLVMGenericValueRef, IsSigned: std::ffi::c_int) -> std::ffi::c_ulonglong;

  pub fn LLVMGenericValueToPointer(GenVal: LLVMGenericValueRef) -> *mut std::ffi::c_void;

  pub fn LLVMGenericValueToFloat(TyRef: LLVMTypeRef, GenVal: LLVMGenericValueRef) -> f64;

  pub fn LLVMDisposeGenericValue(GenVal: LLVMGenericValueRef) -> ();

  pub fn LLVMCreateExecutionEngineForModule(OutEE: *mut LLVMExecutionEngineRef, M: LLVMModuleRef, OutError: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMCreateInterpreterForModule(OutInterp: *mut LLVMExecutionEngineRef, M: LLVMModuleRef, OutError: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMCreateJITCompilerForModule(OutJIT: *mut LLVMExecutionEngineRef, M: LLVMModuleRef, OptLevel: std::ffi::c_uint, OutError: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMInitializeMCJITCompilerOptions(Options: *mut StructLLVMMCJITCompilerOptions, SizeOfOptions: std::ffi::c_ulong) -> ();

  pub fn LLVMCreateMCJITCompilerForModule(OutJIT: *mut LLVMExecutionEngineRef, M: LLVMModuleRef, Options: *mut StructLLVMMCJITCompilerOptions, SizeOfOptions: std::ffi::c_ulong, OutError: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMDisposeExecutionEngine(EE: LLVMExecutionEngineRef) -> ();

  pub fn LLVMRunStaticConstructors(EE: LLVMExecutionEngineRef) -> ();

  pub fn LLVMRunStaticDestructors(EE: LLVMExecutionEngineRef) -> ();

  pub fn LLVMRunFunctionAsMain(EE: LLVMExecutionEngineRef, F: LLVMValueRef, ArgC: std::ffi::c_uint, ArgV: *const *const std::ffi::c_char, EnvP: *const *const std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMRunFunction(EE: LLVMExecutionEngineRef, F: LLVMValueRef, NumArgs: std::ffi::c_uint, Args: *mut LLVMGenericValueRef) -> LLVMGenericValueRef;

  pub fn LLVMFreeMachineCodeForFunction(EE: LLVMExecutionEngineRef, F: LLVMValueRef) -> ();

  pub fn LLVMAddModule(EE: LLVMExecutionEngineRef, M: LLVMModuleRef) -> ();

  pub fn LLVMRemoveModule(EE: LLVMExecutionEngineRef, M: LLVMModuleRef, OutMod: *mut LLVMModuleRef, OutError: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMFindFunction(EE: LLVMExecutionEngineRef, Name: *const std::ffi::c_char, OutFn: *mut LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMRecompileAndRelinkFunction(EE: LLVMExecutionEngineRef, Fn: LLVMValueRef) -> *mut std::ffi::c_void;

  pub fn LLVMGetExecutionEngineTargetData(EE: LLVMExecutionEngineRef) -> LLVMTargetDataRef;

  pub fn LLVMGetExecutionEngineTargetMachine(EE: LLVMExecutionEngineRef) -> LLVMTargetMachineRef;

  pub fn LLVMAddGlobalMapping(EE: LLVMExecutionEngineRef, Global: LLVMValueRef, Addr: *mut std::ffi::c_void) -> ();

  pub fn LLVMGetPointerToGlobal(EE: LLVMExecutionEngineRef, Global: LLVMValueRef) -> *mut std::ffi::c_void;

  pub fn LLVMGetGlobalValueAddress(EE: LLVMExecutionEngineRef, Name: *const std::ffi::c_char) -> std::ffi::c_ulong;

  pub fn LLVMGetFunctionAddress(EE: LLVMExecutionEngineRef, Name: *const std::ffi::c_char) -> std::ffi::c_ulong;

  pub fn LLVMExecutionEngineGetErrMsg(EE: LLVMExecutionEngineRef, OutError: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMCreateSimpleMCJITMemoryManager(Opaque: *mut std::ffi::c_void, AllocateCodeSection: LLVMMemoryManagerAllocateCodeSectionCallback, AllocateDataSection: LLVMMemoryManagerAllocateDataSectionCallback, FinalizeMemory: LLVMMemoryManagerFinalizeMemoryCallback, Destroy: LLVMMemoryManagerDestroyCallback) -> LLVMMCJITMemoryManagerRef;

  pub fn LLVMDisposeMCJITMemoryManager(MM: LLVMMCJITMemoryManagerRef) -> ();

  pub fn LLVMCreateGDBRegistrationListener() -> LLVMJITEventListenerRef;

  pub fn LLVMCreateIntelJITEventListener() -> LLVMJITEventListenerRef;

  pub fn LLVMCreateOProfileJITEventListener() -> LLVMJITEventListenerRef;

  pub fn LLVMCreatePerfJITEventListener() -> LLVMJITEventListenerRef;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMGenericValueRef> {
  pub unsafe fn LLVMCreateGenericValueOfInt<T0_, T1_, T2_>(Ty_:  T0_, N_:  T1_, IsSigned_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_ulonglong>,  T2_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateGenericValueOfInt(Into::<LLVMTypeRef>::into(Ty_), Into::<std::ffi::c_ulonglong>::into(N_), Into::<std::ffi::c_int>::into(IsSigned_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMGenericValueRef> {
  pub unsafe fn LLVMCreateGenericValueOfPointer<T0_>(P_:  T0_)-> Tret_
  where
     T0_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateGenericValueOfPointer(Into::<*mut std::ffi::c_void>::into(P_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMGenericValueRef> {
  pub unsafe fn LLVMCreateGenericValueOfFloat<T0_, T1_>(Ty_:  T0_, N_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<f64>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateGenericValueOfFloat(Into::<LLVMTypeRef>::into(Ty_), Into::<f64>::into(N_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGenericValueIntWidth<T0_>(GenValRef_:  T0_)-> Tret_
  where
     T0_: Into<LLVMGenericValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMGenericValueIntWidth(Into::<LLVMGenericValueRef>::into(GenValRef_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulonglong> {
  pub unsafe fn LLVMGenericValueToInt<T0_, T1_>(GenVal_:  T0_, IsSigned_:  T1_)-> Tret_
  where
     T0_: Into<LLVMGenericValueRef>,  T1_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMGenericValueToInt(Into::<LLVMGenericValueRef>::into(GenVal_), Into::<std::ffi::c_int>::into(IsSigned_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_void> {
  pub unsafe fn LLVMGenericValueToPointer<T0_>(GenVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMGenericValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMGenericValueToPointer(Into::<LLVMGenericValueRef>::into(GenVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn LLVMGenericValueToFloat<T0_, T1_>(TyRef_:  T0_, GenVal_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<LLVMGenericValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMGenericValueToFloat(Into::<LLVMTypeRef>::into(TyRef_), Into::<LLVMGenericValueRef>::into(GenVal_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeGenericValue<T0_>(GenVal_:  T0_)
  where
     T0_: Into<LLVMGenericValueRef>
  {
    unsafe {
      crate::ExecutionEngine::LLVMDisposeGenericValue(Into::<LLVMGenericValueRef>::into(GenVal_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMCreateExecutionEngineForModule<T0_, T1_, T2_>(OutEE_:  T0_, M_:  T1_, OutError_:  T2_)-> Tret_
  where
     T0_: Into<*mut LLVMExecutionEngineRef>,  T1_: Into<LLVMModuleRef>,  T2_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateExecutionEngineForModule(Into::<*mut LLVMExecutionEngineRef>::into(OutEE_), Into::<LLVMModuleRef>::into(M_), Into::<*mut *mut std::ffi::c_char>::into(OutError_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMCreateInterpreterForModule<T0_, T1_, T2_>(OutInterp_:  T0_, M_:  T1_, OutError_:  T2_)-> Tret_
  where
     T0_: Into<*mut LLVMExecutionEngineRef>,  T1_: Into<LLVMModuleRef>,  T2_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateInterpreterForModule(Into::<*mut LLVMExecutionEngineRef>::into(OutInterp_), Into::<LLVMModuleRef>::into(M_), Into::<*mut *mut std::ffi::c_char>::into(OutError_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMCreateJITCompilerForModule<T0_, T1_, T2_, T3_>(OutJIT_:  T0_, M_:  T1_, OptLevel_:  T2_, OutError_:  T3_)-> Tret_
  where
     T0_: Into<*mut LLVMExecutionEngineRef>,  T1_: Into<LLVMModuleRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateJITCompilerForModule(Into::<*mut LLVMExecutionEngineRef>::into(OutJIT_), Into::<LLVMModuleRef>::into(M_), Into::<std::ffi::c_uint>::into(OptLevel_), Into::<*mut *mut std::ffi::c_char>::into(OutError_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMInitializeMCJITCompilerOptions<T0_, T1_>(Options_:  T0_, SizeOfOptions_:  T1_)
  where
     T0_: Into<*mut StructLLVMMCJITCompilerOptions>,  T1_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::ExecutionEngine::LLVMInitializeMCJITCompilerOptions(Into::<*mut StructLLVMMCJITCompilerOptions>::into(Options_), Into::<std::ffi::c_ulong>::into(SizeOfOptions_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMCreateMCJITCompilerForModule<T0_, T1_, T2_, T3_, T4_>(OutJIT_:  T0_, M_:  T1_, Options_:  T2_, SizeOfOptions_:  T3_, OutError_:  T4_)-> Tret_
  where
     T0_: Into<*mut LLVMExecutionEngineRef>,  T1_: Into<LLVMModuleRef>,  T2_: Into<*mut StructLLVMMCJITCompilerOptions>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateMCJITCompilerForModule(Into::<*mut LLVMExecutionEngineRef>::into(OutJIT_), Into::<LLVMModuleRef>::into(M_), Into::<*mut StructLLVMMCJITCompilerOptions>::into(Options_), Into::<std::ffi::c_ulong>::into(SizeOfOptions_), Into::<*mut *mut std::ffi::c_char>::into(OutError_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeExecutionEngine<T0_>(EE_:  T0_)
  where
     T0_: Into<LLVMExecutionEngineRef>
  {
    unsafe {
      crate::ExecutionEngine::LLVMDisposeExecutionEngine(Into::<LLVMExecutionEngineRef>::into(EE_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMRunStaticConstructors<T0_>(EE_:  T0_)
  where
     T0_: Into<LLVMExecutionEngineRef>
  {
    unsafe {
      crate::ExecutionEngine::LLVMRunStaticConstructors(Into::<LLVMExecutionEngineRef>::into(EE_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMRunStaticDestructors<T0_>(EE_:  T0_)
  where
     T0_: Into<LLVMExecutionEngineRef>
  {
    unsafe {
      crate::ExecutionEngine::LLVMRunStaticDestructors(Into::<LLVMExecutionEngineRef>::into(EE_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMRunFunctionAsMain<T0_, T1_, T2_, T3_, T4_>(EE_:  T0_, F_:  T1_, ArgC_:  T2_, ArgV_:  T3_, EnvP_:  T4_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<*const *const std::ffi::c_char>,  T4_: Into<*const *const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMRunFunctionAsMain(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<LLVMValueRef>::into(F_), Into::<std::ffi::c_uint>::into(ArgC_), Into::<*const *const std::ffi::c_char>::into(ArgV_), Into::<*const *const std::ffi::c_char>::into(EnvP_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMGenericValueRef> {
  pub unsafe fn LLVMRunFunction<T0_, T1_, T2_, T3_>(EE_:  T0_, F_:  T1_, NumArgs_:  T2_, Args_:  T3_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<*mut LLVMGenericValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMRunFunction(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<LLVMValueRef>::into(F_), Into::<std::ffi::c_uint>::into(NumArgs_), Into::<*mut LLVMGenericValueRef>::into(Args_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMFreeMachineCodeForFunction<T0_, T1_>(EE_:  T0_, F_:  T1_)
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::ExecutionEngine::LLVMFreeMachineCodeForFunction(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<LLVMValueRef>::into(F_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddModule<T0_, T1_>(EE_:  T0_, M_:  T1_)
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<LLVMModuleRef>
  {
    unsafe {
      crate::ExecutionEngine::LLVMAddModule(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<LLVMModuleRef>::into(M_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMRemoveModule<T0_, T1_, T2_, T3_>(EE_:  T0_, M_:  T1_, OutMod_:  T2_, OutError_:  T3_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<LLVMModuleRef>,  T2_: Into<*mut LLVMModuleRef>,  T3_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMRemoveModule(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<LLVMModuleRef>::into(M_), Into::<*mut LLVMModuleRef>::into(OutMod_), Into::<*mut *mut std::ffi::c_char>::into(OutError_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMFindFunction<T0_, T1_, T2_>(EE_:  T0_, Name_:  T1_, OutFn_:  T2_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*mut LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMFindFunction(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<*const std::ffi::c_char>::into(Name_), Into::<*mut LLVMValueRef>::into(OutFn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_void> {
  pub unsafe fn LLVMRecompileAndRelinkFunction<T0_, T1_>(EE_:  T0_, Fn_:  T1_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMRecompileAndRelinkFunction(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetDataRef> {
  pub unsafe fn LLVMGetExecutionEngineTargetData<T0_>(EE_:  T0_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMGetExecutionEngineTargetData(Into::<LLVMExecutionEngineRef>::into(EE_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetMachineRef> {
  pub unsafe fn LLVMGetExecutionEngineTargetMachine<T0_>(EE_:  T0_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMGetExecutionEngineTargetMachine(Into::<LLVMExecutionEngineRef>::into(EE_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddGlobalMapping<T0_, T1_, T2_>(EE_:  T0_, Global_:  T1_, Addr_:  T2_)
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::ExecutionEngine::LLVMAddGlobalMapping(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<LLVMValueRef>::into(Global_), Into::<*mut std::ffi::c_void>::into(Addr_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_void> {
  pub unsafe fn LLVMGetPointerToGlobal<T0_, T1_>(EE_:  T0_, Global_:  T1_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMGetPointerToGlobal(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<LLVMValueRef>::into(Global_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetGlobalValueAddress<T0_, T1_>(EE_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMGetGlobalValueAddress(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetFunctionAddress<T0_, T1_>(EE_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMGetFunctionAddress(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMExecutionEngineGetErrMsg<T0_, T1_>(EE_:  T0_, OutError_:  T1_)-> Tret_
  where
     T0_: Into<LLVMExecutionEngineRef>,  T1_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMExecutionEngineGetErrMsg(Into::<LLVMExecutionEngineRef>::into(EE_), Into::<*mut *mut std::ffi::c_char>::into(OutError_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMCJITMemoryManagerRef> {
  pub unsafe fn LLVMCreateSimpleMCJITMemoryManager<T0_, T1_, T2_, T3_, T4_>(Opaque_:  T0_, AllocateCodeSection_:  T1_, AllocateDataSection_:  T2_, FinalizeMemory_:  T3_, Destroy_:  T4_)-> Tret_
  where
     T0_: Into<*mut std::ffi::c_void>,  T1_: Into<LLVMMemoryManagerAllocateCodeSectionCallback>,  T2_: Into<LLVMMemoryManagerAllocateDataSectionCallback>,  T3_: Into<LLVMMemoryManagerFinalizeMemoryCallback>,  T4_: Into<LLVMMemoryManagerDestroyCallback>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateSimpleMCJITMemoryManager(Into::<*mut std::ffi::c_void>::into(Opaque_), Into::<LLVMMemoryManagerAllocateCodeSectionCallback>::into(AllocateCodeSection_), Into::<LLVMMemoryManagerAllocateDataSectionCallback>::into(AllocateDataSection_), Into::<LLVMMemoryManagerFinalizeMemoryCallback>::into(FinalizeMemory_), Into::<LLVMMemoryManagerDestroyCallback>::into(Destroy_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeMCJITMemoryManager<T0_>(MM_:  T0_)
  where
     T0_: Into<LLVMMCJITMemoryManagerRef>
  {
    unsafe {
      crate::ExecutionEngine::LLVMDisposeMCJITMemoryManager(Into::<LLVMMCJITMemoryManagerRef>::into(MM_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMJITEventListenerRef> {
  pub unsafe fn LLVMCreateGDBRegistrationListener()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateGDBRegistrationListener()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMJITEventListenerRef> {
  pub unsafe fn LLVMCreateIntelJITEventListener()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateIntelJITEventListener()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMJITEventListenerRef> {
  pub unsafe fn LLVMCreateOProfileJITEventListener()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreateOProfileJITEventListener()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMJITEventListenerRef> {
  pub unsafe fn LLVMCreatePerfJITEventListener()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::LLVMCreatePerfJITEventListener()
      }
    )
  }
}

