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
pub type LLVMSectionIteratorRef = *mut u8;
pub type LLVMSymbolIteratorRef = *mut u8;
pub type LLVMRelocationIteratorRef = *mut u8;

pub const LLVMBinaryTypeArchive: std::ffi::c_uint = 0;
pub const LLVMBinaryTypeMachOUniversalBinary: std::ffi::c_uint = 1;
pub const LLVMBinaryTypeCOFFImportFile: std::ffi::c_uint = 2;
pub const LLVMBinaryTypeIR: std::ffi::c_uint = 3;
pub const LLVMBinaryTypeWinRes: std::ffi::c_uint = 4;
pub const LLVMBinaryTypeCOFF: std::ffi::c_uint = 5;
pub const LLVMBinaryTypeELF32L: std::ffi::c_uint = 6;
pub const LLVMBinaryTypeELF32B: std::ffi::c_uint = 7;
pub const LLVMBinaryTypeELF64L: std::ffi::c_uint = 8;
pub const LLVMBinaryTypeELF64B: std::ffi::c_uint = 9;
pub const LLVMBinaryTypeMachO32L: std::ffi::c_uint = 10;
pub const LLVMBinaryTypeMachO32B: std::ffi::c_uint = 11;
pub const LLVMBinaryTypeMachO64L: std::ffi::c_uint = 12;
pub const LLVMBinaryTypeMachO64B: std::ffi::c_uint = 13;
pub const LLVMBinaryTypeWasm: std::ffi::c_uint = 14;
pub const LLVMBinaryTypeOffload: std::ffi::c_uint = 15;
pub type LLVMBinaryType = std::ffi::c_uint;
pub type LLVMObjectFileRef = *mut u8;

