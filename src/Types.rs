// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::DataTypes::*;
use crate::ExternC::*;
pub type LLVMBool = std::ffi::c_int;
pub type LLVMMemoryBufferRef = *mut u8;
pub type LLVMContextRef = *mut u8;
pub type LLVMModuleRef = *mut u8;
pub type LLVMTypeRef = *mut u8;
pub type LLVMValueRef = *mut u8;
pub type LLVMBasicBlockRef = *mut u8;
pub type LLVMMetadataRef = *mut u8;
pub type LLVMNamedMDNodeRef = *mut u8;
pub type LLVMValueMetadataEntry = u8;
pub type LLVMBuilderRef = *mut u8;
pub type LLVMDIBuilderRef = *mut u8;
pub type LLVMModuleProviderRef = *mut u8;
pub type LLVMPassManagerRef = *mut u8;
pub type LLVMUseRef = *mut u8;
pub type LLVMOperandBundleRef = *mut u8;
pub type LLVMAttributeRef = *mut u8;
pub type LLVMDiagnosticInfoRef = *mut u8;
pub type LLVMComdatRef = *mut u8;
pub type LLVMModuleFlagEntry = u8;
pub type LLVMJITEventListenerRef = *mut u8;
pub type LLVMBinaryRef = *mut u8;
pub type LLVMDbgRecordRef = *mut u8;

#[link(name = "LLVM")]
extern {

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

