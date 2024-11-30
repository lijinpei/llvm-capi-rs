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

pub const LLVMAbortProcessAction: std::ffi::c_uint = 0;
pub const LLVMPrintMessageAction: std::ffi::c_uint = 1;
pub const LLVMReturnStatusAction: std::ffi::c_uint = 2;
pub type LLVMVerifierFailureAction = std::ffi::c_uint;

#[link(name = "LLVM")]
extern {

  pub fn LLVMVerifyModule(M: LLVMModuleRef, Action: std::ffi::c_uint, OutMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMVerifyFunction(Fn: LLVMValueRef, Action: std::ffi::c_uint) -> std::ffi::c_int;

  pub fn LLVMViewFunctionCFG(Fn: LLVMValueRef) -> ();

  pub fn LLVMViewFunctionCFGOnly(Fn: LLVMValueRef) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMVerifyModule<T0_, T1_, T2_>(M_:  T0_, Action_:  T1_, OutMessage_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Analysis::LLVMVerifyModule(Into::<LLVMModuleRef>::into(M_), Into::<std::ffi::c_uint>::into(Action_), Into::<*mut *mut std::ffi::c_char>::into(OutMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMVerifyFunction<T0_, T1_>(Fn_:  T0_, Action_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Analysis::LLVMVerifyFunction(Into::<LLVMValueRef>::into(Fn_), Into::<std::ffi::c_uint>::into(Action_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMViewFunctionCFG<T0_>(Fn_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Analysis::LLVMViewFunctionCFG(Into::<LLVMValueRef>::into(Fn_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMViewFunctionCFGOnly<T0_>(Fn_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Analysis::LLVMViewFunctionCFGOnly(Into::<LLVMValueRef>::into(Fn_))
    }
  }
}

