// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::ExternC::*;
pub type lto_bool_t = u8;

pub const LTO_SYMBOL_ALIGNMENT_MASK: std::ffi::c_uint = 31;
pub const LTO_SYMBOL_PERMISSIONS_MASK: std::ffi::c_uint = 224;
pub const LTO_SYMBOL_PERMISSIONS_CODE: std::ffi::c_uint = 160;
pub const LTO_SYMBOL_PERMISSIONS_DATA: std::ffi::c_uint = 192;
pub const LTO_SYMBOL_PERMISSIONS_RODATA: std::ffi::c_uint = 128;
pub const LTO_SYMBOL_DEFINITION_MASK: std::ffi::c_uint = 1792;
pub const LTO_SYMBOL_DEFINITION_REGULAR: std::ffi::c_uint = 256;
pub const LTO_SYMBOL_DEFINITION_TENTATIVE: std::ffi::c_uint = 512;
pub const LTO_SYMBOL_DEFINITION_WEAK: std::ffi::c_uint = 768;
pub const LTO_SYMBOL_DEFINITION_UNDEFINED: std::ffi::c_uint = 1024;
pub const LTO_SYMBOL_DEFINITION_WEAKUNDEF: std::ffi::c_uint = 1280;
pub const LTO_SYMBOL_SCOPE_MASK: std::ffi::c_uint = 14336;
pub const LTO_SYMBOL_SCOPE_INTERNAL: std::ffi::c_uint = 2048;
pub const LTO_SYMBOL_SCOPE_HIDDEN: std::ffi::c_uint = 4096;
pub const LTO_SYMBOL_SCOPE_PROTECTED: std::ffi::c_uint = 8192;
pub const LTO_SYMBOL_SCOPE_DEFAULT: std::ffi::c_uint = 6144;
pub const LTO_SYMBOL_SCOPE_DEFAULT_CAN_BE_HIDDEN: std::ffi::c_uint = 10240;
pub const LTO_SYMBOL_COMDAT: std::ffi::c_uint = 16384;
pub const LTO_SYMBOL_ALIAS: std::ffi::c_uint = 32768;
pub type lto_symbol_attributes = std::ffi::c_uint;

pub const LTO_DEBUG_MODEL_NONE: std::ffi::c_uint = 0;
pub const LTO_DEBUG_MODEL_DWARF: std::ffi::c_uint = 1;
pub type lto_debug_model = std::ffi::c_uint;

pub const LTO_CODEGEN_PIC_MODEL_STATIC: std::ffi::c_uint = 0;
pub const LTO_CODEGEN_PIC_MODEL_DYNAMIC: std::ffi::c_uint = 1;
pub const LTO_CODEGEN_PIC_MODEL_DYNAMIC_NO_PIC: std::ffi::c_uint = 2;
pub const LTO_CODEGEN_PIC_MODEL_DEFAULT: std::ffi::c_uint = 3;
pub type lto_codegen_model = std::ffi::c_uint;
pub type lto_module_t = *mut u8;
pub type lto_code_gen_t = *mut u8;
pub type thinlto_code_gen_t = *mut u8;

pub const LTO_DS_ERROR: std::ffi::c_uint = 0;
pub const LTO_DS_WARNING: std::ffi::c_uint = 1;
pub const LTO_DS_REMARK: std::ffi::c_uint = 3;
pub const LTO_DS_NOTE: std::ffi::c_uint = 2;
pub type lto_codegen_diagnostic_severity_t = std::ffi::c_uint;
pub type lto_diagnostic_handler_t = *mut extern fn (std::ffi::c_uint, *const std::ffi::c_char, *mut std::ffi::c_void) -> ();
pub type lto_input_t = *mut u8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct__anon_0 {
  pub Buffer: *const std::ffi::c_char,
  pub Size: std::ffi::c_ulong,
}
pub type LTOObjectBuffer = Struct__anon_0;

