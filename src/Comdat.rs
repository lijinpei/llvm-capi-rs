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

pub const LLVMAnyComdatSelectionKind: std::ffi::c_uint = 0;
pub const LLVMExactMatchComdatSelectionKind: std::ffi::c_uint = 1;
pub const LLVMLargestComdatSelectionKind: std::ffi::c_uint = 2;
pub const LLVMNoDeduplicateComdatSelectionKind: std::ffi::c_uint = 3;
pub const LLVMSameSizeComdatSelectionKind: std::ffi::c_uint = 4;
pub type LLVMComdatSelectionKind = std::ffi::c_uint;

#[link(name = "LLVM")]
extern {

  pub fn LLVMGetOrInsertComdat(M: LLVMModuleRef, Name: *const std::ffi::c_char) -> LLVMComdatRef;

  pub fn LLVMGetComdat(V: LLVMValueRef) -> LLVMComdatRef;

  pub fn LLVMSetComdat(V: LLVMValueRef, C: LLVMComdatRef) -> ();

  pub fn LLVMGetComdatSelectionKind(C: LLVMComdatRef) -> std::ffi::c_uint;

  pub fn LLVMSetComdatSelectionKind(C: LLVMComdatRef, Kind: std::ffi::c_uint) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMComdatRef> {
  pub unsafe fn LLVMGetOrInsertComdat<T0_, T1_>(M_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Comdat::LLVMGetOrInsertComdat(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMComdatRef> {
  pub unsafe fn LLVMGetComdat<T0_>(V_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Comdat::LLVMGetComdat(Into::<LLVMValueRef>::into(V_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetComdat<T0_, T1_>(V_:  T0_, C_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMComdatRef>
  {
    unsafe {
      crate::Comdat::LLVMSetComdat(Into::<LLVMValueRef>::into(V_), Into::<LLVMComdatRef>::into(C_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetComdatSelectionKind<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMComdatRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Comdat::LLVMGetComdatSelectionKind(Into::<LLVMComdatRef>::into(C_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetComdatSelectionKind<T0_, T1_>(C_:  T0_, Kind_:  T1_)
  where
     T0_: Into<LLVMComdatRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Comdat::LLVMSetComdatSelectionKind(Into::<LLVMComdatRef>::into(C_), Into::<std::ffi::c_uint>::into(Kind_))
    }
  }
}

