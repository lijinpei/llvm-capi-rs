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

pub const LLVMLinkerDestroySource: std::ffi::c_uint = 0;
pub const LLVMLinkerPreserveSource_Removed: std::ffi::c_uint = 1;
pub type LLVMLinkerMode = std::ffi::c_uint;

#[link(name = "LLVM")]
extern {

  pub fn LLVMLinkModules2(Dest: LLVMModuleRef, Src: LLVMModuleRef) -> std::ffi::c_int;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMLinkModules2<T0_, T1_>(Dest_:  T0_, Src_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Linker::LLVMLinkModules2(Into::<LLVMModuleRef>::into(Dest_), Into::<LLVMModuleRef>::into(Src_))
      }
    )
  }
}

