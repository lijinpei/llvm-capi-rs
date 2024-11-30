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

pub const LLVMRemarkTypeUnknown: std::ffi::c_uint = 0;
pub const LLVMRemarkTypePassed: std::ffi::c_uint = 1;
pub const LLVMRemarkTypeMissed: std::ffi::c_uint = 2;
pub const LLVMRemarkTypeAnalysis: std::ffi::c_uint = 3;
pub const LLVMRemarkTypeAnalysisFPCommute: std::ffi::c_uint = 4;
pub const LLVMRemarkTypeAnalysisAliasing: std::ffi::c_uint = 5;
pub const LLVMRemarkTypeFailure: std::ffi::c_uint = 6;
pub type EnumLLVMRemarkType = std::ffi::c_uint;
pub type LLVMRemarkStringRef = *mut u8;
pub type LLVMRemarkDebugLocRef = *mut u8;
pub type LLVMRemarkArgRef = *mut u8;
pub type LLVMRemarkEntryRef = *mut u8;
pub type LLVMRemarkParserRef = *mut u8;

#[link(name = "LLVM")]
extern {

  pub fn LLVMRemarkStringGetData(String: LLVMRemarkStringRef) -> *const std::ffi::c_char;

  pub fn LLVMRemarkStringGetLen(String: LLVMRemarkStringRef) -> std::ffi::c_uint;

  pub fn LLVMRemarkDebugLocGetSourceFilePath(DL: LLVMRemarkDebugLocRef) -> LLVMRemarkStringRef;

  pub fn LLVMRemarkDebugLocGetSourceLine(DL: LLVMRemarkDebugLocRef) -> std::ffi::c_uint;

  pub fn LLVMRemarkDebugLocGetSourceColumn(DL: LLVMRemarkDebugLocRef) -> std::ffi::c_uint;

  pub fn LLVMRemarkArgGetKey(Arg: LLVMRemarkArgRef) -> LLVMRemarkStringRef;

  pub fn LLVMRemarkArgGetValue(Arg: LLVMRemarkArgRef) -> LLVMRemarkStringRef;

  pub fn LLVMRemarkArgGetDebugLoc(Arg: LLVMRemarkArgRef) -> LLVMRemarkDebugLocRef;

  pub fn LLVMRemarkEntryDispose(Remark: LLVMRemarkEntryRef) -> ();

  pub fn LLVMRemarkEntryGetType(Remark: LLVMRemarkEntryRef) -> EnumLLVMRemarkType;

  pub fn LLVMRemarkEntryGetPassName(Remark: LLVMRemarkEntryRef) -> LLVMRemarkStringRef;

  pub fn LLVMRemarkEntryGetRemarkName(Remark: LLVMRemarkEntryRef) -> LLVMRemarkStringRef;

  pub fn LLVMRemarkEntryGetFunctionName(Remark: LLVMRemarkEntryRef) -> LLVMRemarkStringRef;

  pub fn LLVMRemarkEntryGetDebugLoc(Remark: LLVMRemarkEntryRef) -> LLVMRemarkDebugLocRef;

  pub fn LLVMRemarkEntryGetHotness(Remark: LLVMRemarkEntryRef) -> std::ffi::c_ulong;

  pub fn LLVMRemarkEntryGetNumArgs(Remark: LLVMRemarkEntryRef) -> std::ffi::c_uint;

  pub fn LLVMRemarkEntryGetFirstArg(Remark: LLVMRemarkEntryRef) -> LLVMRemarkArgRef;

  pub fn LLVMRemarkEntryGetNextArg(It: LLVMRemarkArgRef, Remark: LLVMRemarkEntryRef) -> LLVMRemarkArgRef;

  pub fn LLVMRemarkParserCreateYAML(Buf: *const std::ffi::c_void, Size: std::ffi::c_ulong) -> LLVMRemarkParserRef;

  pub fn LLVMRemarkParserCreateBitstream(Buf: *const std::ffi::c_void, Size: std::ffi::c_ulong) -> LLVMRemarkParserRef;

  pub fn LLVMRemarkParserGetNext(Parser: LLVMRemarkParserRef) -> LLVMRemarkEntryRef;

  pub fn LLVMRemarkParserHasError(Parser: LLVMRemarkParserRef) -> std::ffi::c_int;

  pub fn LLVMRemarkParserGetErrorMessage(Parser: LLVMRemarkParserRef) -> *const std::ffi::c_char;

  pub fn LLVMRemarkParserDispose(Parser: LLVMRemarkParserRef) -> ();

  pub fn LLVMRemarkVersion() -> std::ffi::c_uint;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMRemarkStringGetData<T0_>(String_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkStringGetData(Into::<LLVMRemarkStringRef>::into(String_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMRemarkStringGetLen<T0_>(String_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkStringGetLen(Into::<LLVMRemarkStringRef>::into(String_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkStringRef> {
  pub unsafe fn LLVMRemarkDebugLocGetSourceFilePath<T0_>(DL_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkDebugLocRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkDebugLocGetSourceFilePath(Into::<LLVMRemarkDebugLocRef>::into(DL_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMRemarkDebugLocGetSourceLine<T0_>(DL_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkDebugLocRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkDebugLocGetSourceLine(Into::<LLVMRemarkDebugLocRef>::into(DL_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMRemarkDebugLocGetSourceColumn<T0_>(DL_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkDebugLocRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkDebugLocGetSourceColumn(Into::<LLVMRemarkDebugLocRef>::into(DL_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkStringRef> {
  pub unsafe fn LLVMRemarkArgGetKey<T0_>(Arg_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkArgRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkArgGetKey(Into::<LLVMRemarkArgRef>::into(Arg_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkStringRef> {
  pub unsafe fn LLVMRemarkArgGetValue<T0_>(Arg_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkArgRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkArgGetValue(Into::<LLVMRemarkArgRef>::into(Arg_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkDebugLocRef> {
  pub unsafe fn LLVMRemarkArgGetDebugLoc<T0_>(Arg_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkArgRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkArgGetDebugLoc(Into::<LLVMRemarkArgRef>::into(Arg_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMRemarkEntryDispose<T0_>(Remark_:  T0_)
  where
     T0_: Into<LLVMRemarkEntryRef>
  {
    unsafe {
      crate::Remarks::LLVMRemarkEntryDispose(Into::<LLVMRemarkEntryRef>::into(Remark_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<EnumLLVMRemarkType> {
  pub unsafe fn LLVMRemarkEntryGetType<T0_>(Remark_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkEntryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkEntryGetType(Into::<LLVMRemarkEntryRef>::into(Remark_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkStringRef> {
  pub unsafe fn LLVMRemarkEntryGetPassName<T0_>(Remark_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkEntryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkEntryGetPassName(Into::<LLVMRemarkEntryRef>::into(Remark_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkStringRef> {
  pub unsafe fn LLVMRemarkEntryGetRemarkName<T0_>(Remark_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkEntryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkEntryGetRemarkName(Into::<LLVMRemarkEntryRef>::into(Remark_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkStringRef> {
  pub unsafe fn LLVMRemarkEntryGetFunctionName<T0_>(Remark_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkEntryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkEntryGetFunctionName(Into::<LLVMRemarkEntryRef>::into(Remark_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkDebugLocRef> {
  pub unsafe fn LLVMRemarkEntryGetDebugLoc<T0_>(Remark_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkEntryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkEntryGetDebugLoc(Into::<LLVMRemarkEntryRef>::into(Remark_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMRemarkEntryGetHotness<T0_>(Remark_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkEntryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkEntryGetHotness(Into::<LLVMRemarkEntryRef>::into(Remark_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMRemarkEntryGetNumArgs<T0_>(Remark_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkEntryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkEntryGetNumArgs(Into::<LLVMRemarkEntryRef>::into(Remark_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkArgRef> {
  pub unsafe fn LLVMRemarkEntryGetFirstArg<T0_>(Remark_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkEntryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkEntryGetFirstArg(Into::<LLVMRemarkEntryRef>::into(Remark_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkArgRef> {
  pub unsafe fn LLVMRemarkEntryGetNextArg<T0_, T1_>(It_:  T0_, Remark_:  T1_)-> Tret_
  where
     T0_: Into<LLVMRemarkArgRef>,  T1_: Into<LLVMRemarkEntryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkEntryGetNextArg(Into::<LLVMRemarkArgRef>::into(It_), Into::<LLVMRemarkEntryRef>::into(Remark_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkParserRef> {
  pub unsafe fn LLVMRemarkParserCreateYAML<T0_, T1_>(Buf_:  T0_, Size_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkParserCreateYAML(Into::<*const std::ffi::c_void>::into(Buf_), Into::<std::ffi::c_ulong>::into(Size_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkParserRef> {
  pub unsafe fn LLVMRemarkParserCreateBitstream<T0_, T1_>(Buf_:  T0_, Size_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkParserCreateBitstream(Into::<*const std::ffi::c_void>::into(Buf_), Into::<std::ffi::c_ulong>::into(Size_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRemarkEntryRef> {
  pub unsafe fn LLVMRemarkParserGetNext<T0_>(Parser_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkParserRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkParserGetNext(Into::<LLVMRemarkParserRef>::into(Parser_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMRemarkParserHasError<T0_>(Parser_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkParserRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkParserHasError(Into::<LLVMRemarkParserRef>::into(Parser_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMRemarkParserGetErrorMessage<T0_>(Parser_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRemarkParserRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkParserGetErrorMessage(Into::<LLVMRemarkParserRef>::into(Parser_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMRemarkParserDispose<T0_>(Parser_:  T0_)
  where
     T0_: Into<LLVMRemarkParserRef>
  {
    unsafe {
      crate::Remarks::LLVMRemarkParserDispose(Into::<LLVMRemarkParserRef>::into(Parser_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMRemarkVersion()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Remarks::LLVMRemarkVersion()
      }
    )
  }
}

