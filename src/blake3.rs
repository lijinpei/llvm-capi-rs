// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_0 {
  pub cv: [std::ffi::c_uint; 8],
  pub chunk_counter: std::ffi::c_ulong,
  pub buf: [std::ffi::c_uchar; 64],
  pub buf_len: std::ffi::c_uchar,
  pub blocks_compressed: std::ffi::c_uchar,
  pub flags: std::ffi::c_uchar,
}
pub type llvm_blake3_chunk_state = Struct__anon_0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_1 {
  pub key: [std::ffi::c_uint; 8],
  pub chunk: llvm_blake3_chunk_state,
  pub cv_stack_len: std::ffi::c_uchar,
  pub cv_stack: [std::ffi::c_uchar; 1760],
}
pub type llvm_blake3_hasher = Struct__anon_1;

#[link(name = "LLVM")]
extern {

  pub fn llvm_blake3_version() -> *const std::ffi::c_char;

  pub fn llvm_blake3_hasher_init(self_: *mut llvm_blake3_hasher) -> ();

  pub fn llvm_blake3_hasher_init_keyed(self_: *mut llvm_blake3_hasher, key: *const std::ffi::c_uchar) -> ();

  pub fn llvm_blake3_hasher_init_derive_key(self_: *mut llvm_blake3_hasher, context: *const std::ffi::c_char) -> ();

  pub fn llvm_blake3_hasher_init_derive_key_raw(self_: *mut llvm_blake3_hasher, context: *const std::ffi::c_void, context_len: std::ffi::c_ulong) -> ();

  pub fn llvm_blake3_hasher_update(self_: *mut llvm_blake3_hasher, input: *const std::ffi::c_void, input_len: std::ffi::c_ulong) -> ();

  pub fn llvm_blake3_hasher_finalize(self_: *const llvm_blake3_hasher, out: *mut std::ffi::c_uchar, out_len: std::ffi::c_ulong) -> ();

  pub fn llvm_blake3_hasher_finalize_seek(self_: *const llvm_blake3_hasher, seek: std::ffi::c_ulong, out: *mut std::ffi::c_uchar, out_len: std::ffi::c_ulong) -> ();

  pub fn llvm_blake3_hasher_reset(self_: *mut llvm_blake3_hasher) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn llvm_blake3_version()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::blake3::llvm_blake3_version()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn llvm_blake3_hasher_init<T0_>(self_:  T0_)
  where
     T0_: Into<*mut llvm_blake3_hasher>
  {
    unsafe {
      crate::blake3::llvm_blake3_hasher_init(Into::<*mut llvm_blake3_hasher>::into(self_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn llvm_blake3_hasher_init_keyed<T0_, T1_>(self_:  T0_, key_:  T1_)
  where
     T0_: Into<*mut llvm_blake3_hasher>,  T1_: Into<*const std::ffi::c_uchar>
  {
    unsafe {
      crate::blake3::llvm_blake3_hasher_init_keyed(Into::<*mut llvm_blake3_hasher>::into(self_), Into::<*const std::ffi::c_uchar>::into(key_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn llvm_blake3_hasher_init_derive_key<T0_, T1_>(self_:  T0_, context_:  T1_)
  where
     T0_: Into<*mut llvm_blake3_hasher>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::blake3::llvm_blake3_hasher_init_derive_key(Into::<*mut llvm_blake3_hasher>::into(self_), Into::<*const std::ffi::c_char>::into(context_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn llvm_blake3_hasher_init_derive_key_raw<T0_, T1_, T2_>(self_:  T0_, context_:  T1_, context_len_:  T2_)
  where
     T0_: Into<*mut llvm_blake3_hasher>,  T1_: Into<*const std::ffi::c_void>,  T2_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::blake3::llvm_blake3_hasher_init_derive_key_raw(Into::<*mut llvm_blake3_hasher>::into(self_), Into::<*const std::ffi::c_void>::into(context_), Into::<std::ffi::c_ulong>::into(context_len_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn llvm_blake3_hasher_update<T0_, T1_, T2_>(self_:  T0_, input_:  T1_, input_len_:  T2_)
  where
     T0_: Into<*mut llvm_blake3_hasher>,  T1_: Into<*const std::ffi::c_void>,  T2_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::blake3::llvm_blake3_hasher_update(Into::<*mut llvm_blake3_hasher>::into(self_), Into::<*const std::ffi::c_void>::into(input_), Into::<std::ffi::c_ulong>::into(input_len_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn llvm_blake3_hasher_finalize<T0_, T1_, T2_>(self_:  T0_, out_:  T1_, out_len_:  T2_)
  where
     T0_: Into<*const llvm_blake3_hasher>,  T1_: Into<*mut std::ffi::c_uchar>,  T2_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::blake3::llvm_blake3_hasher_finalize(Into::<*const llvm_blake3_hasher>::into(self_), Into::<*mut std::ffi::c_uchar>::into(out_), Into::<std::ffi::c_ulong>::into(out_len_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn llvm_blake3_hasher_finalize_seek<T0_, T1_, T2_, T3_>(self_:  T0_, seek_:  T1_, out_:  T2_, out_len_:  T3_)
  where
     T0_: Into<*const llvm_blake3_hasher>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*mut std::ffi::c_uchar>,  T3_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::blake3::llvm_blake3_hasher_finalize_seek(Into::<*const llvm_blake3_hasher>::into(self_), Into::<std::ffi::c_ulong>::into(seek_), Into::<*mut std::ffi::c_uchar>::into(out_), Into::<std::ffi::c_ulong>::into(out_len_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn llvm_blake3_hasher_reset<T0_>(self_:  T0_)
  where
     T0_: Into<*mut llvm_blake3_hasher>
  {
    unsafe {
      crate::blake3::llvm_blake3_hasher_reset(Into::<*mut llvm_blake3_hasher>::into(self_))
    }
  }
}

