// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::ExternC::*;
pub type LLVMFatalErrorHandler = *mut extern fn (*const std::ffi::c_char) -> ();

#[link(name = "LLVM")]
extern {

  pub fn LLVMInstallFatalErrorHandler(Handler: LLVMFatalErrorHandler) -> ();

  pub fn LLVMResetFatalErrorHandler() -> ();

  pub fn LLVMEnablePrettyStackTrace() -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl FFIVoid_ {
  pub unsafe fn LLVMInstallFatalErrorHandler<T0_>(Handler_:  T0_)
  where
     T0_: Into<LLVMFatalErrorHandler>
  {
    unsafe {
      crate::ErrorHandling::LLVMInstallFatalErrorHandler(Into::<LLVMFatalErrorHandler>::into(Handler_))
    }
  }
}

