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

#[link(name = "LLVM")]
extern {

  pub fn LLVMWriteBitcodeToFile(M: LLVMModuleRef, Path: *const std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMWriteBitcodeToFD(M: LLVMModuleRef, FD: std::ffi::c_int, ShouldClose: std::ffi::c_int, Unbuffered: std::ffi::c_int) -> std::ffi::c_int;

  pub fn LLVMWriteBitcodeToFileHandle(M: LLVMModuleRef, Handle: std::ffi::c_int) -> std::ffi::c_int;

  pub fn LLVMWriteBitcodeToMemoryBuffer(M: LLVMModuleRef) -> LLVMMemoryBufferRef;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMWriteBitcodeToFile<T0_, T1_>(M_:  T0_, Path_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitWriter::LLVMWriteBitcodeToFile(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Path_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMWriteBitcodeToFD<T0_, T1_, T2_, T3_>(M_:  T0_, FD_:  T1_, ShouldClose_:  T2_, Unbuffered_:  T3_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<std::ffi::c_int>,  T2_: Into<std::ffi::c_int>,  T3_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitWriter::LLVMWriteBitcodeToFD(Into::<LLVMModuleRef>::into(M_), Into::<std::ffi::c_int>::into(FD_), Into::<std::ffi::c_int>::into(ShouldClose_), Into::<std::ffi::c_int>::into(Unbuffered_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMWriteBitcodeToFileHandle<T0_, T1_>(M_:  T0_, Handle_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitWriter::LLVMWriteBitcodeToFileHandle(Into::<LLVMModuleRef>::into(M_), Into::<std::ffi::c_int>::into(Handle_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMemoryBufferRef> {
  pub unsafe fn LLVMWriteBitcodeToMemoryBuffer<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitWriter::LLVMWriteBitcodeToMemoryBuffer(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

