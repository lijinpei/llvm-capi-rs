// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::DisassemblerTypes::*;
use crate::DataTypes::*;
use crate::ExternC::*;

#[link(name = "LLVM")]
extern {

  pub fn LLVMCreateDisasm(TripleName: *const std::ffi::c_char, DisInfo: *mut std::ffi::c_void, TagType: std::ffi::c_int, GetOpInfo: LLVMOpInfoCallback, SymbolLookUp: LLVMSymbolLookupCallback) -> LLVMDisasmContextRef;

  pub fn LLVMCreateDisasmCPU(Triple: *const std::ffi::c_char, CPU: *const std::ffi::c_char, DisInfo: *mut std::ffi::c_void, TagType: std::ffi::c_int, GetOpInfo: LLVMOpInfoCallback, SymbolLookUp: LLVMSymbolLookupCallback) -> LLVMDisasmContextRef;

  pub fn LLVMCreateDisasmCPUFeatures(Triple: *const std::ffi::c_char, CPU: *const std::ffi::c_char, Features: *const std::ffi::c_char, DisInfo: *mut std::ffi::c_void, TagType: std::ffi::c_int, GetOpInfo: LLVMOpInfoCallback, SymbolLookUp: LLVMSymbolLookupCallback) -> LLVMDisasmContextRef;

  pub fn LLVMSetDisasmOptions(DC: LLVMDisasmContextRef, Options: std::ffi::c_ulong) -> std::ffi::c_int;

  pub fn LLVMDisasmDispose(DC: LLVMDisasmContextRef) -> ();

  pub fn LLVMDisasmInstruction(DC: LLVMDisasmContextRef, Bytes: *mut std::ffi::c_uchar, BytesSize: std::ffi::c_ulong, PC: std::ffi::c_ulong, OutString: *mut std::ffi::c_char, OutStringSize: std::ffi::c_ulong) -> std::ffi::c_ulong;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDisasmContextRef> {
  pub unsafe fn LLVMCreateDisasm<T0_, T1_, T2_, T3_, T4_>(TripleName_:  T0_, DisInfo_:  T1_, TagType_:  T2_, GetOpInfo_:  T3_, SymbolLookUp_:  T4_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<*mut std::ffi::c_void>,  T2_: Into<std::ffi::c_int>,  T3_: Into<LLVMOpInfoCallback>,  T4_: Into<LLVMSymbolLookupCallback>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Disassembler::LLVMCreateDisasm(Into::<*const std::ffi::c_char>::into(TripleName_), Into::<*mut std::ffi::c_void>::into(DisInfo_), Into::<std::ffi::c_int>::into(TagType_), Into::<LLVMOpInfoCallback>::into(GetOpInfo_), Into::<LLVMSymbolLookupCallback>::into(SymbolLookUp_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDisasmContextRef> {
  pub unsafe fn LLVMCreateDisasmCPU<T0_, T1_, T2_, T3_, T4_, T5_>(Triple_:  T0_, CPU_:  T1_, DisInfo_:  T2_, TagType_:  T3_, GetOpInfo_:  T4_, SymbolLookUp_:  T5_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*mut std::ffi::c_void>,  T3_: Into<std::ffi::c_int>,  T4_: Into<LLVMOpInfoCallback>,  T5_: Into<LLVMSymbolLookupCallback>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Disassembler::LLVMCreateDisasmCPU(Into::<*const std::ffi::c_char>::into(Triple_), Into::<*const std::ffi::c_char>::into(CPU_), Into::<*mut std::ffi::c_void>::into(DisInfo_), Into::<std::ffi::c_int>::into(TagType_), Into::<LLVMOpInfoCallback>::into(GetOpInfo_), Into::<LLVMSymbolLookupCallback>::into(SymbolLookUp_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDisasmContextRef> {
  pub unsafe fn LLVMCreateDisasmCPUFeatures<T0_, T1_, T2_, T3_, T4_, T5_, T6_>(Triple_:  T0_, CPU_:  T1_, Features_:  T2_, DisInfo_:  T3_, TagType_:  T4_, GetOpInfo_:  T5_, SymbolLookUp_:  T6_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<*mut std::ffi::c_void>,  T4_: Into<std::ffi::c_int>,  T5_: Into<LLVMOpInfoCallback>,  T6_: Into<LLVMSymbolLookupCallback>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Disassembler::LLVMCreateDisasmCPUFeatures(Into::<*const std::ffi::c_char>::into(Triple_), Into::<*const std::ffi::c_char>::into(CPU_), Into::<*const std::ffi::c_char>::into(Features_), Into::<*mut std::ffi::c_void>::into(DisInfo_), Into::<std::ffi::c_int>::into(TagType_), Into::<LLVMOpInfoCallback>::into(GetOpInfo_), Into::<LLVMSymbolLookupCallback>::into(SymbolLookUp_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMSetDisasmOptions<T0_, T1_>(DC_:  T0_, Options_:  T1_)-> Tret_
  where
     T0_: Into<LLVMDisasmContextRef>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Disassembler::LLVMSetDisasmOptions(Into::<LLVMDisasmContextRef>::into(DC_), Into::<std::ffi::c_ulong>::into(Options_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisasmDispose<T0_>(DC_:  T0_)
  where
     T0_: Into<LLVMDisasmContextRef>
  {
    unsafe {
      crate::Disassembler::LLVMDisasmDispose(Into::<LLVMDisasmContextRef>::into(DC_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMDisasmInstruction<T0_, T1_, T2_, T3_, T4_, T5_>(DC_:  T0_, Bytes_:  T1_, BytesSize_:  T2_, PC_:  T3_, OutString_:  T4_, OutStringSize_:  T5_)-> Tret_
  where
     T0_: Into<LLVMDisasmContextRef>,  T1_: Into<*mut std::ffi::c_uchar>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<*mut std::ffi::c_char>,  T5_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Disassembler::LLVMDisasmInstruction(Into::<LLVMDisasmContextRef>::into(DC_), Into::<*mut std::ffi::c_uchar>::into(Bytes_), Into::<std::ffi::c_ulong>::into(BytesSize_), Into::<std::ffi::c_ulong>::into(PC_), Into::<*mut std::ffi::c_char>::into(OutString_), Into::<std::ffi::c_ulong>::into(OutStringSize_))
      }
    )
  }
}