#[link(name = "LLVM")]
extern {

  pub fn LLVMCreateBinary(MemBuf: LLVMMemoryBufferRef, Context: LLVMContextRef, ErrorMessage: *mut *mut std::ffi::c_char) -> LLVMBinaryRef;

  pub fn LLVMDisposeBinary(BR: LLVMBinaryRef) -> ();

  pub fn LLVMBinaryCopyMemoryBuffer(BR: LLVMBinaryRef) -> LLVMMemoryBufferRef;

  pub fn LLVMBinaryGetType(BR: LLVMBinaryRef) -> std::ffi::c_uint;

  pub fn LLVMMachOUniversalBinaryCopyObjectForArch(BR: LLVMBinaryRef, Arch: *const std::ffi::c_char, ArchLen: std::ffi::c_ulong, ErrorMessage: *mut *mut std::ffi::c_char) -> LLVMBinaryRef;

  pub fn LLVMObjectFileCopySectionIterator(BR: LLVMBinaryRef) -> LLVMSectionIteratorRef;

  pub fn LLVMObjectFileIsSectionIteratorAtEnd(BR: LLVMBinaryRef, SI: LLVMSectionIteratorRef) -> std::ffi::c_int;

  pub fn LLVMObjectFileCopySymbolIterator(BR: LLVMBinaryRef) -> LLVMSymbolIteratorRef;

  pub fn LLVMObjectFileIsSymbolIteratorAtEnd(BR: LLVMBinaryRef, SI: LLVMSymbolIteratorRef) -> std::ffi::c_int;

  pub fn LLVMDisposeSectionIterator(SI: LLVMSectionIteratorRef) -> ();

  pub fn LLVMMoveToNextSection(SI: LLVMSectionIteratorRef) -> ();

  pub fn LLVMMoveToContainingSection(Sect: LLVMSectionIteratorRef, Sym: LLVMSymbolIteratorRef) -> ();

  pub fn LLVMDisposeSymbolIterator(SI: LLVMSymbolIteratorRef) -> ();

  pub fn LLVMMoveToNextSymbol(SI: LLVMSymbolIteratorRef) -> ();

  pub fn LLVMGetSectionName(SI: LLVMSectionIteratorRef) -> *const std::ffi::c_char;

  pub fn LLVMGetSectionSize(SI: LLVMSectionIteratorRef) -> std::ffi::c_ulong;

  pub fn LLVMGetSectionContents(SI: LLVMSectionIteratorRef) -> *const std::ffi::c_char;

  pub fn LLVMGetSectionAddress(SI: LLVMSectionIteratorRef) -> std::ffi::c_ulong;

  pub fn LLVMGetSectionContainsSymbol(SI: LLVMSectionIteratorRef, Sym: LLVMSymbolIteratorRef) -> std::ffi::c_int;

  pub fn LLVMGetRelocations(Section: LLVMSectionIteratorRef) -> LLVMRelocationIteratorRef;

  pub fn LLVMDisposeRelocationIterator(RI: LLVMRelocationIteratorRef) -> ();

  pub fn LLVMIsRelocationIteratorAtEnd(Section: LLVMSectionIteratorRef, RI: LLVMRelocationIteratorRef) -> std::ffi::c_int;

  pub fn LLVMMoveToNextRelocation(RI: LLVMRelocationIteratorRef) -> ();

  pub fn LLVMGetSymbolName(SI: LLVMSymbolIteratorRef) -> *const std::ffi::c_char;

  pub fn LLVMGetSymbolAddress(SI: LLVMSymbolIteratorRef) -> std::ffi::c_ulong;

  pub fn LLVMGetSymbolSize(SI: LLVMSymbolIteratorRef) -> std::ffi::c_ulong;

  pub fn LLVMGetRelocationOffset(RI: LLVMRelocationIteratorRef) -> std::ffi::c_ulong;

  pub fn LLVMGetRelocationSymbol(RI: LLVMRelocationIteratorRef) -> LLVMSymbolIteratorRef;

  pub fn LLVMGetRelocationType(RI: LLVMRelocationIteratorRef) -> std::ffi::c_ulong;

  pub fn LLVMGetRelocationTypeName(RI: LLVMRelocationIteratorRef) -> *const std::ffi::c_char;

  pub fn LLVMGetRelocationValueString(RI: LLVMRelocationIteratorRef) -> *const std::ffi::c_char;

  pub fn LLVMCreateObjectFile(MemBuf: LLVMMemoryBufferRef) -> LLVMObjectFileRef;

  pub fn LLVMDisposeObjectFile(ObjectFile: LLVMObjectFileRef) -> ();

  pub fn LLVMGetSections(ObjectFile: LLVMObjectFileRef) -> LLVMSectionIteratorRef;

  pub fn LLVMIsSectionIteratorAtEnd(ObjectFile: LLVMObjectFileRef, SI: LLVMSectionIteratorRef) -> std::ffi::c_int;

  pub fn LLVMGetSymbols(ObjectFile: LLVMObjectFileRef) -> LLVMSymbolIteratorRef;

  pub fn LLVMIsSymbolIteratorAtEnd(ObjectFile: LLVMObjectFileRef, SI: LLVMSymbolIteratorRef) -> std::ffi::c_int;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBinaryRef> {
  pub unsafe fn LLVMCreateBinary<T0_, T1_, T2_>(MemBuf_:  T0_, Context_:  T1_, ErrorMessage_:  T2_)-> Tret_
  where
     T0_: Into<LLVMMemoryBufferRef>,  T1_: Into<LLVMContextRef>,  T2_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMCreateBinary(Into::<LLVMMemoryBufferRef>::into(MemBuf_), Into::<LLVMContextRef>::into(Context_), Into::<*mut *mut std::ffi::c_char>::into(ErrorMessage_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeBinary<T0_>(BR_:  T0_)
  where
     T0_: Into<LLVMBinaryRef>
  {
    unsafe {
      crate::Object::LLVMDisposeBinary(Into::<LLVMBinaryRef>::into(BR_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMemoryBufferRef> {
  pub unsafe fn LLVMBinaryCopyMemoryBuffer<T0_>(BR_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBinaryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMBinaryCopyMemoryBuffer(Into::<LLVMBinaryRef>::into(BR_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMBinaryGetType<T0_>(BR_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBinaryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMBinaryGetType(Into::<LLVMBinaryRef>::into(BR_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBinaryRef> {
  pub unsafe fn LLVMMachOUniversalBinaryCopyObjectForArch<T0_, T1_, T2_, T3_>(BR_:  T0_, Arch_:  T1_, ArchLen_:  T2_, ErrorMessage_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBinaryRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMMachOUniversalBinaryCopyObjectForArch(Into::<LLVMBinaryRef>::into(BR_), Into::<*const std::ffi::c_char>::into(Arch_), Into::<std::ffi::c_ulong>::into(ArchLen_), Into::<*mut *mut std::ffi::c_char>::into(ErrorMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMSectionIteratorRef> {
  pub unsafe fn LLVMObjectFileCopySectionIterator<T0_>(BR_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBinaryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMObjectFileCopySectionIterator(Into::<LLVMBinaryRef>::into(BR_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMObjectFileIsSectionIteratorAtEnd<T0_, T1_>(BR_:  T0_, SI_:  T1_)-> Tret_
  where
     T0_: Into<LLVMBinaryRef>,  T1_: Into<LLVMSectionIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMObjectFileIsSectionIteratorAtEnd(Into::<LLVMBinaryRef>::into(BR_), Into::<LLVMSectionIteratorRef>::into(SI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMSymbolIteratorRef> {
  pub unsafe fn LLVMObjectFileCopySymbolIterator<T0_>(BR_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBinaryRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMObjectFileCopySymbolIterator(Into::<LLVMBinaryRef>::into(BR_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMObjectFileIsSymbolIteratorAtEnd<T0_, T1_>(BR_:  T0_, SI_:  T1_)-> Tret_
  where
     T0_: Into<LLVMBinaryRef>,  T1_: Into<LLVMSymbolIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMObjectFileIsSymbolIteratorAtEnd(Into::<LLVMBinaryRef>::into(BR_), Into::<LLVMSymbolIteratorRef>::into(SI_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeSectionIterator<T0_>(SI_:  T0_)
  where
     T0_: Into<LLVMSectionIteratorRef>
  {
    unsafe {
      crate::Object::LLVMDisposeSectionIterator(Into::<LLVMSectionIteratorRef>::into(SI_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMMoveToNextSection<T0_>(SI_:  T0_)
  where
     T0_: Into<LLVMSectionIteratorRef>
  {
    unsafe {
      crate::Object::LLVMMoveToNextSection(Into::<LLVMSectionIteratorRef>::into(SI_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMMoveToContainingSection<T0_, T1_>(Sect_:  T0_, Sym_:  T1_)
  where
     T0_: Into<LLVMSectionIteratorRef>,  T1_: Into<LLVMSymbolIteratorRef>
  {
    unsafe {
      crate::Object::LLVMMoveToContainingSection(Into::<LLVMSectionIteratorRef>::into(Sect_), Into::<LLVMSymbolIteratorRef>::into(Sym_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeSymbolIterator<T0_>(SI_:  T0_)
  where
     T0_: Into<LLVMSymbolIteratorRef>
  {
    unsafe {
      crate::Object::LLVMDisposeSymbolIterator(Into::<LLVMSymbolIteratorRef>::into(SI_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMMoveToNextSymbol<T0_>(SI_:  T0_)
  where
     T0_: Into<LLVMSymbolIteratorRef>
  {
    unsafe {
      crate::Object::LLVMMoveToNextSymbol(Into::<LLVMSymbolIteratorRef>::into(SI_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetSectionName<T0_>(SI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMSectionIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetSectionName(Into::<LLVMSectionIteratorRef>::into(SI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetSectionSize<T0_>(SI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMSectionIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetSectionSize(Into::<LLVMSectionIteratorRef>::into(SI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetSectionContents<T0_>(SI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMSectionIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetSectionContents(Into::<LLVMSectionIteratorRef>::into(SI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetSectionAddress<T0_>(SI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMSectionIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetSectionAddress(Into::<LLVMSectionIteratorRef>::into(SI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetSectionContainsSymbol<T0_, T1_>(SI_:  T0_, Sym_:  T1_)-> Tret_
  where
     T0_: Into<LLVMSectionIteratorRef>,  T1_: Into<LLVMSymbolIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetSectionContainsSymbol(Into::<LLVMSectionIteratorRef>::into(SI_), Into::<LLVMSymbolIteratorRef>::into(Sym_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMRelocationIteratorRef> {
  pub unsafe fn LLVMGetRelocations<T0_>(Section_:  T0_)-> Tret_
  where
     T0_: Into<LLVMSectionIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetRelocations(Into::<LLVMSectionIteratorRef>::into(Section_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeRelocationIterator<T0_>(RI_:  T0_)
  where
     T0_: Into<LLVMRelocationIteratorRef>
  {
    unsafe {
      crate::Object::LLVMDisposeRelocationIterator(Into::<LLVMRelocationIteratorRef>::into(RI_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsRelocationIteratorAtEnd<T0_, T1_>(Section_:  T0_, RI_:  T1_)-> Tret_
  where
     T0_: Into<LLVMSectionIteratorRef>,  T1_: Into<LLVMRelocationIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMIsRelocationIteratorAtEnd(Into::<LLVMSectionIteratorRef>::into(Section_), Into::<LLVMRelocationIteratorRef>::into(RI_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMMoveToNextRelocation<T0_>(RI_:  T0_)
  where
     T0_: Into<LLVMRelocationIteratorRef>
  {
    unsafe {
      crate::Object::LLVMMoveToNextRelocation(Into::<LLVMRelocationIteratorRef>::into(RI_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetSymbolName<T0_>(SI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMSymbolIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetSymbolName(Into::<LLVMSymbolIteratorRef>::into(SI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetSymbolAddress<T0_>(SI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMSymbolIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetSymbolAddress(Into::<LLVMSymbolIteratorRef>::into(SI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetSymbolSize<T0_>(SI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMSymbolIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetSymbolSize(Into::<LLVMSymbolIteratorRef>::into(SI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetRelocationOffset<T0_>(RI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRelocationIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetRelocationOffset(Into::<LLVMRelocationIteratorRef>::into(RI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMSymbolIteratorRef> {
  pub unsafe fn LLVMGetRelocationSymbol<T0_>(RI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRelocationIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetRelocationSymbol(Into::<LLVMRelocationIteratorRef>::into(RI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetRelocationType<T0_>(RI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRelocationIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetRelocationType(Into::<LLVMRelocationIteratorRef>::into(RI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetRelocationTypeName<T0_>(RI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRelocationIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetRelocationTypeName(Into::<LLVMRelocationIteratorRef>::into(RI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetRelocationValueString<T0_>(RI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMRelocationIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetRelocationValueString(Into::<LLVMRelocationIteratorRef>::into(RI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMObjectFileRef> {
  pub unsafe fn LLVMCreateObjectFile<T0_>(MemBuf_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMemoryBufferRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMCreateObjectFile(Into::<LLVMMemoryBufferRef>::into(MemBuf_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeObjectFile<T0_>(ObjectFile_:  T0_)
  where
     T0_: Into<LLVMObjectFileRef>
  {
    unsafe {
      crate::Object::LLVMDisposeObjectFile(Into::<LLVMObjectFileRef>::into(ObjectFile_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMSectionIteratorRef> {
  pub unsafe fn LLVMGetSections<T0_>(ObjectFile_:  T0_)-> Tret_
  where
     T0_: Into<LLVMObjectFileRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetSections(Into::<LLVMObjectFileRef>::into(ObjectFile_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsSectionIteratorAtEnd<T0_, T1_>(ObjectFile_:  T0_, SI_:  T1_)-> Tret_
  where
     T0_: Into<LLVMObjectFileRef>,  T1_: Into<LLVMSectionIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMIsSectionIteratorAtEnd(Into::<LLVMObjectFileRef>::into(ObjectFile_), Into::<LLVMSectionIteratorRef>::into(SI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMSymbolIteratorRef> {
  pub unsafe fn LLVMGetSymbols<T0_>(ObjectFile_:  T0_)-> Tret_
  where
     T0_: Into<LLVMObjectFileRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMGetSymbols(Into::<LLVMObjectFileRef>::into(ObjectFile_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsSymbolIteratorAtEnd<T0_, T1_>(ObjectFile_:  T0_, SI_:  T1_)-> Tret_
  where
     T0_: Into<LLVMObjectFileRef>,  T1_: Into<LLVMSymbolIteratorRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Object::LLVMIsSymbolIteratorAtEnd(Into::<LLVMObjectFileRef>::into(ObjectFile_), Into::<LLVMSymbolIteratorRef>::into(SI_))
      }
    )
  }
}

