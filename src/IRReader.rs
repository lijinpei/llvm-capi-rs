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

  pub fn LLVMParseIRInContext(ContextRef: LLVMContextRef, MemBuf: LLVMMemoryBufferRef, OutM: *mut LLVMModuleRef, OutMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMParseIRInContext<T0_, T1_, T2_, T3_>(ContextRef_:  T0_, MemBuf_:  T1_, OutM_:  T2_, OutMessage_:  T3_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMMemoryBufferRef>,  T2_: Into<*mut LLVMModuleRef>,  T3_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IRReader::LLVMParseIRInContext(Into::<LLVMContextRef>::into(ContextRef_), Into::<LLVMMemoryBufferRef>::into(MemBuf_), Into::<*mut LLVMModuleRef>::into(OutM_), Into::<*mut *mut std::ffi::c_char>::into(OutMessage_))
      }
    )
  }
}

