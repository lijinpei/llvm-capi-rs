// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::DataTypes::*;
pub type LLVMDisasmContextRef = *mut std::ffi::c_void;
pub type LLVMOpInfoCallback = *mut extern fn (*mut std::ffi::c_void, std::ffi::c_ulong, std::ffi::c_ulong, std::ffi::c_ulong, std::ffi::c_ulong, std::ffi::c_int, *mut std::ffi::c_void) -> std::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructLLVMOpInfoSymbol1 {
  pub Present: std::ffi::c_ulong,
  pub Name: *const std::ffi::c_char,
  pub Value: std::ffi::c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructLLVMOpInfo1 {
  pub AddSymbol: StructLLVMOpInfoSymbol1,
  pub SubtractSymbol: StructLLVMOpInfoSymbol1,
  pub Value: std::ffi::c_ulong,
  pub VariantKind: std::ffi::c_ulong,
}
pub type LLVMSymbolLookupCallback = *mut extern fn (*mut std::ffi::c_void, std::ffi::c_ulong, *mut std::ffi::c_ulong, std::ffi::c_ulong, *mut *const std::ffi::c_char) -> *const std::ffi::c_char;

#[link(name = "LLVM")]
extern {

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