#[link(name = "LLVM")]
extern {

  pub fn lto_get_version() -> *const std::ffi::c_char;

  pub fn lto_get_error_message() -> *const std::ffi::c_char;

  pub fn lto_module_is_object_file(path: *const std::ffi::c_char) -> u8;

  pub fn lto_module_is_object_file_for_target(path: *const std::ffi::c_char, target_triple_prefix: *const std::ffi::c_char) -> u8;

  pub fn lto_module_has_objc_category(mem: *const std::ffi::c_void, length: std::ffi::c_ulong) -> u8;

  pub fn lto_module_is_object_file_in_memory(mem: *const std::ffi::c_void, length: std::ffi::c_ulong) -> u8;

  pub fn lto_module_is_object_file_in_memory_for_target(mem: *const std::ffi::c_void, length: std::ffi::c_ulong, target_triple_prefix: *const std::ffi::c_char) -> u8;

  pub fn lto_module_create(path: *const std::ffi::c_char) -> lto_module_t;

  pub fn lto_module_create_from_memory(mem: *const std::ffi::c_void, length: std::ffi::c_ulong) -> lto_module_t;

  pub fn lto_module_create_from_memory_with_path(mem: *const std::ffi::c_void, length: std::ffi::c_ulong, path: *const std::ffi::c_char) -> lto_module_t;

  pub fn lto_module_create_in_local_context(mem: *const std::ffi::c_void, length: std::ffi::c_ulong, path: *const std::ffi::c_char) -> lto_module_t;

  pub fn lto_module_create_in_codegen_context(mem: *const std::ffi::c_void, length: std::ffi::c_ulong, path: *const std::ffi::c_char, cg: lto_code_gen_t) -> lto_module_t;

  pub fn lto_module_create_from_fd(fd: std::ffi::c_int, path: *const std::ffi::c_char, file_size: std::ffi::c_ulong) -> lto_module_t;

  pub fn lto_module_create_from_fd_at_offset(fd: std::ffi::c_int, path: *const std::ffi::c_char, file_size: std::ffi::c_ulong, map_size: std::ffi::c_ulong, offset: std::ffi::c_long) -> lto_module_t;

  pub fn lto_module_dispose(r#mod: lto_module_t) -> ();

  pub fn lto_module_get_target_triple(r#mod: lto_module_t) -> *const std::ffi::c_char;

  pub fn lto_module_set_target_triple(r#mod: lto_module_t, triple: *const std::ffi::c_char) -> ();

  pub fn lto_module_get_num_symbols(r#mod: lto_module_t) -> std::ffi::c_uint;

  pub fn lto_module_get_symbol_name(r#mod: lto_module_t, index: std::ffi::c_uint) -> *const std::ffi::c_char;

  pub fn lto_module_get_symbol_attribute(r#mod: lto_module_t, index: std::ffi::c_uint) -> std::ffi::c_uint;

  pub fn lto_module_get_linkeropts(r#mod: lto_module_t) -> *const std::ffi::c_char;

  pub fn lto_module_get_macho_cputype(r#mod: lto_module_t, out_cputype: *mut std::ffi::c_uint, out_cpusubtype: *mut std::ffi::c_uint) -> u8;

  pub fn lto_module_has_ctor_dtor(r#mod: lto_module_t) -> u8;

  pub fn lto_codegen_set_diagnostic_handler(_: lto_code_gen_t, _: lto_diagnostic_handler_t, _: *mut std::ffi::c_void) -> ();

  pub fn lto_codegen_create() -> lto_code_gen_t;

  pub fn lto_codegen_create_in_local_context() -> lto_code_gen_t;

  pub fn lto_codegen_dispose(_: lto_code_gen_t) -> ();

  pub fn lto_codegen_add_module(cg: lto_code_gen_t, r#mod: lto_module_t) -> u8;

  pub fn lto_codegen_set_module(cg: lto_code_gen_t, r#mod: lto_module_t) -> ();

  pub fn lto_codegen_set_debug_model(cg: lto_code_gen_t, _: std::ffi::c_uint) -> u8;

  pub fn lto_codegen_set_pic_model(cg: lto_code_gen_t, _: std::ffi::c_uint) -> u8;

  pub fn lto_codegen_set_cpu(cg: lto_code_gen_t, cpu: *const std::ffi::c_char) -> ();

  pub fn lto_codegen_set_assembler_path(cg: lto_code_gen_t, path: *const std::ffi::c_char) -> ();

  pub fn lto_codegen_set_assembler_args(cg: lto_code_gen_t, args: *mut *const std::ffi::c_char, nargs: std::ffi::c_int) -> ();

  pub fn lto_codegen_add_must_preserve_symbol(cg: lto_code_gen_t, symbol: *const std::ffi::c_char) -> ();

  pub fn lto_codegen_write_merged_modules(cg: lto_code_gen_t, path: *const std::ffi::c_char) -> u8;

  pub fn lto_codegen_compile(cg: lto_code_gen_t, length: *mut std::ffi::c_ulong) -> *const std::ffi::c_void;

  pub fn lto_codegen_compile_to_file(cg: lto_code_gen_t, name: *mut *const std::ffi::c_char) -> u8;

  pub fn lto_codegen_optimize(cg: lto_code_gen_t) -> u8;

  pub fn lto_codegen_compile_optimized(cg: lto_code_gen_t, length: *mut std::ffi::c_ulong) -> *const std::ffi::c_void;

  pub fn lto_api_version() -> std::ffi::c_uint;

  pub fn lto_set_debug_options(options: *const *const std::ffi::c_char, number: std::ffi::c_int) -> ();

  pub fn lto_codegen_debug_options(cg: lto_code_gen_t, _: *const std::ffi::c_char) -> ();

  pub fn lto_codegen_debug_options_array(cg: lto_code_gen_t, _: *const *const std::ffi::c_char, number: std::ffi::c_int) -> ();

  pub fn lto_initialize_disassembler() -> ();

  pub fn lto_codegen_set_should_internalize(cg: lto_code_gen_t, ShouldInternalize: u8) -> ();

  pub fn lto_codegen_set_should_embed_uselists(cg: lto_code_gen_t, ShouldEmbedUselists: u8) -> ();

  pub fn lto_input_create(buffer: *const std::ffi::c_void, buffer_size: std::ffi::c_ulong, path: *const std::ffi::c_char) -> lto_input_t;

  pub fn lto_input_dispose(input: lto_input_t) -> ();

  pub fn lto_input_get_num_dependent_libraries(input: lto_input_t) -> std::ffi::c_uint;

  pub fn lto_input_get_dependent_library(input: lto_input_t, index: std::ffi::c_ulong, size: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn lto_runtime_lib_symbols_list(size: *mut std::ffi::c_ulong) -> *const *const std::ffi::c_char;

  pub fn thinlto_create_codegen() -> thinlto_code_gen_t;

  pub fn thinlto_codegen_dispose(cg: thinlto_code_gen_t) -> ();

  pub fn thinlto_codegen_add_module(cg: thinlto_code_gen_t, identifier: *const std::ffi::c_char, data: *const std::ffi::c_char, length: std::ffi::c_int) -> ();

  pub fn thinlto_codegen_process(cg: thinlto_code_gen_t) -> ();

  pub fn thinlto_module_get_num_objects(cg: thinlto_code_gen_t) -> std::ffi::c_uint;

  pub fn thinlto_module_get_object(cg: thinlto_code_gen_t, index: std::ffi::c_uint) -> LTOObjectBuffer;

  pub fn thinlto_module_get_num_object_files(cg: thinlto_code_gen_t) -> std::ffi::c_uint;

  pub fn thinlto_module_get_object_file(cg: thinlto_code_gen_t, index: std::ffi::c_uint) -> *const std::ffi::c_char;

  pub fn thinlto_codegen_set_pic_model(cg: thinlto_code_gen_t, _: std::ffi::c_uint) -> u8;

  pub fn thinlto_codegen_set_savetemps_dir(cg: thinlto_code_gen_t, save_temps_dir: *const std::ffi::c_char) -> ();

  pub fn thinlto_set_generated_objects_dir(cg: thinlto_code_gen_t, save_temps_dir: *const std::ffi::c_char) -> ();

  pub fn thinlto_codegen_set_cpu(cg: thinlto_code_gen_t, cpu: *const std::ffi::c_char) -> ();

  pub fn thinlto_codegen_disable_codegen(cg: thinlto_code_gen_t, disable: u8) -> ();

  pub fn thinlto_codegen_set_codegen_only(cg: thinlto_code_gen_t, codegen_only: u8) -> ();

  pub fn thinlto_debug_options(options: *const *const std::ffi::c_char, number: std::ffi::c_int) -> ();

  pub fn lto_module_is_thinlto(r#mod: lto_module_t) -> u8;

  pub fn thinlto_codegen_add_must_preserve_symbol(cg: thinlto_code_gen_t, name: *const std::ffi::c_char, length: std::ffi::c_int) -> ();

  pub fn thinlto_codegen_add_cross_referenced_symbol(cg: thinlto_code_gen_t, name: *const std::ffi::c_char, length: std::ffi::c_int) -> ();

  pub fn thinlto_codegen_set_cache_dir(cg: thinlto_code_gen_t, cache_dir: *const std::ffi::c_char) -> ();

  pub fn thinlto_codegen_set_cache_pruning_interval(cg: thinlto_code_gen_t, interval: std::ffi::c_int) -> ();

  pub fn thinlto_codegen_set_final_cache_size_relative_to_available_space(cg: thinlto_code_gen_t, percentage: std::ffi::c_uint) -> ();

  pub fn thinlto_codegen_set_cache_entry_expiration(cg: thinlto_code_gen_t, expiration: std::ffi::c_uint) -> ();

  pub fn thinlto_codegen_set_cache_size_bytes(cg: thinlto_code_gen_t, max_size_bytes: std::ffi::c_uint) -> ();

  pub fn thinlto_codegen_set_cache_size_megabytes(cg: thinlto_code_gen_t, max_size_megabytes: std::ffi::c_uint) -> ();

  pub fn thinlto_codegen_set_cache_size_files(cg: thinlto_code_gen_t, max_size_files: std::ffi::c_uint) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn lto_get_version()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_get_version()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn lto_get_error_message()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_get_error_message()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_module_is_object_file<T0_>(path_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_is_object_file(Into::<*const std::ffi::c_char>::into(path_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_module_is_object_file_for_target<T0_, T1_>(path_:  T0_, target_triple_prefix_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_is_object_file_for_target(Into::<*const std::ffi::c_char>::into(path_), Into::<*const std::ffi::c_char>::into(target_triple_prefix_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_module_has_objc_category<T0_, T1_>(mem_:  T0_, length_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_has_objc_category(Into::<*const std::ffi::c_void>::into(mem_), Into::<std::ffi::c_ulong>::into(length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_module_is_object_file_in_memory<T0_, T1_>(mem_:  T0_, length_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_is_object_file_in_memory(Into::<*const std::ffi::c_void>::into(mem_), Into::<std::ffi::c_ulong>::into(length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_module_is_object_file_in_memory_for_target<T0_, T1_, T2_>(mem_:  T0_, length_:  T1_, target_triple_prefix_:  T2_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_is_object_file_in_memory_for_target(Into::<*const std::ffi::c_void>::into(mem_), Into::<std::ffi::c_ulong>::into(length_), Into::<*const std::ffi::c_char>::into(target_triple_prefix_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<lto_module_t> {
  pub unsafe fn lto_module_create<T0_>(path_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_create(Into::<*const std::ffi::c_char>::into(path_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<lto_module_t> {
  pub unsafe fn lto_module_create_from_memory<T0_, T1_>(mem_:  T0_, length_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_create_from_memory(Into::<*const std::ffi::c_void>::into(mem_), Into::<std::ffi::c_ulong>::into(length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<lto_module_t> {
  pub unsafe fn lto_module_create_from_memory_with_path<T0_, T1_, T2_>(mem_:  T0_, length_:  T1_, path_:  T2_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_create_from_memory_with_path(Into::<*const std::ffi::c_void>::into(mem_), Into::<std::ffi::c_ulong>::into(length_), Into::<*const std::ffi::c_char>::into(path_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<lto_module_t> {
  pub unsafe fn lto_module_create_in_local_context<T0_, T1_, T2_>(mem_:  T0_, length_:  T1_, path_:  T2_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_create_in_local_context(Into::<*const std::ffi::c_void>::into(mem_), Into::<std::ffi::c_ulong>::into(length_), Into::<*const std::ffi::c_char>::into(path_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<lto_module_t> {
  pub unsafe fn lto_module_create_in_codegen_context<T0_, T1_, T2_, T3_>(mem_:  T0_, length_:  T1_, path_:  T2_, cg_:  T3_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<lto_code_gen_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_create_in_codegen_context(Into::<*const std::ffi::c_void>::into(mem_), Into::<std::ffi::c_ulong>::into(length_), Into::<*const std::ffi::c_char>::into(path_), Into::<lto_code_gen_t>::into(cg_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<lto_module_t> {
  pub unsafe fn lto_module_create_from_fd<T0_, T1_, T2_>(fd_:  T0_, path_:  T1_, file_size_:  T2_)-> Tret_
  where
     T0_: Into<std::ffi::c_int>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_create_from_fd(Into::<std::ffi::c_int>::into(fd_), Into::<*const std::ffi::c_char>::into(path_), Into::<std::ffi::c_ulong>::into(file_size_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<lto_module_t> {
  pub unsafe fn lto_module_create_from_fd_at_offset<T0_, T1_, T2_, T3_, T4_>(fd_:  T0_, path_:  T1_, file_size_:  T2_, map_size_:  T3_, offset_:  T4_)-> Tret_
  where
     T0_: Into<std::ffi::c_int>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_create_from_fd_at_offset(Into::<std::ffi::c_int>::into(fd_), Into::<*const std::ffi::c_char>::into(path_), Into::<std::ffi::c_ulong>::into(file_size_), Into::<std::ffi::c_ulong>::into(map_size_), Into::<std::ffi::c_long>::into(offset_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_module_dispose<T0_>(mod_:  T0_)
  where
     T0_: Into<lto_module_t>
  {
    unsafe {
      crate::lto::lto_module_dispose(Into::<lto_module_t>::into(mod_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn lto_module_get_target_triple<T0_>(mod_:  T0_)-> Tret_
  where
     T0_: Into<lto_module_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_get_target_triple(Into::<lto_module_t>::into(mod_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_module_set_target_triple<T0_, T1_>(mod_:  T0_, triple_:  T1_)
  where
     T0_: Into<lto_module_t>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::lto::lto_module_set_target_triple(Into::<lto_module_t>::into(mod_), Into::<*const std::ffi::c_char>::into(triple_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn lto_module_get_num_symbols<T0_>(mod_:  T0_)-> Tret_
  where
     T0_: Into<lto_module_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_get_num_symbols(Into::<lto_module_t>::into(mod_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn lto_module_get_symbol_name<T0_, T1_>(mod_:  T0_, index_:  T1_)-> Tret_
  where
     T0_: Into<lto_module_t>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_get_symbol_name(Into::<lto_module_t>::into(mod_), Into::<std::ffi::c_uint>::into(index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn lto_module_get_symbol_attribute<T0_, T1_>(mod_:  T0_, index_:  T1_)-> Tret_
  where
     T0_: Into<lto_module_t>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_get_symbol_attribute(Into::<lto_module_t>::into(mod_), Into::<std::ffi::c_uint>::into(index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn lto_module_get_linkeropts<T0_>(mod_:  T0_)-> Tret_
  where
     T0_: Into<lto_module_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_get_linkeropts(Into::<lto_module_t>::into(mod_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_module_get_macho_cputype<T0_, T1_, T2_>(mod_:  T0_, out_cputype_:  T1_, out_cpusubtype_:  T2_)-> Tret_
  where
     T0_: Into<lto_module_t>,  T1_: Into<*mut std::ffi::c_uint>,  T2_: Into<*mut std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_get_macho_cputype(Into::<lto_module_t>::into(mod_), Into::<*mut std::ffi::c_uint>::into(out_cputype_), Into::<*mut std::ffi::c_uint>::into(out_cpusubtype_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_module_has_ctor_dtor<T0_>(mod_:  T0_)-> Tret_
  where
     T0_: Into<lto_module_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_has_ctor_dtor(Into::<lto_module_t>::into(mod_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_set_diagnostic_handler<T0_, T1_, T2_>(arg0_:  T0_, arg1_:  T1_, arg2_:  T2_)
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<lto_diagnostic_handler_t>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::lto::lto_codegen_set_diagnostic_handler(Into::<lto_code_gen_t>::into(arg0_), Into::<lto_diagnostic_handler_t>::into(arg1_), Into::<*mut std::ffi::c_void>::into(arg2_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<lto_code_gen_t> {
  pub unsafe fn lto_codegen_create()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_codegen_create()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<lto_code_gen_t> {
  pub unsafe fn lto_codegen_create_in_local_context()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_codegen_create_in_local_context()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_dispose<T0_>(arg0_:  T0_)
  where
     T0_: Into<lto_code_gen_t>
  {
    unsafe {
      crate::lto::lto_codegen_dispose(Into::<lto_code_gen_t>::into(arg0_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_codegen_add_module<T0_, T1_>(cg_:  T0_, mod_:  T1_)-> Tret_
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<lto_module_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_codegen_add_module(Into::<lto_code_gen_t>::into(cg_), Into::<lto_module_t>::into(mod_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_set_module<T0_, T1_>(cg_:  T0_, mod_:  T1_)
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<lto_module_t>
  {
    unsafe {
      crate::lto::lto_codegen_set_module(Into::<lto_code_gen_t>::into(cg_), Into::<lto_module_t>::into(mod_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_codegen_set_debug_model<T0_, T1_>(cg_:  T0_, arg1_:  T1_)-> Tret_
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_codegen_set_debug_model(Into::<lto_code_gen_t>::into(cg_), Into::<std::ffi::c_uint>::into(arg1_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_codegen_set_pic_model<T0_, T1_>(cg_:  T0_, arg1_:  T1_)-> Tret_
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_codegen_set_pic_model(Into::<lto_code_gen_t>::into(cg_), Into::<std::ffi::c_uint>::into(arg1_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_set_cpu<T0_, T1_>(cg_:  T0_, cpu_:  T1_)
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::lto::lto_codegen_set_cpu(Into::<lto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(cpu_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_set_assembler_path<T0_, T1_>(cg_:  T0_, path_:  T1_)
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::lto::lto_codegen_set_assembler_path(Into::<lto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(path_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_set_assembler_args<T0_, T1_, T2_>(cg_:  T0_, args_:  T1_, nargs_:  T2_)
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<*mut *const std::ffi::c_char>,  T2_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::lto::lto_codegen_set_assembler_args(Into::<lto_code_gen_t>::into(cg_), Into::<*mut *const std::ffi::c_char>::into(args_), Into::<std::ffi::c_int>::into(nargs_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_add_must_preserve_symbol<T0_, T1_>(cg_:  T0_, symbol_:  T1_)
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::lto::lto_codegen_add_must_preserve_symbol(Into::<lto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(symbol_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_codegen_write_merged_modules<T0_, T1_>(cg_:  T0_, path_:  T1_)-> Tret_
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_codegen_write_merged_modules(Into::<lto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(path_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_void> {
  pub unsafe fn lto_codegen_compile<T0_, T1_>(cg_:  T0_, length_:  T1_)-> Tret_
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_codegen_compile(Into::<lto_code_gen_t>::into(cg_), Into::<*mut std::ffi::c_ulong>::into(length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_codegen_compile_to_file<T0_, T1_>(cg_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<*mut *const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_codegen_compile_to_file(Into::<lto_code_gen_t>::into(cg_), Into::<*mut *const std::ffi::c_char>::into(name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_codegen_optimize<T0_>(cg_:  T0_)-> Tret_
  where
     T0_: Into<lto_code_gen_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_codegen_optimize(Into::<lto_code_gen_t>::into(cg_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_void> {
  pub unsafe fn lto_codegen_compile_optimized<T0_, T1_>(cg_:  T0_, length_:  T1_)-> Tret_
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_codegen_compile_optimized(Into::<lto_code_gen_t>::into(cg_), Into::<*mut std::ffi::c_ulong>::into(length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn lto_api_version()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_api_version()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_set_debug_options<T0_, T1_>(options_:  T0_, number_:  T1_)
  where
     T0_: Into<*const *const std::ffi::c_char>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::lto::lto_set_debug_options(Into::<*const *const std::ffi::c_char>::into(options_), Into::<std::ffi::c_int>::into(number_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_debug_options<T0_, T1_>(cg_:  T0_, arg1_:  T1_)
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::lto::lto_codegen_debug_options(Into::<lto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(arg1_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_debug_options_array<T0_, T1_, T2_>(cg_:  T0_, arg1_:  T1_, number_:  T2_)
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<*const *const std::ffi::c_char>,  T2_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::lto::lto_codegen_debug_options_array(Into::<lto_code_gen_t>::into(cg_), Into::<*const *const std::ffi::c_char>::into(arg1_), Into::<std::ffi::c_int>::into(number_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_set_should_internalize<T0_, T1_>(cg_:  T0_, ShouldInternalize_:  T1_)
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<u8>
  {
    unsafe {
      crate::lto::lto_codegen_set_should_internalize(Into::<lto_code_gen_t>::into(cg_), Into::<u8>::into(ShouldInternalize_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_codegen_set_should_embed_uselists<T0_, T1_>(cg_:  T0_, ShouldEmbedUselists_:  T1_)
  where
     T0_: Into<lto_code_gen_t>,  T1_: Into<u8>
  {
    unsafe {
      crate::lto::lto_codegen_set_should_embed_uselists(Into::<lto_code_gen_t>::into(cg_), Into::<u8>::into(ShouldEmbedUselists_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<lto_input_t> {
  pub unsafe fn lto_input_create<T0_, T1_, T2_>(buffer_:  T0_, buffer_size_:  T1_, path_:  T2_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_input_create(Into::<*const std::ffi::c_void>::into(buffer_), Into::<std::ffi::c_ulong>::into(buffer_size_), Into::<*const std::ffi::c_char>::into(path_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn lto_input_dispose<T0_>(input_:  T0_)
  where
     T0_: Into<lto_input_t>
  {
    unsafe {
      crate::lto::lto_input_dispose(Into::<lto_input_t>::into(input_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn lto_input_get_num_dependent_libraries<T0_>(input_:  T0_)-> Tret_
  where
     T0_: Into<lto_input_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_input_get_num_dependent_libraries(Into::<lto_input_t>::into(input_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn lto_input_get_dependent_library<T0_, T1_, T2_>(input_:  T0_, index_:  T1_, size_:  T2_)-> Tret_
  where
     T0_: Into<lto_input_t>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_input_get_dependent_library(Into::<lto_input_t>::into(input_), Into::<std::ffi::c_ulong>::into(index_), Into::<*mut std::ffi::c_ulong>::into(size_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const *const std::ffi::c_char> {
  pub unsafe fn lto_runtime_lib_symbols_list<T0_>(size_:  T0_)-> Tret_
  where
     T0_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_runtime_lib_symbols_list(Into::<*mut std::ffi::c_ulong>::into(size_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<thinlto_code_gen_t> {
  pub unsafe fn thinlto_create_codegen()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::thinlto_create_codegen()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_dispose<T0_>(cg_:  T0_)
  where
     T0_: Into<thinlto_code_gen_t>
  {
    unsafe {
      crate::lto::thinlto_codegen_dispose(Into::<thinlto_code_gen_t>::into(cg_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_add_module<T0_, T1_, T2_, T3_>(cg_:  T0_, identifier_:  T1_, data_:  T2_, length_:  T3_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::lto::thinlto_codegen_add_module(Into::<thinlto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(identifier_), Into::<*const std::ffi::c_char>::into(data_), Into::<std::ffi::c_int>::into(length_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_process<T0_>(cg_:  T0_)
  where
     T0_: Into<thinlto_code_gen_t>
  {
    unsafe {
      crate::lto::thinlto_codegen_process(Into::<thinlto_code_gen_t>::into(cg_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn thinlto_module_get_num_objects<T0_>(cg_:  T0_)-> Tret_
  where
     T0_: Into<thinlto_code_gen_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::thinlto_module_get_num_objects(Into::<thinlto_code_gen_t>::into(cg_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LTOObjectBuffer> {
  pub unsafe fn thinlto_module_get_object<T0_, T1_>(cg_:  T0_, index_:  T1_)-> Tret_
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::thinlto_module_get_object(Into::<thinlto_code_gen_t>::into(cg_), Into::<std::ffi::c_uint>::into(index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn thinlto_module_get_num_object_files<T0_>(cg_:  T0_)-> Tret_
  where
     T0_: Into<thinlto_code_gen_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::thinlto_module_get_num_object_files(Into::<thinlto_code_gen_t>::into(cg_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn thinlto_module_get_object_file<T0_, T1_>(cg_:  T0_, index_:  T1_)-> Tret_
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::thinlto_module_get_object_file(Into::<thinlto_code_gen_t>::into(cg_), Into::<std::ffi::c_uint>::into(index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn thinlto_codegen_set_pic_model<T0_, T1_>(cg_:  T0_, arg1_:  T1_)-> Tret_
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::thinlto_codegen_set_pic_model(Into::<thinlto_code_gen_t>::into(cg_), Into::<std::ffi::c_uint>::into(arg1_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_set_savetemps_dir<T0_, T1_>(cg_:  T0_, save_temps_dir_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::lto::thinlto_codegen_set_savetemps_dir(Into::<thinlto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(save_temps_dir_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_set_generated_objects_dir<T0_, T1_>(cg_:  T0_, save_temps_dir_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::lto::thinlto_set_generated_objects_dir(Into::<thinlto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(save_temps_dir_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_set_cpu<T0_, T1_>(cg_:  T0_, cpu_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::lto::thinlto_codegen_set_cpu(Into::<thinlto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(cpu_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_disable_codegen<T0_, T1_>(cg_:  T0_, disable_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<u8>
  {
    unsafe {
      crate::lto::thinlto_codegen_disable_codegen(Into::<thinlto_code_gen_t>::into(cg_), Into::<u8>::into(disable_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_set_codegen_only<T0_, T1_>(cg_:  T0_, codegen_only_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<u8>
  {
    unsafe {
      crate::lto::thinlto_codegen_set_codegen_only(Into::<thinlto_code_gen_t>::into(cg_), Into::<u8>::into(codegen_only_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_debug_options<T0_, T1_>(options_:  T0_, number_:  T1_)
  where
     T0_: Into<*const *const std::ffi::c_char>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::lto::thinlto_debug_options(Into::<*const *const std::ffi::c_char>::into(options_), Into::<std::ffi::c_int>::into(number_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn lto_module_is_thinlto<T0_>(mod_:  T0_)-> Tret_
  where
     T0_: Into<lto_module_t>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::lto::lto_module_is_thinlto(Into::<lto_module_t>::into(mod_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_add_must_preserve_symbol<T0_, T1_, T2_>(cg_:  T0_, name_:  T1_, length_:  T2_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::lto::thinlto_codegen_add_must_preserve_symbol(Into::<thinlto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(name_), Into::<std::ffi::c_int>::into(length_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_add_cross_referenced_symbol<T0_, T1_, T2_>(cg_:  T0_, name_:  T1_, length_:  T2_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::lto::thinlto_codegen_add_cross_referenced_symbol(Into::<thinlto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(name_), Into::<std::ffi::c_int>::into(length_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_set_cache_dir<T0_, T1_>(cg_:  T0_, cache_dir_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::lto::thinlto_codegen_set_cache_dir(Into::<thinlto_code_gen_t>::into(cg_), Into::<*const std::ffi::c_char>::into(cache_dir_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_set_cache_pruning_interval<T0_, T1_>(cg_:  T0_, interval_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::lto::thinlto_codegen_set_cache_pruning_interval(Into::<thinlto_code_gen_t>::into(cg_), Into::<std::ffi::c_int>::into(interval_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_set_final_cache_size_relative_to_available_space<T0_, T1_>(cg_:  T0_, percentage_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::lto::thinlto_codegen_set_final_cache_size_relative_to_available_space(Into::<thinlto_code_gen_t>::into(cg_), Into::<std::ffi::c_uint>::into(percentage_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_set_cache_entry_expiration<T0_, T1_>(cg_:  T0_, expiration_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::lto::thinlto_codegen_set_cache_entry_expiration(Into::<thinlto_code_gen_t>::into(cg_), Into::<std::ffi::c_uint>::into(expiration_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_set_cache_size_bytes<T0_, T1_>(cg_:  T0_, max_size_bytes_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::lto::thinlto_codegen_set_cache_size_bytes(Into::<thinlto_code_gen_t>::into(cg_), Into::<std::ffi::c_uint>::into(max_size_bytes_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_set_cache_size_megabytes<T0_, T1_>(cg_:  T0_, max_size_megabytes_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::lto::thinlto_codegen_set_cache_size_megabytes(Into::<thinlto_code_gen_t>::into(cg_), Into::<std::ffi::c_uint>::into(max_size_megabytes_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn thinlto_codegen_set_cache_size_files<T0_, T1_>(cg_:  T0_, max_size_files_:  T1_)
  where
     T0_: Into<thinlto_code_gen_t>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::lto::thinlto_codegen_set_cache_size_files(Into::<thinlto_code_gen_t>::into(cg_), Into::<std::ffi::c_uint>::into(max_size_files_))
    }
  }
}

