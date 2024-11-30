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
use crate::Types::*;
pub type LLVMTargetMachineOptionsRef = *mut u8;
pub type LLVMTargetMachineRef = *mut u8;
pub type LLVMTargetRef = *mut u8;

pub const LLVMCodeGenLevelNone: std::ffi::c_uint = 0;
pub const LLVMCodeGenLevelLess: std::ffi::c_uint = 1;
pub const LLVMCodeGenLevelDefault: std::ffi::c_uint = 2;
pub const LLVMCodeGenLevelAggressive: std::ffi::c_uint = 3;
pub type LLVMCodeGenOptLevel = std::ffi::c_uint;

pub const LLVMRelocDefault: std::ffi::c_uint = 0;
pub const LLVMRelocStatic: std::ffi::c_uint = 1;
pub const LLVMRelocPIC: std::ffi::c_uint = 2;
pub const LLVMRelocDynamicNoPic: std::ffi::c_uint = 3;
pub const LLVMRelocROPI: std::ffi::c_uint = 4;
pub const LLVMRelocRWPI: std::ffi::c_uint = 5;
pub const LLVMRelocROPI_RWPI: std::ffi::c_uint = 6;
pub type LLVMRelocMode = std::ffi::c_uint;

pub const LLVMCodeModelDefault: std::ffi::c_uint = 0;
pub const LLVMCodeModelJITDefault: std::ffi::c_uint = 1;
pub const LLVMCodeModelTiny: std::ffi::c_uint = 2;
pub const LLVMCodeModelSmall: std::ffi::c_uint = 3;
pub const LLVMCodeModelKernel: std::ffi::c_uint = 4;
pub const LLVMCodeModelMedium: std::ffi::c_uint = 5;
pub const LLVMCodeModelLarge: std::ffi::c_uint = 6;
pub type LLVMCodeModel = std::ffi::c_uint;

pub const LLVMAssemblyFile: std::ffi::c_uint = 0;
pub const LLVMObjectFile: std::ffi::c_uint = 1;
pub type LLVMCodeGenFileType = std::ffi::c_uint;

