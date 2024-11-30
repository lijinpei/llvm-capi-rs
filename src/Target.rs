// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::ExternC::*;
use crate::Types::*;
use crate::DataTypes::*;
use crate::ExternC::*;

pub const LLVMBigEndian: std::ffi::c_uint = 0;
pub const LLVMLittleEndian: std::ffi::c_uint = 1;
pub type EnumLLVMByteOrdering = std::ffi::c_uint;
pub type LLVMTargetDataRef = *mut u8;
pub type LLVMTargetLibraryInfoRef = *mut u8;

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

  pub fn LLVMInitializeAllTargetInfos() -> ();

  pub fn LLVMInitializeAllTargets() -> ();

  pub fn LLVMInitializeAllTargetMCs() -> ();

  pub fn LLVMInitializeAllAsmPrinters() -> ();

  pub fn LLVMInitializeAllAsmParsers() -> ();

  pub fn LLVMInitializeAllDisassemblers() -> ();

  pub fn LLVMInitializeNativeTarget() -> std::ffi::c_int;

  pub fn LLVMInitializeNativeAsmParser() -> std::ffi::c_int;

  pub fn LLVMInitializeNativeAsmPrinter() -> std::ffi::c_int;

  pub fn LLVMInitializeNativeDisassembler() -> std::ffi::c_int;

  pub fn LLVMGetModuleDataLayout(M: LLVMModuleRef) -> LLVMTargetDataRef;

  pub fn LLVMSetModuleDataLayout(M: LLVMModuleRef, DL: LLVMTargetDataRef) -> ();

  pub fn LLVMCreateTargetData(StringRep: *const std::ffi::c_char) -> LLVMTargetDataRef;

  pub fn LLVMDisposeTargetData(TD: LLVMTargetDataRef) -> ();

  pub fn LLVMAddTargetLibraryInfo(TLI: LLVMTargetLibraryInfoRef, PM: LLVMPassManagerRef) -> ();

  pub fn LLVMCopyStringRepOfTargetData(TD: LLVMTargetDataRef) -> *mut std::ffi::c_char;

  pub fn LLVMByteOrder(TD: LLVMTargetDataRef) -> EnumLLVMByteOrdering;

  pub fn LLVMPointerSize(TD: LLVMTargetDataRef) -> std::ffi::c_uint;

  pub fn LLVMPointerSizeForAS(TD: LLVMTargetDataRef, AS: std::ffi::c_uint) -> std::ffi::c_uint;

  pub fn LLVMIntPtrType(TD: LLVMTargetDataRef) -> LLVMTypeRef;

  pub fn LLVMIntPtrTypeForAS(TD: LLVMTargetDataRef, AS: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMIntPtrTypeInContext(C: LLVMContextRef, TD: LLVMTargetDataRef) -> LLVMTypeRef;

  pub fn LLVMIntPtrTypeForASInContext(C: LLVMContextRef, TD: LLVMTargetDataRef, AS: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMSizeOfTypeInBits(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> std::ffi::c_ulonglong;

  pub fn LLVMStoreSizeOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> std::ffi::c_ulonglong;

  pub fn LLVMABISizeOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> std::ffi::c_ulonglong;

  pub fn LLVMABIAlignmentOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMCallFrameAlignmentOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMPreferredAlignmentOfType(TD: LLVMTargetDataRef, Ty: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMPreferredAlignmentOfGlobal(TD: LLVMTargetDataRef, GlobalVar: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMElementAtOffset(TD: LLVMTargetDataRef, StructTy: LLVMTypeRef, Offset: std::ffi::c_ulonglong) -> std::ffi::c_uint;

  pub fn LLVMOffsetOfElement(TD: LLVMTargetDataRef, StructTy: LLVMTypeRef, Element: std::ffi::c_uint) -> std::ffi::c_ulonglong;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMInitializeNativeTarget()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMInitializeNativeTarget()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMInitializeNativeAsmParser()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMInitializeNativeAsmParser()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMInitializeNativeAsmPrinter()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMInitializeNativeAsmPrinter()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMInitializeNativeDisassembler()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMInitializeNativeDisassembler()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetDataRef> {
  pub unsafe fn LLVMGetModuleDataLayout<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMGetModuleDataLayout(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetModuleDataLayout<T0_, T1_>(M_:  T0_, DL_:  T1_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<LLVMTargetDataRef>
  {
    unsafe {
      crate::Target::LLVMSetModuleDataLayout(Into::<LLVMModuleRef>::into(M_), Into::<LLVMTargetDataRef>::into(DL_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTargetDataRef> {
  pub unsafe fn LLVMCreateTargetData<T0_>(StringRep_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMCreateTargetData(Into::<*const std::ffi::c_char>::into(StringRep_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeTargetData<T0_>(TD_:  T0_)
  where
     T0_: Into<LLVMTargetDataRef>
  {
    unsafe {
      crate::Target::LLVMDisposeTargetData(Into::<LLVMTargetDataRef>::into(TD_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddTargetLibraryInfo<T0_, T1_>(TLI_:  T0_, PM_:  T1_)
  where
     T0_: Into<LLVMTargetLibraryInfoRef>,  T1_: Into<LLVMPassManagerRef>
  {
    unsafe {
      crate::Target::LLVMAddTargetLibraryInfo(Into::<LLVMTargetLibraryInfoRef>::into(TLI_), Into::<LLVMPassManagerRef>::into(PM_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMCopyStringRepOfTargetData<T0_>(TD_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMCopyStringRepOfTargetData(Into::<LLVMTargetDataRef>::into(TD_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<EnumLLVMByteOrdering> {
  pub unsafe fn LLVMByteOrder<T0_>(TD_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMByteOrder(Into::<LLVMTargetDataRef>::into(TD_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMPointerSize<T0_>(TD_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMPointerSize(Into::<LLVMTargetDataRef>::into(TD_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMPointerSizeForAS<T0_, T1_>(TD_:  T0_, AS_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMPointerSizeForAS(Into::<LLVMTargetDataRef>::into(TD_), Into::<std::ffi::c_uint>::into(AS_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMIntPtrType<T0_>(TD_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMIntPtrType(Into::<LLVMTargetDataRef>::into(TD_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMIntPtrTypeForAS<T0_, T1_>(TD_:  T0_, AS_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMIntPtrTypeForAS(Into::<LLVMTargetDataRef>::into(TD_), Into::<std::ffi::c_uint>::into(AS_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMIntPtrTypeInContext<T0_, T1_>(C_:  T0_, TD_:  T1_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMTargetDataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMIntPtrTypeInContext(Into::<LLVMContextRef>::into(C_), Into::<LLVMTargetDataRef>::into(TD_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMIntPtrTypeForASInContext<T0_, T1_, T2_>(C_:  T0_, TD_:  T1_, AS_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMTargetDataRef>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMIntPtrTypeForASInContext(Into::<LLVMContextRef>::into(C_), Into::<LLVMTargetDataRef>::into(TD_), Into::<std::ffi::c_uint>::into(AS_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulonglong> {
  pub unsafe fn LLVMSizeOfTypeInBits<T0_, T1_>(TD_:  T0_, Ty_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMSizeOfTypeInBits(Into::<LLVMTargetDataRef>::into(TD_), Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulonglong> {
  pub unsafe fn LLVMStoreSizeOfType<T0_, T1_>(TD_:  T0_, Ty_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMStoreSizeOfType(Into::<LLVMTargetDataRef>::into(TD_), Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulonglong> {
  pub unsafe fn LLVMABISizeOfType<T0_, T1_>(TD_:  T0_, Ty_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMABISizeOfType(Into::<LLVMTargetDataRef>::into(TD_), Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMABIAlignmentOfType<T0_, T1_>(TD_:  T0_, Ty_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMABIAlignmentOfType(Into::<LLVMTargetDataRef>::into(TD_), Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMCallFrameAlignmentOfType<T0_, T1_>(TD_:  T0_, Ty_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMCallFrameAlignmentOfType(Into::<LLVMTargetDataRef>::into(TD_), Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMPreferredAlignmentOfType<T0_, T1_>(TD_:  T0_, Ty_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMPreferredAlignmentOfType(Into::<LLVMTargetDataRef>::into(TD_), Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMPreferredAlignmentOfGlobal<T0_, T1_>(TD_:  T0_, GlobalVar_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMPreferredAlignmentOfGlobal(Into::<LLVMTargetDataRef>::into(TD_), Into::<LLVMValueRef>::into(GlobalVar_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMElementAtOffset<T0_, T1_, T2_>(TD_:  T0_, StructTy_:  T1_, Offset_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<std::ffi::c_ulonglong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMElementAtOffset(Into::<LLVMTargetDataRef>::into(TD_), Into::<LLVMTypeRef>::into(StructTy_), Into::<std::ffi::c_ulonglong>::into(Offset_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulonglong> {
  pub unsafe fn LLVMOffsetOfElement<T0_, T1_, T2_>(TD_:  T0_, StructTy_:  T1_, Element_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTargetDataRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target::LLVMOffsetOfElement(Into::<LLVMTargetDataRef>::into(TD_), Into::<LLVMTypeRef>::into(StructTy_), Into::<std::ffi::c_uint>::into(Element_))
      }
    )
  }
}

