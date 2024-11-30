// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::ExternC::*;
pub type LLVMErrorRef = *mut u8;
pub type LLVMErrorTypeId = *const std::ffi::c_void;

#[link(name = "LLVM")]
extern {

  pub fn LLVMGetErrorTypeId(Err: LLVMErrorRef) -> LLVMErrorTypeId;

  pub fn LLVMConsumeError(Err: LLVMErrorRef) -> ();

  pub fn LLVMCantFail(Err: LLVMErrorRef) -> ();

  pub fn LLVMGetErrorMessage(Err: LLVMErrorRef) -> *mut std::ffi::c_char;

  pub fn LLVMDisposeErrorMessage(ErrMsg: *mut std::ffi::c_char) -> ();

  pub fn LLVMGetStringErrorTypeId() -> LLVMErrorTypeId;

  pub fn LLVMCreateStringError(ErrMsg: *const std::ffi::c_char) -> LLVMErrorRef;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorTypeId> {
  pub unsafe fn LLVMGetErrorTypeId<T0_>(Err_:  T0_)-> Tret_
  where
     T0_: Into<LLVMErrorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Error::LLVMGetErrorTypeId(Into::<LLVMErrorRef>::into(Err_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMConsumeError<T0_>(Err_:  T0_)
  where
     T0_: Into<LLVMErrorRef>
  {
    unsafe {
      crate::Error::LLVMConsumeError(Into::<LLVMErrorRef>::into(Err_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMCantFail<T0_>(Err_:  T0_)
  where
     T0_: Into<LLVMErrorRef>
  {
    unsafe {
      crate::Error::LLVMCantFail(Into::<LLVMErrorRef>::into(Err_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMGetErrorMessage<T0_>(Err_:  T0_)-> Tret_
  where
     T0_: Into<LLVMErrorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Error::LLVMGetErrorMessage(Into::<LLVMErrorRef>::into(Err_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeErrorMessage<T0_>(ErrMsg_:  T0_)
  where
     T0_: Into<*mut std::ffi::c_char>
  {
    unsafe {
      crate::Error::LLVMDisposeErrorMessage(Into::<*mut std::ffi::c_char>::into(ErrMsg_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorTypeId> {
  pub unsafe fn LLVMGetStringErrorTypeId()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Error::LLVMGetStringErrorTypeId()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMErrorRef> {
  pub unsafe fn LLVMCreateStringError<T0_>(ErrMsg_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Error::LLVMCreateStringError(Into::<*const std::ffi::c_char>::into(ErrMsg_))
      }
    )
  }
}