pub const LLVMGlobalISelAbortEnable: std::ffi::c_uint = 0;
pub const LLVMGlobalISelAbortDisable: std::ffi::c_uint = 1;
pub const LLVMGlobalISelAbortDisableWithDiag: std::ffi::c_uint = 2;
pub type LLVMGlobalISelAbortMode = std::ffi::c_uint;

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

  pub fn LLVMGetFirstTarget() -> LLVMTargetRef;

  pub fn LLVMGetNextTarget(T: LLVMTargetRef) -> LLVMTargetRef;

  pub fn LLVMGetTargetFromName(Name: *const std::ffi::c_char) -> LLVMTargetRef;

  pub fn LLVMGetTargetFromTriple(Triple: *const std::ffi::c_char, T: *mut LLVMTargetRef, ErrorMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMGetTargetName(T: LLVMTargetRef) -> *const std::ffi::c_char;

  pub fn LLVMGetTargetDescription(T: LLVMTargetRef) -> *const std::ffi::c_char;

  pub fn LLVMTargetHasJIT(T: LLVMTargetRef) -> std::ffi::c_int;

  pub fn LLVMTargetHasTargetMachine(T: LLVMTargetRef) -> std::ffi::c_int;

  pub fn LLVMTargetHasAsmBackend(T: LLVMTargetRef) -> std::ffi::c_int;

  pub fn LLVMCreateTargetMachineOptions() -> LLVMTargetMachineOptionsRef;

  pub fn LLVMDisposeTargetMachineOptions(Options: LLVMTargetMachineOptionsRef) -> ();

  pub fn LLVMTargetMachineOptionsSetCPU(Options: LLVMTargetMachineOptionsRef, CPU: *const std::ffi::c_char) -> ();

  pub fn LLVMTargetMachineOptionsSetFeatures(Options: LLVMTargetMachineOptionsRef, Features: *const std::ffi::c_char) -> ();

  pub fn LLVMTargetMachineOptionsSetABI(Options: LLVMTargetMachineOptionsRef, ABI: *const std::ffi::c_char) -> ();

  pub fn LLVMTargetMachineOptionsSetCodeGenOptLevel(Options: LLVMTargetMachineOptionsRef, Level: std::ffi::c_uint) -> ();

  pub fn LLVMTargetMachineOptionsSetRelocMode(Options: LLVMTargetMachineOptionsRef, Reloc: std::ffi::c_uint) -> ();

  pub fn LLVMTargetMachineOptionsSetCodeModel(Options: LLVMTargetMachineOptionsRef, CodeModel: std::ffi::c_uint) -> ();

  pub fn LLVMCreateTargetMachineWithOptions(T: LLVMTargetRef, Triple: *const std::ffi::c_char, Options: LLVMTargetMachineOptionsRef) -> LLVMTargetMachineRef;

  pub fn LLVMCreateTargetMachine(T: LLVMTargetRef, Triple: *const std::ffi::c_char, CPU: *const std::ffi::c_char, Features: *const std::ffi::c_char, Level: std::ffi::c_uint, Reloc: std::ffi::c_uint, CodeModel: std::ffi::c_uint) -> LLVMTargetMachineRef;

  pub fn LLVMDisposeTargetMachine(T: LLVMTargetMachineRef) -> ();

  pub fn LLVMGetTargetMachineTarget(T: LLVMTargetMachineRef) -> LLVMTargetRef;

  pub fn LLVMGetTargetMachineTriple(T: LLVMTargetMachineRef) -> *mut std::ffi::c_char;

  pub fn LLVMGetTargetMachineCPU(T: LLVMTargetMachineRef) -> *mut std::ffi::c_char;

  pub fn LLVMGetTargetMachineFeatureString(T: LLVMTargetMachineRef) -> *mut std::ffi::c_char;

  pub fn LLVMCreateTargetDataLayout(T: LLVMTargetMachineRef) -> LLVMTargetDataRef;

  pub fn LLVMSetTargetMachineAsmVerbosity(T: LLVMTargetMachineRef, VerboseAsm: std::ffi::c_int) -> ();

  pub fn LLVMSetTargetMachineFastISel(T: LLVMTargetMachineRef, Enable: std::ffi::c_int) -> ();

  pub fn LLVMSetTargetMachineGlobalISel(T: LLVMTargetMachineRef, Enable: std::ffi::c_int) -> ();

  pub fn LLVMSetTargetMachineGlobalISelAbort(T: LLVMTargetMachineRef, Mode: std::ffi::c_uint) -> ();

  pub fn LLVMSetTargetMachineMachineOutliner(T: LLVMTargetMachineRef, Enable: std::ffi::c_int) -> ();

  pub fn LLVMTargetMachineEmitToFile(T: LLVMTargetMachineRef, M: LLVMModuleRef, Filename: *const std::ffi::c_char, codegen: std::ffi::c_uint, ErrorMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMTargetMachineEmitToMemoryBuffer(T: LLVMTargetMachineRef, M: LLVMModuleRef, codegen: std::ffi::c_uint, ErrorMessage: *mut *mut std::ffi::c_char, OutMemBuf: *mut LLVMMemoryBufferRef) -> std::ffi::c_int;

  pub fn LLVMGetDefaultTargetTriple() -> *mut std::ffi::c_char;

  pub fn LLVMNormalizeTargetTriple(triple: *const std::ffi::c_char) -> *mut std::ffi::c_char;

  pub fn LLVMGetHostCPUName() -> *mut std::ffi::c_char;

  pub fn LLVMGetHostCPUFeatures() -> *mut std::ffi::c_char;

  pub fn LLVMAddAnalysisPasses(T: LLVMTargetMachineRef, PM: LLVMPassManagerRef) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetRef> {
  pub unsafe fn LLVMGetFirstTarget()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetFirstTarget()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetRef> {
  pub unsafe fn LLVMGetNextTarget<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetNextTarget(Into::<LLVMTargetRef>::into(T_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetRef> {
  pub unsafe fn LLVMGetTargetFromName<T0_>(Name_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetTargetFromName(Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetTargetFromTriple<T0_, T1_, T2_>(Triple_:  T0_, T_:  T1_, ErrorMessage_:  T2_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<*mut LLVMTargetRef>,  T2_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetTargetFromTriple(Into::<*const std::ffi::c_char>::into(Triple_), Into::<*mut LLVMTargetRef>::into(T_), Into::<*mut *mut std::ffi::c_char>::into(ErrorMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetTargetName<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetTargetName(Into::<LLVMTargetRef>::into(T_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetTargetDescription<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetTargetDescription(Into::<LLVMTargetRef>::into(T_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMTargetHasJIT<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMTargetHasJIT(Into::<LLVMTargetRef>::into(T_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMTargetHasTargetMachine<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMTargetHasTargetMachine(Into::<LLVMTargetRef>::into(T_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMTargetHasAsmBackend<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMTargetHasAsmBackend(Into::<LLVMTargetRef>::into(T_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetMachineOptionsRef> {
  pub unsafe fn LLVMCreateTargetMachineOptions()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMCreateTargetMachineOptions()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeTargetMachineOptions<T0_>(Options_:  T0_)
  where
     T0_: Into<LLVMTargetMachineOptionsRef>
  {
    unsafe {
      crate::TargetMachine::LLVMDisposeTargetMachineOptions(Into::<LLVMTargetMachineOptionsRef>::into(Options_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMTargetMachineOptionsSetCPU<T0_, T1_>(Options_:  T0_, CPU_:  T1_)
  where
     T0_: Into<LLVMTargetMachineOptionsRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::TargetMachine::LLVMTargetMachineOptionsSetCPU(Into::<LLVMTargetMachineOptionsRef>::into(Options_), Into::<*const std::ffi::c_char>::into(CPU_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMTargetMachineOptionsSetFeatures<T0_, T1_>(Options_:  T0_, Features_:  T1_)
  where
     T0_: Into<LLVMTargetMachineOptionsRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::TargetMachine::LLVMTargetMachineOptionsSetFeatures(Into::<LLVMTargetMachineOptionsRef>::into(Options_), Into::<*const std::ffi::c_char>::into(Features_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMTargetMachineOptionsSetABI<T0_, T1_>(Options_:  T0_, ABI_:  T1_)
  where
     T0_: Into<LLVMTargetMachineOptionsRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::TargetMachine::LLVMTargetMachineOptionsSetABI(Into::<LLVMTargetMachineOptionsRef>::into(Options_), Into::<*const std::ffi::c_char>::into(ABI_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMTargetMachineOptionsSetCodeGenOptLevel<T0_, T1_>(Options_:  T0_, Level_:  T1_)
  where
     T0_: Into<LLVMTargetMachineOptionsRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::TargetMachine::LLVMTargetMachineOptionsSetCodeGenOptLevel(Into::<LLVMTargetMachineOptionsRef>::into(Options_), Into::<std::ffi::c_uint>::into(Level_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMTargetMachineOptionsSetRelocMode<T0_, T1_>(Options_:  T0_, Reloc_:  T1_)
  where
     T0_: Into<LLVMTargetMachineOptionsRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::TargetMachine::LLVMTargetMachineOptionsSetRelocMode(Into::<LLVMTargetMachineOptionsRef>::into(Options_), Into::<std::ffi::c_uint>::into(Reloc_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMTargetMachineOptionsSetCodeModel<T0_, T1_>(Options_:  T0_, CodeModel_:  T1_)
  where
     T0_: Into<LLVMTargetMachineOptionsRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::TargetMachine::LLVMTargetMachineOptionsSetCodeModel(Into::<LLVMTargetMachineOptionsRef>::into(Options_), Into::<std::ffi::c_uint>::into(CodeModel_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetMachineRef> {
  pub unsafe fn LLVMCreateTargetMachineWithOptions<T0_, T1_, T2_>(T_:  T0_, Triple_:  T1_, Options_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTargetRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<LLVMTargetMachineOptionsRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMCreateTargetMachineWithOptions(Into::<LLVMTargetRef>::into(T_), Into::<*const std::ffi::c_char>::into(Triple_), Into::<LLVMTargetMachineOptionsRef>::into(Options_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetMachineRef> {
  pub unsafe fn LLVMCreateTargetMachine<T0_, T1_, T2_, T3_, T4_, T5_, T6_>(T_:  T0_, Triple_:  T1_, CPU_:  T2_, Features_:  T3_, Level_:  T4_, Reloc_:  T5_, CodeModel_:  T6_)-> Tret_
  where
     T0_: Into<LLVMTargetRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<*const std::ffi::c_char>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMCreateTargetMachine(Into::<LLVMTargetRef>::into(T_), Into::<*const std::ffi::c_char>::into(Triple_), Into::<*const std::ffi::c_char>::into(CPU_), Into::<*const std::ffi::c_char>::into(Features_), Into::<std::ffi::c_uint>::into(Level_), Into::<std::ffi::c_uint>::into(Reloc_), Into::<std::ffi::c_uint>::into(CodeModel_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeTargetMachine<T0_>(T_:  T0_)
  where
     T0_: Into<LLVMTargetMachineRef>
  {
    unsafe {
      crate::TargetMachine::LLVMDisposeTargetMachine(Into::<LLVMTargetMachineRef>::into(T_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetRef> {
  pub unsafe fn LLVMGetTargetMachineTarget<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetMachineRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetTargetMachineTarget(Into::<LLVMTargetMachineRef>::into(T_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMGetTargetMachineTriple<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetMachineRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetTargetMachineTriple(Into::<LLVMTargetMachineRef>::into(T_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMGetTargetMachineCPU<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetMachineRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetTargetMachineCPU(Into::<LLVMTargetMachineRef>::into(T_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMGetTargetMachineFeatureString<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetMachineRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetTargetMachineFeatureString(Into::<LLVMTargetMachineRef>::into(T_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetDataRef> {
  pub unsafe fn LLVMCreateTargetDataLayout<T0_>(T_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetMachineRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMCreateTargetDataLayout(Into::<LLVMTargetMachineRef>::into(T_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetTargetMachineAsmVerbosity<T0_, T1_>(T_:  T0_, VerboseAsm_:  T1_)
  where
     T0_: Into<LLVMTargetMachineRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::TargetMachine::LLVMSetTargetMachineAsmVerbosity(Into::<LLVMTargetMachineRef>::into(T_), Into::<std::ffi::c_int>::into(VerboseAsm_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetTargetMachineFastISel<T0_, T1_>(T_:  T0_, Enable_:  T1_)
  where
     T0_: Into<LLVMTargetMachineRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::TargetMachine::LLVMSetTargetMachineFastISel(Into::<LLVMTargetMachineRef>::into(T_), Into::<std::ffi::c_int>::into(Enable_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetTargetMachineGlobalISel<T0_, T1_>(T_:  T0_, Enable_:  T1_)
  where
     T0_: Into<LLVMTargetMachineRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::TargetMachine::LLVMSetTargetMachineGlobalISel(Into::<LLVMTargetMachineRef>::into(T_), Into::<std::ffi::c_int>::into(Enable_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetTargetMachineGlobalISelAbort<T0_, T1_>(T_:  T0_, Mode_:  T1_)
  where
     T0_: Into<LLVMTargetMachineRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::TargetMachine::LLVMSetTargetMachineGlobalISelAbort(Into::<LLVMTargetMachineRef>::into(T_), Into::<std::ffi::c_uint>::into(Mode_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetTargetMachineMachineOutliner<T0_, T1_>(T_:  T0_, Enable_:  T1_)
  where
     T0_: Into<LLVMTargetMachineRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::TargetMachine::LLVMSetTargetMachineMachineOutliner(Into::<LLVMTargetMachineRef>::into(T_), Into::<std::ffi::c_int>::into(Enable_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMTargetMachineEmitToFile<T0_, T1_, T2_, T3_, T4_>(T_:  T0_, M_:  T1_, Filename_:  T2_, codegen_:  T3_, ErrorMessage_:  T4_)-> Tret_
  where
     T0_: Into<LLVMTargetMachineRef>,  T1_: Into<LLVMModuleRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMTargetMachineEmitToFile(Into::<LLVMTargetMachineRef>::into(T_), Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Filename_), Into::<std::ffi::c_uint>::into(codegen_), Into::<*mut *mut std::ffi::c_char>::into(ErrorMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMTargetMachineEmitToMemoryBuffer<T0_, T1_, T2_, T3_, T4_>(T_:  T0_, M_:  T1_, codegen_:  T2_, ErrorMessage_:  T3_, OutMemBuf_:  T4_)-> Tret_
  where
     T0_: Into<LLVMTargetMachineRef>,  T1_: Into<LLVMModuleRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<*mut *mut std::ffi::c_char>,  T4_: Into<*mut LLVMMemoryBufferRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMTargetMachineEmitToMemoryBuffer(Into::<LLVMTargetMachineRef>::into(T_), Into::<LLVMModuleRef>::into(M_), Into::<std::ffi::c_uint>::into(codegen_), Into::<*mut *mut std::ffi::c_char>::into(ErrorMessage_), Into::<*mut LLVMMemoryBufferRef>::into(OutMemBuf_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMGetDefaultTargetTriple()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetDefaultTargetTriple()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMNormalizeTargetTriple<T0_>(triple_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMNormalizeTargetTriple(Into::<*const std::ffi::c_char>::into(triple_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMGetHostCPUName()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetHostCPUName()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMGetHostCPUFeatures()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::TargetMachine::LLVMGetHostCPUFeatures()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddAnalysisPasses<T0_, T1_>(T_:  T0_, PM_:  T1_)
  where
     T0_: Into<LLVMTargetMachineRef>,  T1_: Into<LLVMPassManagerRef>
  {
    unsafe {
      crate::TargetMachine::LLVMAddAnalysisPasses(Into::<LLVMTargetMachineRef>::into(T_), Into::<LLVMPassManagerRef>::into(PM_))
    }
  }
}

