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

  pub fn LLVMParseBitcode(MemBuf: LLVMMemoryBufferRef, OutModule: *mut LLVMModuleRef, OutMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMParseBitcode2(MemBuf: LLVMMemoryBufferRef, OutModule: *mut LLVMModuleRef) -> std::ffi::c_int;

  pub fn LLVMParseBitcodeInContext(ContextRef: LLVMContextRef, MemBuf: LLVMMemoryBufferRef, OutModule: *mut LLVMModuleRef, OutMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMParseBitcodeInContext2(ContextRef: LLVMContextRef, MemBuf: LLVMMemoryBufferRef, OutModule: *mut LLVMModuleRef) -> std::ffi::c_int;

  pub fn LLVMGetBitcodeModuleInContext(ContextRef: LLVMContextRef, MemBuf: LLVMMemoryBufferRef, OutM: *mut LLVMModuleRef, OutMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMGetBitcodeModuleInContext2(ContextRef: LLVMContextRef, MemBuf: LLVMMemoryBufferRef, OutM: *mut LLVMModuleRef) -> std::ffi::c_int;

  pub fn LLVMGetBitcodeModule(MemBuf: LLVMMemoryBufferRef, OutM: *mut LLVMModuleRef, OutMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMGetBitcodeModule2(MemBuf: LLVMMemoryBufferRef, OutM: *mut LLVMModuleRef) -> std::ffi::c_int;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMParseBitcode<T0_, T1_, T2_>(MemBuf_:  T0_, OutModule_:  T1_, OutMessage_:  T2_)-> Tret_
  where
     T0_: Into<LLVMMemoryBufferRef>,  T1_: Into<*mut LLVMModuleRef>,  T2_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitReader::LLVMParseBitcode(Into::<LLVMMemoryBufferRef>::into(MemBuf_), Into::<*mut LLVMModuleRef>::into(OutModule_), Into::<*mut *mut std::ffi::c_char>::into(OutMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMParseBitcode2<T0_, T1_>(MemBuf_:  T0_, OutModule_:  T1_)-> Tret_
  where
     T0_: Into<LLVMMemoryBufferRef>,  T1_: Into<*mut LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitReader::LLVMParseBitcode2(Into::<LLVMMemoryBufferRef>::into(MemBuf_), Into::<*mut LLVMModuleRef>::into(OutModule_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMParseBitcodeInContext<T0_, T1_, T2_, T3_>(ContextRef_:  T0_, MemBuf_:  T1_, OutModule_:  T2_, OutMessage_:  T3_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMMemoryBufferRef>,  T2_: Into<*mut LLVMModuleRef>,  T3_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitReader::LLVMParseBitcodeInContext(Into::<LLVMContextRef>::into(ContextRef_), Into::<LLVMMemoryBufferRef>::into(MemBuf_), Into::<*mut LLVMModuleRef>::into(OutModule_), Into::<*mut *mut std::ffi::c_char>::into(OutMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMParseBitcodeInContext2<T0_, T1_, T2_>(ContextRef_:  T0_, MemBuf_:  T1_, OutModule_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMMemoryBufferRef>,  T2_: Into<*mut LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitReader::LLVMParseBitcodeInContext2(Into::<LLVMContextRef>::into(ContextRef_), Into::<LLVMMemoryBufferRef>::into(MemBuf_), Into::<*mut LLVMModuleRef>::into(OutModule_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetBitcodeModuleInContext<T0_, T1_, T2_, T3_>(ContextRef_:  T0_, MemBuf_:  T1_, OutM_:  T2_, OutMessage_:  T3_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMMemoryBufferRef>,  T2_: Into<*mut LLVMModuleRef>,  T3_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitReader::LLVMGetBitcodeModuleInContext(Into::<LLVMContextRef>::into(ContextRef_), Into::<LLVMMemoryBufferRef>::into(MemBuf_), Into::<*mut LLVMModuleRef>::into(OutM_), Into::<*mut *mut std::ffi::c_char>::into(OutMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetBitcodeModuleInContext2<T0_, T1_, T2_>(ContextRef_:  T0_, MemBuf_:  T1_, OutM_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMMemoryBufferRef>,  T2_: Into<*mut LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitReader::LLVMGetBitcodeModuleInContext2(Into::<LLVMContextRef>::into(ContextRef_), Into::<LLVMMemoryBufferRef>::into(MemBuf_), Into::<*mut LLVMModuleRef>::into(OutM_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetBitcodeModule<T0_, T1_, T2_>(MemBuf_:  T0_, OutM_:  T1_, OutMessage_:  T2_)-> Tret_
  where
     T0_: Into<LLVMMemoryBufferRef>,  T1_: Into<*mut LLVMModuleRef>,  T2_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitReader::LLVMGetBitcodeModule(Into::<LLVMMemoryBufferRef>::into(MemBuf_), Into::<*mut LLVMModuleRef>::into(OutM_), Into::<*mut *mut std::ffi::c_char>::into(OutMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetBitcodeModule2<T0_, T1_>(MemBuf_:  T0_, OutM_:  T1_)-> Tret_
  where
     T0_: Into<LLVMMemoryBufferRef>,  T1_: Into<*mut LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BitReader::LLVMGetBitcodeModule2(Into::<LLVMMemoryBufferRef>::into(MemBuf_), Into::<*mut LLVMModuleRef>::into(OutM_))
      }
    )
  }
}

