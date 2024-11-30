// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::DataTypes::*;
use crate::ExternC::*;
use crate::Types::*;
use crate::DataTypes::*;
use crate::ExternC::*;

#[link(name = "LLVM")]
extern {

  pub fn LLVMLoadLibraryPermanently(Filename: *const std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMParseCommandLineOptions(argc: std::ffi::c_int, argv: *const *const std::ffi::c_char, Overview: *const std::ffi::c_char) -> ();

  pub fn LLVMSearchForAddressOfSymbol(symbolName: *const std::ffi::c_char) -> *mut std::ffi::c_void;

  pub fn LLVMAddSymbol(symbolName: *const std::ffi::c_char, symbolValue: *mut std::ffi::c_void) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMLoadLibraryPermanently<T0_>(Filename_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::LLVMLoadLibraryPermanently(Into::<*const std::ffi::c_char>::into(Filename_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMParseCommandLineOptions<T0_, T1_, T2_>(argc_:  T0_, argv_:  T1_, Overview_:  T2_)
  where
     T0_: Into<std::ffi::c_int>,  T1_: Into<*const *const std::ffi::c_char>,  T2_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Support::LLVMParseCommandLineOptions(Into::<std::ffi::c_int>::into(argc_), Into::<*const *const std::ffi::c_char>::into(argv_), Into::<*const std::ffi::c_char>::into(Overview_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_void> {
  pub unsafe fn LLVMSearchForAddressOfSymbol<T0_>(symbolName_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::LLVMSearchForAddressOfSymbol(Into::<*const std::ffi::c_char>::into(symbolName_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddSymbol<T0_, T1_>(symbolName_:  T0_, symbolValue_:  T1_)
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::Support::LLVMAddSymbol(Into::<*const std::ffi::c_char>::into(symbolName_), Into::<*mut std::ffi::c_void>::into(symbolValue_))
    }
  }
}

