// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::Deprecated::*;
use crate::ErrorHandling::*;
use crate::ExternC::*;
use crate::ExternC::*;
use crate::Types::*;
use crate::DataTypes::*;
use crate::ExternC::*;

pub const LLVMRet: std::ffi::c_uint = 1;
pub const LLVMBr: std::ffi::c_uint = 2;
pub const LLVMSwitch: std::ffi::c_uint = 3;
pub const LLVMIndirectBr: std::ffi::c_uint = 4;
pub const LLVMInvoke: std::ffi::c_uint = 5;
pub const LLVMUnreachable: std::ffi::c_uint = 7;
pub const LLVMCallBr: std::ffi::c_uint = 67;
pub const LLVMFNeg: std::ffi::c_uint = 66;
pub const LLVMAdd: std::ffi::c_uint = 8;
pub const LLVMFAdd: std::ffi::c_uint = 9;
pub const LLVMSub: std::ffi::c_uint = 10;
pub const LLVMFSub: std::ffi::c_uint = 11;
pub const LLVMMul: std::ffi::c_uint = 12;
pub const LLVMFMul: std::ffi::c_uint = 13;
pub const LLVMUDiv: std::ffi::c_uint = 14;
pub const LLVMSDiv: std::ffi::c_uint = 15;
pub const LLVMFDiv: std::ffi::c_uint = 16;
pub const LLVMURem: std::ffi::c_uint = 17;
pub const LLVMSRem: std::ffi::c_uint = 18;
pub const LLVMFRem: std::ffi::c_uint = 19;
pub const LLVMShl: std::ffi::c_uint = 20;
pub const LLVMLShr: std::ffi::c_uint = 21;
pub const LLVMAShr: std::ffi::c_uint = 22;
pub const LLVMAnd: std::ffi::c_uint = 23;
pub const LLVMOr: std::ffi::c_uint = 24;
pub const LLVMXor: std::ffi::c_uint = 25;
pub const LLVMAlloca: std::ffi::c_uint = 26;
pub const LLVMLoad: std::ffi::c_uint = 27;
pub const LLVMStore: std::ffi::c_uint = 28;
pub const LLVMGetElementPtr: std::ffi::c_uint = 29;
pub const LLVMTrunc: std::ffi::c_uint = 30;
pub const LLVMZExt: std::ffi::c_uint = 31;
pub const LLVMSExt: std::ffi::c_uint = 32;
pub const LLVMFPToUI: std::ffi::c_uint = 33;
pub const LLVMFPToSI: std::ffi::c_uint = 34;
pub const LLVMUIToFP: std::ffi::c_uint = 35;
pub const LLVMSIToFP: std::ffi::c_uint = 36;
pub const LLVMFPTrunc: std::ffi::c_uint = 37;
pub const LLVMFPExt: std::ffi::c_uint = 38;
pub const LLVMPtrToInt: std::ffi::c_uint = 39;
pub const LLVMIntToPtr: std::ffi::c_uint = 40;
pub const LLVMBitCast: std::ffi::c_uint = 41;
pub const LLVMAddrSpaceCast: std::ffi::c_uint = 60;
pub const LLVMICmp: std::ffi::c_uint = 42;
pub const LLVMFCmp: std::ffi::c_uint = 43;
pub const LLVMPHI: std::ffi::c_uint = 44;
pub const LLVMCall: std::ffi::c_uint = 45;
pub const LLVMSelect: std::ffi::c_uint = 46;
pub const LLVMUserOp1: std::ffi::c_uint = 47;
pub const LLVMUserOp2: std::ffi::c_uint = 48;
pub const LLVMVAArg: std::ffi::c_uint = 49;
pub const LLVMExtractElement: std::ffi::c_uint = 50;
pub const LLVMInsertElement: std::ffi::c_uint = 51;
pub const LLVMShuffleVector: std::ffi::c_uint = 52;
pub const LLVMExtractValue: std::ffi::c_uint = 53;
pub const LLVMInsertValue: std::ffi::c_uint = 54;
pub const LLVMFreeze: std::ffi::c_uint = 68;
pub const LLVMFence: std::ffi::c_uint = 55;
pub const LLVMAtomicCmpXchg: std::ffi::c_uint = 56;
pub const LLVMAtomicRMW: std::ffi::c_uint = 57;
pub const LLVMResume: std::ffi::c_uint = 58;
pub const LLVMLandingPad: std::ffi::c_uint = 59;
pub const LLVMCleanupRet: std::ffi::c_uint = 61;
pub const LLVMCatchRet: std::ffi::c_uint = 62;
pub const LLVMCatchPad: std::ffi::c_uint = 63;
pub const LLVMCleanupPad: std::ffi::c_uint = 64;
pub const LLVMCatchSwitch: std::ffi::c_uint = 65;
pub type LLVMOpcode = std::ffi::c_uint;

pub const LLVMVoidTypeKind: std::ffi::c_uint = 0;
pub const LLVMHalfTypeKind: std::ffi::c_uint = 1;
pub const LLVMFloatTypeKind: std::ffi::c_uint = 2;
pub const LLVMDoubleTypeKind: std::ffi::c_uint = 3;
pub const LLVMX86_FP80TypeKind: std::ffi::c_uint = 4;
pub const LLVMFP128TypeKind: std::ffi::c_uint = 5;
pub const LLVMPPC_FP128TypeKind: std::ffi::c_uint = 6;
pub const LLVMLabelTypeKind: std::ffi::c_uint = 7;
pub const LLVMIntegerTypeKind: std::ffi::c_uint = 8;
pub const LLVMFunctionTypeKind: std::ffi::c_uint = 9;
pub const LLVMStructTypeKind: std::ffi::c_uint = 10;
pub const LLVMArrayTypeKind: std::ffi::c_uint = 11;
pub const LLVMPointerTypeKind: std::ffi::c_uint = 12;
pub const LLVMVectorTypeKind: std::ffi::c_uint = 13;
pub const LLVMMetadataTypeKind: std::ffi::c_uint = 14;
pub const LLVMTokenTypeKind: std::ffi::c_uint = 16;
pub const LLVMScalableVectorTypeKind: std::ffi::c_uint = 17;
pub const LLVMBFloatTypeKind: std::ffi::c_uint = 18;
pub const LLVMX86_AMXTypeKind: std::ffi::c_uint = 19;
pub const LLVMTargetExtTypeKind: std::ffi::c_uint = 20;
pub type LLVMTypeKind = std::ffi::c_uint;

pub const LLVMExternalLinkage: std::ffi::c_uint = 0;
pub const LLVMAvailableExternallyLinkage: std::ffi::c_uint = 1;
pub const LLVMLinkOnceAnyLinkage: std::ffi::c_uint = 2;
pub const LLVMLinkOnceODRLinkage: std::ffi::c_uint = 3;
pub const LLVMLinkOnceODRAutoHideLinkage: std::ffi::c_uint = 4;
pub const LLVMWeakAnyLinkage: std::ffi::c_uint = 5;
pub const LLVMWeakODRLinkage: std::ffi::c_uint = 6;
pub const LLVMAppendingLinkage: std::ffi::c_uint = 7;
pub const LLVMInternalLinkage: std::ffi::c_uint = 8;
pub const LLVMPrivateLinkage: std::ffi::c_uint = 9;
pub const LLVMDLLImportLinkage: std::ffi::c_uint = 10;
pub const LLVMDLLExportLinkage: std::ffi::c_uint = 11;
pub const LLVMExternalWeakLinkage: std::ffi::c_uint = 12;
pub const LLVMGhostLinkage: std::ffi::c_uint = 13;
pub const LLVMCommonLinkage: std::ffi::c_uint = 14;
pub const LLVMLinkerPrivateLinkage: std::ffi::c_uint = 15;
pub const LLVMLinkerPrivateWeakLinkage: std::ffi::c_uint = 16;
pub type LLVMLinkage = std::ffi::c_uint;

pub const LLVMDefaultVisibility: std::ffi::c_uint = 0;
pub const LLVMHiddenVisibility: std::ffi::c_uint = 1;
pub const LLVMProtectedVisibility: std::ffi::c_uint = 2;
pub type LLVMVisibility = std::ffi::c_uint;

pub const LLVMNoUnnamedAddr: std::ffi::c_uint = 0;
pub const LLVMLocalUnnamedAddr: std::ffi::c_uint = 1;
pub const LLVMGlobalUnnamedAddr: std::ffi::c_uint = 2;
pub type LLVMUnnamedAddr = std::ffi::c_uint;

pub const LLVMDefaultStorageClass: std::ffi::c_uint = 0;
pub const LLVMDLLImportStorageClass: std::ffi::c_uint = 1;
pub const LLVMDLLExportStorageClass: std::ffi::c_uint = 2;
pub type LLVMDLLStorageClass = std::ffi::c_uint;

pub const LLVMCCallConv: std::ffi::c_uint = 0;
pub const LLVMFastCallConv: std::ffi::c_uint = 8;
pub const LLVMColdCallConv: std::ffi::c_uint = 9;
pub const LLVMGHCCallConv: std::ffi::c_uint = 10;
pub const LLVMHiPECallConv: std::ffi::c_uint = 11;
pub const LLVMAnyRegCallConv: std::ffi::c_uint = 13;
pub const LLVMPreserveMostCallConv: std::ffi::c_uint = 14;
pub const LLVMPreserveAllCallConv: std::ffi::c_uint = 15;
pub const LLVMSwiftCallConv: std::ffi::c_uint = 16;
pub const LLVMCXXFASTTLSCallConv: std::ffi::c_uint = 17;
pub const LLVMX86StdcallCallConv: std::ffi::c_uint = 64;
pub const LLVMX86FastcallCallConv: std::ffi::c_uint = 65;
pub const LLVMARMAPCSCallConv: std::ffi::c_uint = 66;
pub const LLVMARMAAPCSCallConv: std::ffi::c_uint = 67;
pub const LLVMARMAAPCSVFPCallConv: std::ffi::c_uint = 68;
pub const LLVMMSP430INTRCallConv: std::ffi::c_uint = 69;
pub const LLVMX86ThisCallCallConv: std::ffi::c_uint = 70;
pub const LLVMPTXKernelCallConv: std::ffi::c_uint = 71;
pub const LLVMPTXDeviceCallConv: std::ffi::c_uint = 72;
pub const LLVMSPIRFUNCCallConv: std::ffi::c_uint = 75;
pub const LLVMSPIRKERNELCallConv: std::ffi::c_uint = 76;
pub const LLVMIntelOCLBICallConv: std::ffi::c_uint = 77;
pub const LLVMX8664SysVCallConv: std::ffi::c_uint = 78;
pub const LLVMWin64CallConv: std::ffi::c_uint = 79;
pub const LLVMX86VectorCallCallConv: std::ffi::c_uint = 80;
pub const LLVMHHVMCallConv: std::ffi::c_uint = 81;
pub const LLVMHHVMCCallConv: std::ffi::c_uint = 82;
pub const LLVMX86INTRCallConv: std::ffi::c_uint = 83;
pub const LLVMAVRINTRCallConv: std::ffi::c_uint = 84;
pub const LLVMAVRSIGNALCallConv: std::ffi::c_uint = 85;
pub const LLVMAVRBUILTINCallConv: std::ffi::c_uint = 86;
pub const LLVMAMDGPUVSCallConv: std::ffi::c_uint = 87;
pub const LLVMAMDGPUGSCallConv: std::ffi::c_uint = 88;
pub const LLVMAMDGPUPSCallConv: std::ffi::c_uint = 89;
pub const LLVMAMDGPUCSCallConv: std::ffi::c_uint = 90;
pub const LLVMAMDGPUKERNELCallConv: std::ffi::c_uint = 91;
pub const LLVMX86RegCallCallConv: std::ffi::c_uint = 92;
pub const LLVMAMDGPUHSCallConv: std::ffi::c_uint = 93;
pub const LLVMMSP430BUILTINCallConv: std::ffi::c_uint = 94;
pub const LLVMAMDGPULSCallConv: std::ffi::c_uint = 95;
pub const LLVMAMDGPUESCallConv: std::ffi::c_uint = 96;
pub type LLVMCallConv = std::ffi::c_uint;

pub const LLVMArgumentValueKind: std::ffi::c_uint = 0;
pub const LLVMBasicBlockValueKind: std::ffi::c_uint = 1;
pub const LLVMMemoryUseValueKind: std::ffi::c_uint = 2;
pub const LLVMMemoryDefValueKind: std::ffi::c_uint = 3;
pub const LLVMMemoryPhiValueKind: std::ffi::c_uint = 4;
pub const LLVMFunctionValueKind: std::ffi::c_uint = 5;
pub const LLVMGlobalAliasValueKind: std::ffi::c_uint = 6;
pub const LLVMGlobalIFuncValueKind: std::ffi::c_uint = 7;
pub const LLVMGlobalVariableValueKind: std::ffi::c_uint = 8;
pub const LLVMBlockAddressValueKind: std::ffi::c_uint = 9;
pub const LLVMConstantExprValueKind: std::ffi::c_uint = 10;
pub const LLVMConstantArrayValueKind: std::ffi::c_uint = 11;
pub const LLVMConstantStructValueKind: std::ffi::c_uint = 12;
pub const LLVMConstantVectorValueKind: std::ffi::c_uint = 13;
pub const LLVMUndefValueValueKind: std::ffi::c_uint = 14;
pub const LLVMConstantAggregateZeroValueKind: std::ffi::c_uint = 15;
pub const LLVMConstantDataArrayValueKind: std::ffi::c_uint = 16;
pub const LLVMConstantDataVectorValueKind: std::ffi::c_uint = 17;
pub const LLVMConstantIntValueKind: std::ffi::c_uint = 18;
pub const LLVMConstantFPValueKind: std::ffi::c_uint = 19;
pub const LLVMConstantPointerNullValueKind: std::ffi::c_uint = 20;
pub const LLVMConstantTokenNoneValueKind: std::ffi::c_uint = 21;
pub const LLVMMetadataAsValueValueKind: std::ffi::c_uint = 22;
pub const LLVMInlineAsmValueKind: std::ffi::c_uint = 23;
pub const LLVMInstructionValueKind: std::ffi::c_uint = 24;
pub const LLVMPoisonValueValueKind: std::ffi::c_uint = 25;
pub const LLVMConstantTargetNoneValueKind: std::ffi::c_uint = 26;
pub const LLVMConstantPtrAuthValueKind: std::ffi::c_uint = 27;
pub type LLVMValueKind = std::ffi::c_uint;

pub const LLVMIntEQ: std::ffi::c_uint = 32;
pub const LLVMIntNE: std::ffi::c_uint = 33;
pub const LLVMIntUGT: std::ffi::c_uint = 34;
pub const LLVMIntUGE: std::ffi::c_uint = 35;
pub const LLVMIntULT: std::ffi::c_uint = 36;
pub const LLVMIntULE: std::ffi::c_uint = 37;
pub const LLVMIntSGT: std::ffi::c_uint = 38;
pub const LLVMIntSGE: std::ffi::c_uint = 39;
pub const LLVMIntSLT: std::ffi::c_uint = 40;
pub const LLVMIntSLE: std::ffi::c_uint = 41;
pub type LLVMIntPredicate = std::ffi::c_uint;

pub const LLVMRealPredicateFalse: std::ffi::c_uint = 0;
pub const LLVMRealOEQ: std::ffi::c_uint = 1;
pub const LLVMRealOGT: std::ffi::c_uint = 2;
pub const LLVMRealOGE: std::ffi::c_uint = 3;
pub const LLVMRealOLT: std::ffi::c_uint = 4;
pub const LLVMRealOLE: std::ffi::c_uint = 5;
pub const LLVMRealONE: std::ffi::c_uint = 6;
pub const LLVMRealORD: std::ffi::c_uint = 7;
pub const LLVMRealUNO: std::ffi::c_uint = 8;
pub const LLVMRealUEQ: std::ffi::c_uint = 9;
pub const LLVMRealUGT: std::ffi::c_uint = 10;
pub const LLVMRealUGE: std::ffi::c_uint = 11;
pub const LLVMRealULT: std::ffi::c_uint = 12;
pub const LLVMRealULE: std::ffi::c_uint = 13;
pub const LLVMRealUNE: std::ffi::c_uint = 14;
pub const LLVMRealPredicateTrue: std::ffi::c_uint = 15;
pub type LLVMRealPredicate = std::ffi::c_uint;

pub const LLVMLandingPadCatch: std::ffi::c_uint = 0;
pub const LLVMLandingPadFilter: std::ffi::c_uint = 1;
pub type LLVMLandingPadClauseTy = std::ffi::c_uint;

pub const LLVMNotThreadLocal: std::ffi::c_uint = 0;
pub const LLVMGeneralDynamicTLSModel: std::ffi::c_uint = 1;
pub const LLVMLocalDynamicTLSModel: std::ffi::c_uint = 2;
pub const LLVMInitialExecTLSModel: std::ffi::c_uint = 3;
pub const LLVMLocalExecTLSModel: std::ffi::c_uint = 4;
pub type LLVMThreadLocalMode = std::ffi::c_uint;

pub const LLVMAtomicOrderingNotAtomic: std::ffi::c_uint = 0;
pub const LLVMAtomicOrderingUnordered: std::ffi::c_uint = 1;
pub const LLVMAtomicOrderingMonotonic: std::ffi::c_uint = 2;
pub const LLVMAtomicOrderingAcquire: std::ffi::c_uint = 4;
pub const LLVMAtomicOrderingRelease: std::ffi::c_uint = 5;
pub const LLVMAtomicOrderingAcquireRelease: std::ffi::c_uint = 6;
pub const LLVMAtomicOrderingSequentiallyConsistent: std::ffi::c_uint = 7;
pub type LLVMAtomicOrdering = std::ffi::c_uint;

pub const LLVMAtomicRMWBinOpXchg: std::ffi::c_uint = 0;
pub const LLVMAtomicRMWBinOpAdd: std::ffi::c_uint = 1;
pub const LLVMAtomicRMWBinOpSub: std::ffi::c_uint = 2;
pub const LLVMAtomicRMWBinOpAnd: std::ffi::c_uint = 3;
pub const LLVMAtomicRMWBinOpNand: std::ffi::c_uint = 4;
pub const LLVMAtomicRMWBinOpOr: std::ffi::c_uint = 5;
pub const LLVMAtomicRMWBinOpXor: std::ffi::c_uint = 6;
pub const LLVMAtomicRMWBinOpMax: std::ffi::c_uint = 7;
pub const LLVMAtomicRMWBinOpMin: std::ffi::c_uint = 8;
pub const LLVMAtomicRMWBinOpUMax: std::ffi::c_uint = 9;
pub const LLVMAtomicRMWBinOpUMin: std::ffi::c_uint = 10;
pub const LLVMAtomicRMWBinOpFAdd: std::ffi::c_uint = 11;
pub const LLVMAtomicRMWBinOpFSub: std::ffi::c_uint = 12;
pub const LLVMAtomicRMWBinOpFMax: std::ffi::c_uint = 13;
pub const LLVMAtomicRMWBinOpFMin: std::ffi::c_uint = 14;
pub const LLVMAtomicRMWBinOpUIncWrap: std::ffi::c_uint = 15;
pub const LLVMAtomicRMWBinOpUDecWrap: std::ffi::c_uint = 16;
pub const LLVMAtomicRMWBinOpUSubCond: std::ffi::c_uint = 17;
pub const LLVMAtomicRMWBinOpUSubSat: std::ffi::c_uint = 18;
pub type LLVMAtomicRMWBinOp = std::ffi::c_uint;

pub const LLVMDSError: std::ffi::c_uint = 0;
pub const LLVMDSWarning: std::ffi::c_uint = 1;
pub const LLVMDSRemark: std::ffi::c_uint = 2;
pub const LLVMDSNote: std::ffi::c_uint = 3;
pub type LLVMDiagnosticSeverity = std::ffi::c_uint;

pub const LLVMInlineAsmDialectATT: std::ffi::c_uint = 0;
pub const LLVMInlineAsmDialectIntel: std::ffi::c_uint = 1;
pub type LLVMInlineAsmDialect = std::ffi::c_uint;

pub const LLVMModuleFlagBehaviorError: std::ffi::c_uint = 0;
pub const LLVMModuleFlagBehaviorWarning: std::ffi::c_uint = 1;
pub const LLVMModuleFlagBehaviorRequire: std::ffi::c_uint = 2;
pub const LLVMModuleFlagBehaviorOverride: std::ffi::c_uint = 3;
pub const LLVMModuleFlagBehaviorAppend: std::ffi::c_uint = 4;
pub const LLVMModuleFlagBehaviorAppendUnique: std::ffi::c_uint = 5;
pub type LLVMModuleFlagBehavior = std::ffi::c_uint;

pub const LLVMAttributeReturnIndex: std::ffi::c_int = 0;
pub const LLVMAttributeFunctionIndex: std::ffi::c_int = -1;
pub type LLVMAttributeIndex = std::ffi::c_uint;

pub const LLVMTailCallKindNone: std::ffi::c_uint = 0;
pub const LLVMTailCallKindTail: std::ffi::c_uint = 1;
pub const LLVMTailCallKindMustTail: std::ffi::c_uint = 2;
pub const LLVMTailCallKindNoTail: std::ffi::c_uint = 3;
pub type LLVMTailCallKind = std::ffi::c_uint;

pub const LLVMFastMathAllowReassoc: std::ffi::c_uint = 1;
pub const LLVMFastMathNoNaNs: std::ffi::c_uint = 2;
pub const LLVMFastMathNoInfs: std::ffi::c_uint = 4;
pub const LLVMFastMathNoSignedZeros: std::ffi::c_uint = 8;
pub const LLVMFastMathAllowReciprocal: std::ffi::c_uint = 16;
pub const LLVMFastMathAllowContract: std::ffi::c_uint = 32;
pub const LLVMFastMathApproxFunc: std::ffi::c_uint = 64;
pub const LLVMFastMathNone: std::ffi::c_uint = 0;
pub const LLVMFastMathAll: std::ffi::c_uint = 127;
pub type LLVMFastMathFlags = std::ffi::c_uint;

pub const LLVMGEPFlagInBounds: std::ffi::c_uint = 1;
pub const LLVMGEPFlagNUSW: std::ffi::c_uint = 2;
pub const LLVMGEPFlagNUW: std::ffi::c_uint = 4;
pub type LLVMGEPNoWrapFlags = std::ffi::c_uint;
pub type LLVMDiagnosticHandler = *mut extern fn (LLVMDiagnosticInfoRef, *mut std::ffi::c_void) -> ();
pub type LLVMYieldCallback = *mut extern fn (LLVMContextRef, *mut std::ffi::c_void) -> ();

#[link(name = "LLVM")]
extern {

  pub fn LLVMShutdown() -> ();

  pub fn LLVMGetVersion(Major: *mut std::ffi::c_uint, Minor: *mut std::ffi::c_uint, Patch: *mut std::ffi::c_uint) -> ();

  pub fn LLVMCreateMessage(Message: *const std::ffi::c_char) -> *mut std::ffi::c_char;

  pub fn LLVMDisposeMessage(Message: *mut std::ffi::c_char) -> ();

  pub fn LLVMContextCreate() -> LLVMContextRef;

  pub fn LLVMGetGlobalContext() -> LLVMContextRef;

  pub fn LLVMContextSetDiagnosticHandler(C: LLVMContextRef, Handler: LLVMDiagnosticHandler, DiagnosticContext: *mut std::ffi::c_void) -> ();

  pub fn LLVMContextGetDiagnosticHandler(C: LLVMContextRef) -> LLVMDiagnosticHandler;

  pub fn LLVMContextGetDiagnosticContext(C: LLVMContextRef) -> *mut std::ffi::c_void;

  pub fn LLVMContextSetYieldCallback(C: LLVMContextRef, Callback: LLVMYieldCallback, OpaqueHandle: *mut std::ffi::c_void) -> ();

  pub fn LLVMContextShouldDiscardValueNames(C: LLVMContextRef) -> std::ffi::c_int;

  pub fn LLVMContextSetDiscardValueNames(C: LLVMContextRef, Discard: std::ffi::c_int) -> ();

  pub fn LLVMContextDispose(C: LLVMContextRef) -> ();

  pub fn LLVMGetDiagInfoDescription(DI: LLVMDiagnosticInfoRef) -> *mut std::ffi::c_char;

  pub fn LLVMGetDiagInfoSeverity(DI: LLVMDiagnosticInfoRef) -> std::ffi::c_uint;

  pub fn LLVMGetMDKindIDInContext(C: LLVMContextRef, Name: *const std::ffi::c_char, SLen: std::ffi::c_uint) -> std::ffi::c_uint;

  pub fn LLVMGetMDKindID(Name: *const std::ffi::c_char, SLen: std::ffi::c_uint) -> std::ffi::c_uint;

  pub fn LLVMGetSyncScopeID(C: LLVMContextRef, Name: *const std::ffi::c_char, SLen: std::ffi::c_ulong) -> std::ffi::c_uint;

  pub fn LLVMGetEnumAttributeKindForName(Name: *const std::ffi::c_char, SLen: std::ffi::c_ulong) -> std::ffi::c_uint;

  pub fn LLVMGetLastEnumAttributeKind() -> std::ffi::c_uint;

  pub fn LLVMCreateEnumAttribute(C: LLVMContextRef, KindID: std::ffi::c_uint, Val: std::ffi::c_ulong) -> LLVMAttributeRef;

  pub fn LLVMGetEnumAttributeKind(A: LLVMAttributeRef) -> std::ffi::c_uint;

  pub fn LLVMGetEnumAttributeValue(A: LLVMAttributeRef) -> std::ffi::c_ulong;

  pub fn LLVMCreateTypeAttribute(C: LLVMContextRef, KindID: std::ffi::c_uint, type_ref: LLVMTypeRef) -> LLVMAttributeRef;

  pub fn LLVMGetTypeAttributeValue(A: LLVMAttributeRef) -> LLVMTypeRef;

  pub fn LLVMCreateConstantRangeAttribute(C: LLVMContextRef, KindID: std::ffi::c_uint, NumBits: std::ffi::c_uint, LowerWords: *const std::ffi::c_ulong, UpperWords: *const std::ffi::c_ulong) -> LLVMAttributeRef;

  pub fn LLVMCreateStringAttribute(C: LLVMContextRef, K: *const std::ffi::c_char, KLength: std::ffi::c_uint, V: *const std::ffi::c_char, VLength: std::ffi::c_uint) -> LLVMAttributeRef;

  pub fn LLVMGetStringAttributeKind(A: LLVMAttributeRef, Length: *mut std::ffi::c_uint) -> *const std::ffi::c_char;

  pub fn LLVMGetStringAttributeValue(A: LLVMAttributeRef, Length: *mut std::ffi::c_uint) -> *const std::ffi::c_char;

  pub fn LLVMIsEnumAttribute(A: LLVMAttributeRef) -> std::ffi::c_int;

  pub fn LLVMIsStringAttribute(A: LLVMAttributeRef) -> std::ffi::c_int;

  pub fn LLVMIsTypeAttribute(A: LLVMAttributeRef) -> std::ffi::c_int;

  pub fn LLVMGetTypeByName2(C: LLVMContextRef, Name: *const std::ffi::c_char) -> LLVMTypeRef;

  pub fn LLVMModuleCreateWithName(ModuleID: *const std::ffi::c_char) -> LLVMModuleRef;

  pub fn LLVMModuleCreateWithNameInContext(ModuleID: *const std::ffi::c_char, C: LLVMContextRef) -> LLVMModuleRef;

  pub fn LLVMCloneModule(M: LLVMModuleRef) -> LLVMModuleRef;

  pub fn LLVMDisposeModule(M: LLVMModuleRef) -> ();

  pub fn LLVMIsNewDbgInfoFormat(M: LLVMModuleRef) -> std::ffi::c_int;

  pub fn LLVMSetIsNewDbgInfoFormat(M: LLVMModuleRef, UseNewFormat: std::ffi::c_int) -> ();

  pub fn LLVMGetModuleIdentifier(M: LLVMModuleRef, Len: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMSetModuleIdentifier(M: LLVMModuleRef, Ident: *const std::ffi::c_char, Len: std::ffi::c_ulong) -> ();

  pub fn LLVMGetSourceFileName(M: LLVMModuleRef, Len: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMSetSourceFileName(M: LLVMModuleRef, Name: *const std::ffi::c_char, Len: std::ffi::c_ulong) -> ();

  pub fn LLVMGetDataLayoutStr(M: LLVMModuleRef) -> *const std::ffi::c_char;

  pub fn LLVMGetDataLayout(M: LLVMModuleRef) -> *const std::ffi::c_char;

  pub fn LLVMSetDataLayout(M: LLVMModuleRef, DataLayoutStr: *const std::ffi::c_char) -> ();

  pub fn LLVMGetTarget(M: LLVMModuleRef) -> *const std::ffi::c_char;

  pub fn LLVMSetTarget(M: LLVMModuleRef, Triple: *const std::ffi::c_char) -> ();

  pub fn LLVMCopyModuleFlagsMetadata(M: LLVMModuleRef, Len: *mut std::ffi::c_ulong) -> *mut LLVMModuleFlagEntry;

  pub fn LLVMDisposeModuleFlagsMetadata(Entries: *mut LLVMModuleFlagEntry) -> ();

  pub fn LLVMModuleFlagEntriesGetFlagBehavior(Entries: *mut LLVMModuleFlagEntry, Index: std::ffi::c_uint) -> std::ffi::c_uint;

  pub fn LLVMModuleFlagEntriesGetKey(Entries: *mut LLVMModuleFlagEntry, Index: std::ffi::c_uint, Len: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMModuleFlagEntriesGetMetadata(Entries: *mut LLVMModuleFlagEntry, Index: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMGetModuleFlag(M: LLVMModuleRef, Key: *const std::ffi::c_char, KeyLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMAddModuleFlag(M: LLVMModuleRef, Behavior: std::ffi::c_uint, Key: *const std::ffi::c_char, KeyLen: std::ffi::c_ulong, Val: LLVMMetadataRef) -> ();

  pub fn LLVMDumpModule(M: LLVMModuleRef) -> ();

  pub fn LLVMPrintModuleToFile(M: LLVMModuleRef, Filename: *const std::ffi::c_char, ErrorMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMPrintModuleToString(M: LLVMModuleRef) -> *mut std::ffi::c_char;

  pub fn LLVMGetModuleInlineAsm(M: LLVMModuleRef, Len: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMSetModuleInlineAsm2(M: LLVMModuleRef, Asm: *const std::ffi::c_char, Len: std::ffi::c_ulong) -> ();

  pub fn LLVMAppendModuleInlineAsm(M: LLVMModuleRef, Asm: *const std::ffi::c_char, Len: std::ffi::c_ulong) -> ();

  pub fn LLVMGetInlineAsm(Ty: LLVMTypeRef, AsmString: *const std::ffi::c_char, AsmStringSize: std::ffi::c_ulong, Constraints: *const std::ffi::c_char, ConstraintsSize: std::ffi::c_ulong, HasSideEffects: std::ffi::c_int, IsAlignStack: std::ffi::c_int, Dialect: std::ffi::c_uint, CanThrow: std::ffi::c_int) -> LLVMValueRef;

  pub fn LLVMGetInlineAsmAsmString(InlineAsmVal: LLVMValueRef, Len: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMGetInlineAsmConstraintString(InlineAsmVal: LLVMValueRef, Len: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMGetInlineAsmDialect(InlineAsmVal: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetInlineAsmFunctionType(InlineAsmVal: LLVMValueRef) -> LLVMTypeRef;

  pub fn LLVMGetInlineAsmHasSideEffects(InlineAsmVal: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMGetInlineAsmNeedsAlignedStack(InlineAsmVal: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMGetInlineAsmCanUnwind(InlineAsmVal: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMGetModuleContext(M: LLVMModuleRef) -> LLVMContextRef;

  pub fn LLVMGetTypeByName(M: LLVMModuleRef, Name: *const std::ffi::c_char) -> LLVMTypeRef;

  pub fn LLVMGetFirstNamedMetadata(M: LLVMModuleRef) -> LLVMNamedMDNodeRef;

  pub fn LLVMGetLastNamedMetadata(M: LLVMModuleRef) -> LLVMNamedMDNodeRef;

  pub fn LLVMGetNextNamedMetadata(NamedMDNode: LLVMNamedMDNodeRef) -> LLVMNamedMDNodeRef;

  pub fn LLVMGetPreviousNamedMetadata(NamedMDNode: LLVMNamedMDNodeRef) -> LLVMNamedMDNodeRef;

  pub fn LLVMGetNamedMetadata(M: LLVMModuleRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong) -> LLVMNamedMDNodeRef;

  pub fn LLVMGetOrInsertNamedMetadata(M: LLVMModuleRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong) -> LLVMNamedMDNodeRef;

  pub fn LLVMGetNamedMetadataName(NamedMD: LLVMNamedMDNodeRef, NameLen: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMGetNamedMetadataNumOperands(M: LLVMModuleRef, Name: *const std::ffi::c_char) -> std::ffi::c_uint;

  pub fn LLVMGetNamedMetadataOperands(M: LLVMModuleRef, Name: *const std::ffi::c_char, Dest: *mut LLVMValueRef) -> ();

  pub fn LLVMAddNamedMetadataOperand(M: LLVMModuleRef, Name: *const std::ffi::c_char, Val: LLVMValueRef) -> ();

  pub fn LLVMGetDebugLocDirectory(Val: LLVMValueRef, Length: *mut std::ffi::c_uint) -> *const std::ffi::c_char;

  pub fn LLVMGetDebugLocFilename(Val: LLVMValueRef, Length: *mut std::ffi::c_uint) -> *const std::ffi::c_char;

  pub fn LLVMGetDebugLocLine(Val: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetDebugLocColumn(Val: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMAddFunction(M: LLVMModuleRef, Name: *const std::ffi::c_char, FunctionTy: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMGetNamedFunction(M: LLVMModuleRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMGetNamedFunctionWithLength(M: LLVMModuleRef, Name: *const std::ffi::c_char, Length: std::ffi::c_ulong) -> LLVMValueRef;

  pub fn LLVMGetFirstFunction(M: LLVMModuleRef) -> LLVMValueRef;

  pub fn LLVMGetLastFunction(M: LLVMModuleRef) -> LLVMValueRef;

  pub fn LLVMGetNextFunction(Fn: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetPreviousFunction(Fn: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMSetModuleInlineAsm(M: LLVMModuleRef, Asm: *const std::ffi::c_char) -> ();

  pub fn LLVMGetTypeKind(Ty: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMTypeIsSized(Ty: LLVMTypeRef) -> std::ffi::c_int;

  pub fn LLVMGetTypeContext(Ty: LLVMTypeRef) -> LLVMContextRef;

  pub fn LLVMDumpType(Val: LLVMTypeRef) -> ();

  pub fn LLVMPrintTypeToString(Val: LLVMTypeRef) -> *mut std::ffi::c_char;

  pub fn LLVMInt1TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMInt8TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMInt16TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMInt32TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMInt64TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMInt128TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMIntTypeInContext(C: LLVMContextRef, NumBits: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMInt1Type() -> LLVMTypeRef;

  pub fn LLVMInt8Type() -> LLVMTypeRef;

  pub fn LLVMInt16Type() -> LLVMTypeRef;

  pub fn LLVMInt32Type() -> LLVMTypeRef;

  pub fn LLVMInt64Type() -> LLVMTypeRef;

  pub fn LLVMInt128Type() -> LLVMTypeRef;

  pub fn LLVMIntType(NumBits: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMGetIntTypeWidth(IntegerTy: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMHalfTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMBFloatTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMFloatTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMDoubleTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMX86FP80TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMFP128TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMPPCFP128TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMHalfType() -> LLVMTypeRef;

  pub fn LLVMBFloatType() -> LLVMTypeRef;

  pub fn LLVMFloatType() -> LLVMTypeRef;

  pub fn LLVMDoubleType() -> LLVMTypeRef;

  pub fn LLVMX86FP80Type() -> LLVMTypeRef;

  pub fn LLVMFP128Type() -> LLVMTypeRef;

  pub fn LLVMPPCFP128Type() -> LLVMTypeRef;

  pub fn LLVMFunctionType(ReturnType: LLVMTypeRef, ParamTypes: *mut LLVMTypeRef, ParamCount: std::ffi::c_uint, IsVarArg: std::ffi::c_int) -> LLVMTypeRef;

  pub fn LLVMIsFunctionVarArg(FunctionTy: LLVMTypeRef) -> std::ffi::c_int;

  pub fn LLVMGetReturnType(FunctionTy: LLVMTypeRef) -> LLVMTypeRef;

  pub fn LLVMCountParamTypes(FunctionTy: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMGetParamTypes(FunctionTy: LLVMTypeRef, Dest: *mut LLVMTypeRef) -> ();

  pub fn LLVMStructTypeInContext(C: LLVMContextRef, ElementTypes: *mut LLVMTypeRef, ElementCount: std::ffi::c_uint, Packed: std::ffi::c_int) -> LLVMTypeRef;

  pub fn LLVMStructType(ElementTypes: *mut LLVMTypeRef, ElementCount: std::ffi::c_uint, Packed: std::ffi::c_int) -> LLVMTypeRef;

  pub fn LLVMStructCreateNamed(C: LLVMContextRef, Name: *const std::ffi::c_char) -> LLVMTypeRef;

  pub fn LLVMGetStructName(Ty: LLVMTypeRef) -> *const std::ffi::c_char;

  pub fn LLVMStructSetBody(StructTy: LLVMTypeRef, ElementTypes: *mut LLVMTypeRef, ElementCount: std::ffi::c_uint, Packed: std::ffi::c_int) -> ();

  pub fn LLVMCountStructElementTypes(StructTy: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMGetStructElementTypes(StructTy: LLVMTypeRef, Dest: *mut LLVMTypeRef) -> ();

  pub fn LLVMStructGetTypeAtIndex(StructTy: LLVMTypeRef, i: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMIsPackedStruct(StructTy: LLVMTypeRef) -> std::ffi::c_int;

  pub fn LLVMIsOpaqueStruct(StructTy: LLVMTypeRef) -> std::ffi::c_int;

  pub fn LLVMIsLiteralStruct(StructTy: LLVMTypeRef) -> std::ffi::c_int;

  pub fn LLVMGetElementType(Ty: LLVMTypeRef) -> LLVMTypeRef;

  pub fn LLVMGetSubtypes(Tp: LLVMTypeRef, Arr: *mut LLVMTypeRef) -> ();

  pub fn LLVMGetNumContainedTypes(Tp: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMArrayType(ElementType: LLVMTypeRef, ElementCount: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMArrayType2(ElementType: LLVMTypeRef, ElementCount: std::ffi::c_ulong) -> LLVMTypeRef;

  pub fn LLVMGetArrayLength(ArrayTy: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMGetArrayLength2(ArrayTy: LLVMTypeRef) -> std::ffi::c_ulong;

  pub fn LLVMPointerType(ElementType: LLVMTypeRef, AddressSpace: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMPointerTypeIsOpaque(Ty: LLVMTypeRef) -> std::ffi::c_int;

  pub fn LLVMPointerTypeInContext(C: LLVMContextRef, AddressSpace: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMGetPointerAddressSpace(PointerTy: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMVectorType(ElementType: LLVMTypeRef, ElementCount: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMScalableVectorType(ElementType: LLVMTypeRef, ElementCount: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMGetVectorSize(VectorTy: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMGetConstantPtrAuthPointer(PtrAuth: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetConstantPtrAuthKey(PtrAuth: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetConstantPtrAuthDiscriminator(PtrAuth: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetConstantPtrAuthAddrDiscriminator(PtrAuth: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMVoidTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMLabelTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMX86AMXTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMTokenTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMMetadataTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

  pub fn LLVMVoidType() -> LLVMTypeRef;

  pub fn LLVMLabelType() -> LLVMTypeRef;

  pub fn LLVMX86AMXType() -> LLVMTypeRef;

  pub fn LLVMTargetExtTypeInContext(C: LLVMContextRef, Name: *const std::ffi::c_char, TypeParams: *mut LLVMTypeRef, TypeParamCount: std::ffi::c_uint, IntParams: *mut std::ffi::c_uint, IntParamCount: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMGetTargetExtTypeName(TargetExtTy: LLVMTypeRef) -> *const std::ffi::c_char;

  pub fn LLVMGetTargetExtTypeNumTypeParams(TargetExtTy: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMGetTargetExtTypeTypeParam(TargetExtTy: LLVMTypeRef, Idx: std::ffi::c_uint) -> LLVMTypeRef;

  pub fn LLVMGetTargetExtTypeNumIntParams(TargetExtTy: LLVMTypeRef) -> std::ffi::c_uint;

  pub fn LLVMGetTargetExtTypeIntParam(TargetExtTy: LLVMTypeRef, Idx: std::ffi::c_uint) -> std::ffi::c_uint;

  pub fn LLVMTypeOf(Val: LLVMValueRef) -> LLVMTypeRef;

  pub fn LLVMGetValueKind(Val: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetValueName2(Val: LLVMValueRef, Length: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMSetValueName2(Val: LLVMValueRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong) -> ();

  pub fn LLVMDumpValue(Val: LLVMValueRef) -> ();

  pub fn LLVMPrintValueToString(Val: LLVMValueRef) -> *mut std::ffi::c_char;

  pub fn LLVMGetValueContext(Val: LLVMValueRef) -> LLVMContextRef;

  pub fn LLVMPrintDbgRecordToString(Record: LLVMDbgRecordRef) -> *mut std::ffi::c_char;

  pub fn LLVMReplaceAllUsesWith(OldVal: LLVMValueRef, NewVal: LLVMValueRef) -> ();

  pub fn LLVMIsConstant(Val: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMIsUndef(Val: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMIsPoison(Val: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMIsAArgument(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsABasicBlock(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAInlineAsm(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAUser(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstant(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsABlockAddress(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantAggregateZero(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantArray(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantDataSequential(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantDataArray(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantDataVector(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantExpr(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantFP(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantInt(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantPointerNull(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantStruct(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantTokenNone(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantVector(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAConstantPtrAuth(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAGlobalValue(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAGlobalAlias(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAGlobalObject(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAFunction(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAGlobalVariable(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAGlobalIFunc(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAUndefValue(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAPoisonValue(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAInstruction(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAUnaryOperator(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsABinaryOperator(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsACallInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAIntrinsicInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsADbgInfoIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsADbgVariableIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsADbgDeclareInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsADbgLabelInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAMemIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAMemCpyInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAMemMoveInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAMemSetInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsACmpInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAFCmpInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAICmpInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAExtractElementInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAGetElementPtrInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAInsertElementInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAInsertValueInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsALandingPadInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAPHINode(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsASelectInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAShuffleVectorInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAStoreInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsABranchInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAIndirectBrInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAInvokeInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAReturnInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsASwitchInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAUnreachableInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAResumeInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsACleanupReturnInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsACatchReturnInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsACatchSwitchInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsACallBrInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAFuncletPadInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsACatchPadInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsACleanupPadInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAUnaryInstruction(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAAllocaInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsACastInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAAddrSpaceCastInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsABitCastInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAFPExtInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAFPToSIInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAFPToUIInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAFPTruncInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAIntToPtrInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAPtrToIntInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsASExtInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsASIToFPInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsATruncInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAUIToFPInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAZExtInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAExtractValueInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsALoadInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAVAArgInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAFreezeInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAAtomicCmpXchgInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAAtomicRMWInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAFenceInst(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAMDNode(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAValueAsMetadata(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsAMDString(Val: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetValueName(Val: LLVMValueRef) -> *const std::ffi::c_char;

  pub fn LLVMSetValueName(Val: LLVMValueRef, Name: *const std::ffi::c_char) -> ();

  pub fn LLVMGetFirstUse(Val: LLVMValueRef) -> LLVMUseRef;

  pub fn LLVMGetNextUse(U: LLVMUseRef) -> LLVMUseRef;

  pub fn LLVMGetUser(U: LLVMUseRef) -> LLVMValueRef;

  pub fn LLVMGetUsedValue(U: LLVMUseRef) -> LLVMValueRef;

  pub fn LLVMGetOperand(Val: LLVMValueRef, Index: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMGetOperandUse(Val: LLVMValueRef, Index: std::ffi::c_uint) -> LLVMUseRef;

  pub fn LLVMSetOperand(User: LLVMValueRef, Index: std::ffi::c_uint, Val: LLVMValueRef) -> ();

  pub fn LLVMGetNumOperands(Val: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMConstNull(Ty: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMConstAllOnes(Ty: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMGetUndef(Ty: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMGetPoison(Ty: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMIsNull(Val: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMConstPointerNull(Ty: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMConstInt(IntTy: LLVMTypeRef, N: std::ffi::c_ulonglong, SignExtend: std::ffi::c_int) -> LLVMValueRef;

  pub fn LLVMConstIntOfArbitraryPrecision(IntTy: LLVMTypeRef, NumWords: std::ffi::c_uint, Words: *const std::ffi::c_ulong) -> LLVMValueRef;

  pub fn LLVMConstIntOfString(IntTy: LLVMTypeRef, Text: *const std::ffi::c_char, Radix: std::ffi::c_uchar) -> LLVMValueRef;

  pub fn LLVMConstIntOfStringAndSize(IntTy: LLVMTypeRef, Text: *const std::ffi::c_char, SLen: std::ffi::c_uint, Radix: std::ffi::c_uchar) -> LLVMValueRef;

  pub fn LLVMConstReal(RealTy: LLVMTypeRef, N: f64) -> LLVMValueRef;

  pub fn LLVMConstRealOfString(RealTy: LLVMTypeRef, Text: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMConstRealOfStringAndSize(RealTy: LLVMTypeRef, Text: *const std::ffi::c_char, SLen: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMConstIntGetZExtValue(ConstantVal: LLVMValueRef) -> std::ffi::c_ulonglong;

  pub fn LLVMConstIntGetSExtValue(ConstantVal: LLVMValueRef) -> std::ffi::c_longlong;

  pub fn LLVMConstRealGetDouble(ConstantVal: LLVMValueRef, losesInfo: *mut std::ffi::c_int) -> f64;

  pub fn LLVMConstStringInContext(C: LLVMContextRef, Str: *const std::ffi::c_char, Length: std::ffi::c_uint, DontNullTerminate: std::ffi::c_int) -> LLVMValueRef;

  pub fn LLVMConstStringInContext2(C: LLVMContextRef, Str: *const std::ffi::c_char, Length: std::ffi::c_ulong, DontNullTerminate: std::ffi::c_int) -> LLVMValueRef;

  pub fn LLVMConstString(Str: *const std::ffi::c_char, Length: std::ffi::c_uint, DontNullTerminate: std::ffi::c_int) -> LLVMValueRef;

  pub fn LLVMIsConstantString(c: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMGetAsString(c: LLVMValueRef, Length: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMConstStructInContext(C: LLVMContextRef, ConstantVals: *mut LLVMValueRef, Count: std::ffi::c_uint, Packed: std::ffi::c_int) -> LLVMValueRef;

  pub fn LLVMConstStruct(ConstantVals: *mut LLVMValueRef, Count: std::ffi::c_uint, Packed: std::ffi::c_int) -> LLVMValueRef;

  pub fn LLVMConstArray(ElementTy: LLVMTypeRef, ConstantVals: *mut LLVMValueRef, Length: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMConstArray2(ElementTy: LLVMTypeRef, ConstantVals: *mut LLVMValueRef, Length: std::ffi::c_ulong) -> LLVMValueRef;

  pub fn LLVMConstNamedStruct(StructTy: LLVMTypeRef, ConstantVals: *mut LLVMValueRef, Count: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMGetAggregateElement(C: LLVMValueRef, Idx: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMGetElementAsConstant(C: LLVMValueRef, idx: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMConstVector(ScalarConstantVals: *mut LLVMValueRef, Size: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMConstantPtrAuth(Ptr: LLVMValueRef, Key: LLVMValueRef, Disc: LLVMValueRef, AddrDisc: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetConstOpcode(ConstantVal: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMAlignOf(Ty: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMSizeOf(Ty: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMConstNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstNSWNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstNUWNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstNot(ConstantVal: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstNSWAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstNUWAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstNSWSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstNUWSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstNSWMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstNUWMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstXor(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstGEP2(Ty: LLVMTypeRef, ConstantVal: LLVMValueRef, ConstantIndices: *mut LLVMValueRef, NumIndices: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMConstInBoundsGEP2(Ty: LLVMTypeRef, ConstantVal: LLVMValueRef, ConstantIndices: *mut LLVMValueRef, NumIndices: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMConstGEPWithNoWrapFlags(Ty: LLVMTypeRef, ConstantVal: LLVMValueRef, ConstantIndices: *mut LLVMValueRef, NumIndices: std::ffi::c_uint, NoWrapFlags: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMConstTrunc(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMConstPtrToInt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMConstIntToPtr(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMConstBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMConstAddrSpaceCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMConstTruncOrBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMConstPointerCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;

  pub fn LLVMConstExtractElement(VectorConstant: LLVMValueRef, IndexConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstInsertElement(VectorConstant: LLVMValueRef, ElementValueConstant: LLVMValueRef, IndexConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMConstShuffleVector(VectorAConstant: LLVMValueRef, VectorBConstant: LLVMValueRef, MaskConstant: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMBlockAddress(F: LLVMValueRef, BB: LLVMBasicBlockRef) -> LLVMValueRef;

  pub fn LLVMGetBlockAddressFunction(BlockAddr: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetBlockAddressBasicBlock(BlockAddr: LLVMValueRef) -> LLVMBasicBlockRef;

  pub fn LLVMConstInlineAsm(Ty: LLVMTypeRef, AsmString: *const std::ffi::c_char, Constraints: *const std::ffi::c_char, HasSideEffects: std::ffi::c_int, IsAlignStack: std::ffi::c_int) -> LLVMValueRef;

  pub fn LLVMGetGlobalParent(Global: LLVMValueRef) -> LLVMModuleRef;

  pub fn LLVMIsDeclaration(Global: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMGetLinkage(Global: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetLinkage(Global: LLVMValueRef, Linkage: std::ffi::c_uint) -> ();

  pub fn LLVMGetSection(Global: LLVMValueRef) -> *const std::ffi::c_char;

  pub fn LLVMSetSection(Global: LLVMValueRef, Section: *const std::ffi::c_char) -> ();

  pub fn LLVMGetVisibility(Global: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetVisibility(Global: LLVMValueRef, Viz: std::ffi::c_uint) -> ();

  pub fn LLVMGetDLLStorageClass(Global: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetDLLStorageClass(Global: LLVMValueRef, Class: std::ffi::c_uint) -> ();

  pub fn LLVMGetUnnamedAddress(Global: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetUnnamedAddress(Global: LLVMValueRef, UnnamedAddr: std::ffi::c_uint) -> ();

  pub fn LLVMGlobalGetValueType(Global: LLVMValueRef) -> LLVMTypeRef;

  pub fn LLVMHasUnnamedAddr(Global: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetUnnamedAddr(Global: LLVMValueRef, HasUnnamedAddr: std::ffi::c_int) -> ();

  pub fn LLVMGetAlignment(V: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetAlignment(V: LLVMValueRef, Bytes: std::ffi::c_uint) -> ();

  pub fn LLVMGlobalSetMetadata(Global: LLVMValueRef, Kind: std::ffi::c_uint, MD: LLVMMetadataRef) -> ();

  pub fn LLVMGlobalEraseMetadata(Global: LLVMValueRef, Kind: std::ffi::c_uint) -> ();

  pub fn LLVMGlobalClearMetadata(Global: LLVMValueRef) -> ();

  pub fn LLVMGlobalCopyAllMetadata(Value: LLVMValueRef, NumEntries: *mut std::ffi::c_ulong) -> *mut LLVMValueMetadataEntry;

  pub fn LLVMDisposeValueMetadataEntries(Entries: *mut LLVMValueMetadataEntry) -> ();

  pub fn LLVMValueMetadataEntriesGetKind(Entries: *mut LLVMValueMetadataEntry, Index: std::ffi::c_uint) -> std::ffi::c_uint;

  pub fn LLVMValueMetadataEntriesGetMetadata(Entries: *mut LLVMValueMetadataEntry, Index: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMAddGlobal(M: LLVMModuleRef, Ty: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMAddGlobalInAddressSpace(M: LLVMModuleRef, Ty: LLVMTypeRef, Name: *const std::ffi::c_char, AddressSpace: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMGetNamedGlobal(M: LLVMModuleRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMGetNamedGlobalWithLength(M: LLVMModuleRef, Name: *const std::ffi::c_char, Length: std::ffi::c_ulong) -> LLVMValueRef;

  pub fn LLVMGetFirstGlobal(M: LLVMModuleRef) -> LLVMValueRef;

  pub fn LLVMGetLastGlobal(M: LLVMModuleRef) -> LLVMValueRef;

  pub fn LLVMGetNextGlobal(GlobalVar: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetPreviousGlobal(GlobalVar: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMDeleteGlobal(GlobalVar: LLVMValueRef) -> ();

  pub fn LLVMGetInitializer(GlobalVar: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMSetInitializer(GlobalVar: LLVMValueRef, ConstantVal: LLVMValueRef) -> ();

  pub fn LLVMIsThreadLocal(GlobalVar: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetThreadLocal(GlobalVar: LLVMValueRef, IsThreadLocal: std::ffi::c_int) -> ();

  pub fn LLVMIsGlobalConstant(GlobalVar: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetGlobalConstant(GlobalVar: LLVMValueRef, IsConstant: std::ffi::c_int) -> ();

  pub fn LLVMGetThreadLocalMode(GlobalVar: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetThreadLocalMode(GlobalVar: LLVMValueRef, Mode: std::ffi::c_uint) -> ();

  pub fn LLVMIsExternallyInitialized(GlobalVar: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetExternallyInitialized(GlobalVar: LLVMValueRef, IsExtInit: std::ffi::c_int) -> ();

  pub fn LLVMAddAlias2(M: LLVMModuleRef, ValueTy: LLVMTypeRef, AddrSpace: std::ffi::c_uint, Aliasee: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMGetNamedGlobalAlias(M: LLVMModuleRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong) -> LLVMValueRef;

  pub fn LLVMGetFirstGlobalAlias(M: LLVMModuleRef) -> LLVMValueRef;

  pub fn LLVMGetLastGlobalAlias(M: LLVMModuleRef) -> LLVMValueRef;

  pub fn LLVMGetNextGlobalAlias(GA: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetPreviousGlobalAlias(GA: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMAliasGetAliasee(Alias: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMAliasSetAliasee(Alias: LLVMValueRef, Aliasee: LLVMValueRef) -> ();

  pub fn LLVMDeleteFunction(Fn: LLVMValueRef) -> ();

  pub fn LLVMHasPersonalityFn(Fn: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMGetPersonalityFn(Fn: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMSetPersonalityFn(Fn: LLVMValueRef, PersonalityFn: LLVMValueRef) -> ();

  pub fn LLVMLookupIntrinsicID(Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong) -> std::ffi::c_uint;

  pub fn LLVMGetIntrinsicID(Fn: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetIntrinsicDeclaration(Mod: LLVMModuleRef, ID: std::ffi::c_uint, ParamTypes: *mut LLVMTypeRef, ParamCount: std::ffi::c_ulong) -> LLVMValueRef;

  pub fn LLVMIntrinsicGetType(Ctx: LLVMContextRef, ID: std::ffi::c_uint, ParamTypes: *mut LLVMTypeRef, ParamCount: std::ffi::c_ulong) -> LLVMTypeRef;

  pub fn LLVMIntrinsicGetName(ID: std::ffi::c_uint, NameLength: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMIntrinsicCopyOverloadedName(ID: std::ffi::c_uint, ParamTypes: *mut LLVMTypeRef, ParamCount: std::ffi::c_ulong, NameLength: *mut std::ffi::c_ulong) -> *mut std::ffi::c_char;

  pub fn LLVMIntrinsicCopyOverloadedName2(Mod: LLVMModuleRef, ID: std::ffi::c_uint, ParamTypes: *mut LLVMTypeRef, ParamCount: std::ffi::c_ulong, NameLength: *mut std::ffi::c_ulong) -> *mut std::ffi::c_char;

  pub fn LLVMIntrinsicIsOverloaded(ID: std::ffi::c_uint) -> std::ffi::c_int;

  pub fn LLVMGetFunctionCallConv(Fn: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetFunctionCallConv(Fn: LLVMValueRef, CC: std::ffi::c_uint) -> ();

  pub fn LLVMGetGC(Fn: LLVMValueRef) -> *const std::ffi::c_char;

  pub fn LLVMSetGC(Fn: LLVMValueRef, Name: *const std::ffi::c_char) -> ();

  pub fn LLVMGetPrefixData(Fn: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMHasPrefixData(Fn: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetPrefixData(Fn: LLVMValueRef, prefixData: LLVMValueRef) -> ();

  pub fn LLVMGetPrologueData(Fn: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMHasPrologueData(Fn: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetPrologueData(Fn: LLVMValueRef, prologueData: LLVMValueRef) -> ();

  pub fn LLVMAddAttributeAtIndex(F: LLVMValueRef, Idx: std::ffi::c_uint, A: LLVMAttributeRef) -> ();

  pub fn LLVMGetAttributeCountAtIndex(F: LLVMValueRef, Idx: std::ffi::c_uint) -> std::ffi::c_uint;

  pub fn LLVMGetAttributesAtIndex(F: LLVMValueRef, Idx: std::ffi::c_uint, Attrs: *mut LLVMAttributeRef) -> ();

  pub fn LLVMGetEnumAttributeAtIndex(F: LLVMValueRef, Idx: std::ffi::c_uint, KindID: std::ffi::c_uint) -> LLVMAttributeRef;

  pub fn LLVMGetStringAttributeAtIndex(F: LLVMValueRef, Idx: std::ffi::c_uint, K: *const std::ffi::c_char, KLen: std::ffi::c_uint) -> LLVMAttributeRef;

  pub fn LLVMRemoveEnumAttributeAtIndex(F: LLVMValueRef, Idx: std::ffi::c_uint, KindID: std::ffi::c_uint) -> ();

  pub fn LLVMRemoveStringAttributeAtIndex(F: LLVMValueRef, Idx: std::ffi::c_uint, K: *const std::ffi::c_char, KLen: std::ffi::c_uint) -> ();

  pub fn LLVMAddTargetDependentFunctionAttr(Fn: LLVMValueRef, A: *const std::ffi::c_char, V: *const std::ffi::c_char) -> ();

  pub fn LLVMCountParams(Fn: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetParams(Fn: LLVMValueRef, Params: *mut LLVMValueRef) -> ();

  pub fn LLVMGetParam(Fn: LLVMValueRef, Index: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMGetParamParent(Inst: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetFirstParam(Fn: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetLastParam(Fn: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetNextParam(Arg: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetPreviousParam(Arg: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMSetParamAlignment(Arg: LLVMValueRef, Align: std::ffi::c_uint) -> ();

  pub fn LLVMAddGlobalIFunc(M: LLVMModuleRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, Ty: LLVMTypeRef, AddrSpace: std::ffi::c_uint, Resolver: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetNamedGlobalIFunc(M: LLVMModuleRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong) -> LLVMValueRef;

  pub fn LLVMGetFirstGlobalIFunc(M: LLVMModuleRef) -> LLVMValueRef;

  pub fn LLVMGetLastGlobalIFunc(M: LLVMModuleRef) -> LLVMValueRef;

  pub fn LLVMGetNextGlobalIFunc(IFunc: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetPreviousGlobalIFunc(IFunc: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetGlobalIFuncResolver(IFunc: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMSetGlobalIFuncResolver(IFunc: LLVMValueRef, Resolver: LLVMValueRef) -> ();

  pub fn LLVMEraseGlobalIFunc(IFunc: LLVMValueRef) -> ();

  pub fn LLVMRemoveGlobalIFunc(IFunc: LLVMValueRef) -> ();

  pub fn LLVMMDStringInContext2(C: LLVMContextRef, Str: *const std::ffi::c_char, SLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMMDNodeInContext2(C: LLVMContextRef, MDs: *mut LLVMMetadataRef, Count: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMMetadataAsValue(C: LLVMContextRef, MD: LLVMMetadataRef) -> LLVMValueRef;

  pub fn LLVMValueAsMetadata(Val: LLVMValueRef) -> LLVMMetadataRef;

  pub fn LLVMGetMDString(V: LLVMValueRef, Length: *mut std::ffi::c_uint) -> *const std::ffi::c_char;

  pub fn LLVMGetMDNodeNumOperands(V: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetMDNodeOperands(V: LLVMValueRef, Dest: *mut LLVMValueRef) -> ();

  pub fn LLVMReplaceMDNodeOperandWith(V: LLVMValueRef, Index: std::ffi::c_uint, Replacement: LLVMMetadataRef) -> ();

  pub fn LLVMMDStringInContext(C: LLVMContextRef, Str: *const std::ffi::c_char, SLen: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMMDString(Str: *const std::ffi::c_char, SLen: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMMDNodeInContext(C: LLVMContextRef, Vals: *mut LLVMValueRef, Count: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMMDNode(Vals: *mut LLVMValueRef, Count: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMCreateOperandBundle(Tag: *const std::ffi::c_char, TagLen: std::ffi::c_ulong, Args: *mut LLVMValueRef, NumArgs: std::ffi::c_uint) -> LLVMOperandBundleRef;

  pub fn LLVMDisposeOperandBundle(Bundle: LLVMOperandBundleRef) -> ();

  pub fn LLVMGetOperandBundleTag(Bundle: LLVMOperandBundleRef, Len: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMGetNumOperandBundleArgs(Bundle: LLVMOperandBundleRef) -> std::ffi::c_uint;

  pub fn LLVMGetOperandBundleArgAtIndex(Bundle: LLVMOperandBundleRef, Index: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMBasicBlockAsValue(BB: LLVMBasicBlockRef) -> LLVMValueRef;

  pub fn LLVMValueIsBasicBlock(Val: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMValueAsBasicBlock(Val: LLVMValueRef) -> LLVMBasicBlockRef;

  pub fn LLVMGetBasicBlockName(BB: LLVMBasicBlockRef) -> *const std::ffi::c_char;

  pub fn LLVMGetBasicBlockParent(BB: LLVMBasicBlockRef) -> LLVMValueRef;

  pub fn LLVMGetBasicBlockTerminator(BB: LLVMBasicBlockRef) -> LLVMValueRef;

  pub fn LLVMCountBasicBlocks(Fn: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetBasicBlocks(Fn: LLVMValueRef, BasicBlocks: *mut LLVMBasicBlockRef) -> ();

  pub fn LLVMGetFirstBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;

  pub fn LLVMGetLastBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;

  pub fn LLVMGetNextBasicBlock(BB: LLVMBasicBlockRef) -> LLVMBasicBlockRef;

  pub fn LLVMGetPreviousBasicBlock(BB: LLVMBasicBlockRef) -> LLVMBasicBlockRef;

  pub fn LLVMGetEntryBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;

  pub fn LLVMInsertExistingBasicBlockAfterInsertBlock(Builder: LLVMBuilderRef, BB: LLVMBasicBlockRef) -> ();

  pub fn LLVMAppendExistingBasicBlock(Fn: LLVMValueRef, BB: LLVMBasicBlockRef) -> ();

  pub fn LLVMCreateBasicBlockInContext(C: LLVMContextRef, Name: *const std::ffi::c_char) -> LLVMBasicBlockRef;

  pub fn LLVMAppendBasicBlockInContext(C: LLVMContextRef, Fn: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMBasicBlockRef;

  pub fn LLVMAppendBasicBlock(Fn: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMBasicBlockRef;

  pub fn LLVMInsertBasicBlockInContext(C: LLVMContextRef, BB: LLVMBasicBlockRef, Name: *const std::ffi::c_char) -> LLVMBasicBlockRef;

  pub fn LLVMInsertBasicBlock(InsertBeforeBB: LLVMBasicBlockRef, Name: *const std::ffi::c_char) -> LLVMBasicBlockRef;

  pub fn LLVMDeleteBasicBlock(BB: LLVMBasicBlockRef) -> ();

  pub fn LLVMRemoveBasicBlockFromParent(BB: LLVMBasicBlockRef) -> ();

  pub fn LLVMMoveBasicBlockBefore(BB: LLVMBasicBlockRef, MovePos: LLVMBasicBlockRef) -> ();

  pub fn LLVMMoveBasicBlockAfter(BB: LLVMBasicBlockRef, MovePos: LLVMBasicBlockRef) -> ();

  pub fn LLVMGetFirstInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;

  pub fn LLVMGetLastInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;

  pub fn LLVMHasMetadata(Val: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMGetMetadata(Val: LLVMValueRef, KindID: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMSetMetadata(Val: LLVMValueRef, KindID: std::ffi::c_uint, Node: LLVMValueRef) -> ();

  pub fn LLVMInstructionGetAllMetadataOtherThanDebugLoc(Instr: LLVMValueRef, NumEntries: *mut std::ffi::c_ulong) -> *mut LLVMValueMetadataEntry;

  pub fn LLVMGetInstructionParent(Inst: LLVMValueRef) -> LLVMBasicBlockRef;

  pub fn LLVMGetNextInstruction(Inst: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetPreviousInstruction(Inst: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMInstructionRemoveFromParent(Inst: LLVMValueRef) -> ();

  pub fn LLVMInstructionEraseFromParent(Inst: LLVMValueRef) -> ();

  pub fn LLVMDeleteInstruction(Inst: LLVMValueRef) -> ();

  pub fn LLVMGetInstructionOpcode(Inst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetICmpPredicate(Inst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetFCmpPredicate(Inst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMInstructionClone(Inst: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMIsATerminatorInst(Inst: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetFirstDbgRecord(Inst: LLVMValueRef) -> LLVMDbgRecordRef;

  pub fn LLVMGetLastDbgRecord(Inst: LLVMValueRef) -> LLVMDbgRecordRef;

  pub fn LLVMGetNextDbgRecord(DbgRecord: LLVMDbgRecordRef) -> LLVMDbgRecordRef;

  pub fn LLVMGetPreviousDbgRecord(DbgRecord: LLVMDbgRecordRef) -> LLVMDbgRecordRef;

  pub fn LLVMGetNumArgOperands(Instr: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetInstructionCallConv(Instr: LLVMValueRef, CC: std::ffi::c_uint) -> ();

  pub fn LLVMGetInstructionCallConv(Instr: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetInstrParamAlignment(Instr: LLVMValueRef, Idx: std::ffi::c_uint, Align: std::ffi::c_uint) -> ();

  pub fn LLVMAddCallSiteAttribute(C: LLVMValueRef, Idx: std::ffi::c_uint, A: LLVMAttributeRef) -> ();

  pub fn LLVMGetCallSiteAttributeCount(C: LLVMValueRef, Idx: std::ffi::c_uint) -> std::ffi::c_uint;

  pub fn LLVMGetCallSiteAttributes(C: LLVMValueRef, Idx: std::ffi::c_uint, Attrs: *mut LLVMAttributeRef) -> ();

  pub fn LLVMGetCallSiteEnumAttribute(C: LLVMValueRef, Idx: std::ffi::c_uint, KindID: std::ffi::c_uint) -> LLVMAttributeRef;

  pub fn LLVMGetCallSiteStringAttribute(C: LLVMValueRef, Idx: std::ffi::c_uint, K: *const std::ffi::c_char, KLen: std::ffi::c_uint) -> LLVMAttributeRef;

  pub fn LLVMRemoveCallSiteEnumAttribute(C: LLVMValueRef, Idx: std::ffi::c_uint, KindID: std::ffi::c_uint) -> ();

  pub fn LLVMRemoveCallSiteStringAttribute(C: LLVMValueRef, Idx: std::ffi::c_uint, K: *const std::ffi::c_char, KLen: std::ffi::c_uint) -> ();

  pub fn LLVMGetCalledFunctionType(C: LLVMValueRef) -> LLVMTypeRef;

  pub fn LLVMGetCalledValue(Instr: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMGetNumOperandBundles(C: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetOperandBundleAtIndex(C: LLVMValueRef, Index: std::ffi::c_uint) -> LLVMOperandBundleRef;

  pub fn LLVMIsTailCall(CallInst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetTailCall(CallInst: LLVMValueRef, IsTailCall: std::ffi::c_int) -> ();

  pub fn LLVMGetTailCallKind(CallInst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetTailCallKind(CallInst: LLVMValueRef, kind: std::ffi::c_uint) -> ();

  pub fn LLVMGetNormalDest(InvokeInst: LLVMValueRef) -> LLVMBasicBlockRef;

  pub fn LLVMGetUnwindDest(InvokeInst: LLVMValueRef) -> LLVMBasicBlockRef;

  pub fn LLVMSetNormalDest(InvokeInst: LLVMValueRef, B: LLVMBasicBlockRef) -> ();

  pub fn LLVMSetUnwindDest(InvokeInst: LLVMValueRef, B: LLVMBasicBlockRef) -> ();

  pub fn LLVMGetCallBrDefaultDest(CallBr: LLVMValueRef) -> LLVMBasicBlockRef;

  pub fn LLVMGetCallBrNumIndirectDests(CallBr: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetCallBrIndirectDest(CallBr: LLVMValueRef, Idx: std::ffi::c_uint) -> LLVMBasicBlockRef;

  pub fn LLVMGetNumSuccessors(Term: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetSuccessor(Term: LLVMValueRef, i: std::ffi::c_uint) -> LLVMBasicBlockRef;

  pub fn LLVMSetSuccessor(Term: LLVMValueRef, i: std::ffi::c_uint, block: LLVMBasicBlockRef) -> ();

  pub fn LLVMIsConditional(Branch: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMGetCondition(Branch: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMSetCondition(Branch: LLVMValueRef, Cond: LLVMValueRef) -> ();

  pub fn LLVMGetSwitchDefaultDest(SwitchInstr: LLVMValueRef) -> LLVMBasicBlockRef;

  pub fn LLVMGetAllocatedType(Alloca: LLVMValueRef) -> LLVMTypeRef;

  pub fn LLVMIsInBounds(GEP: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetIsInBounds(GEP: LLVMValueRef, InBounds: std::ffi::c_int) -> ();

  pub fn LLVMGetGEPSourceElementType(GEP: LLVMValueRef) -> LLVMTypeRef;

  pub fn LLVMGEPGetNoWrapFlags(GEP: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGEPSetNoWrapFlags(GEP: LLVMValueRef, NoWrapFlags: std::ffi::c_uint) -> ();

  pub fn LLVMAddIncoming(PhiNode: LLVMValueRef, IncomingValues: *mut LLVMValueRef, IncomingBlocks: *mut LLVMBasicBlockRef, Count: std::ffi::c_uint) -> ();

  pub fn LLVMCountIncoming(PhiNode: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetIncomingValue(PhiNode: LLVMValueRef, Index: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMGetIncomingBlock(PhiNode: LLVMValueRef, Index: std::ffi::c_uint) -> LLVMBasicBlockRef;

  pub fn LLVMGetNumIndices(Inst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetIndices(Inst: LLVMValueRef) -> *const std::ffi::c_uint;

  pub fn LLVMCreateBuilderInContext(C: LLVMContextRef) -> LLVMBuilderRef;

  pub fn LLVMCreateBuilder() -> LLVMBuilderRef;

  pub fn LLVMPositionBuilder(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef, Instr: LLVMValueRef) -> ();

  pub fn LLVMPositionBuilderBeforeDbgRecords(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef, Inst: LLVMValueRef) -> ();

  pub fn LLVMPositionBuilderBefore(Builder: LLVMBuilderRef, Instr: LLVMValueRef) -> ();

  pub fn LLVMPositionBuilderBeforeInstrAndDbgRecords(Builder: LLVMBuilderRef, Instr: LLVMValueRef) -> ();

  pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef) -> ();

  pub fn LLVMGetInsertBlock(Builder: LLVMBuilderRef) -> LLVMBasicBlockRef;

  pub fn LLVMClearInsertionPosition(Builder: LLVMBuilderRef) -> ();

  pub fn LLVMInsertIntoBuilder(Builder: LLVMBuilderRef, Instr: LLVMValueRef) -> ();

  pub fn LLVMInsertIntoBuilderWithName(Builder: LLVMBuilderRef, Instr: LLVMValueRef, Name: *const std::ffi::c_char) -> ();

  pub fn LLVMDisposeBuilder(Builder: LLVMBuilderRef) -> ();

  pub fn LLVMGetCurrentDebugLocation2(Builder: LLVMBuilderRef) -> LLVMMetadataRef;

  pub fn LLVMSetCurrentDebugLocation2(Builder: LLVMBuilderRef, Loc: LLVMMetadataRef) -> ();

  pub fn LLVMSetInstDebugLocation(Builder: LLVMBuilderRef, Inst: LLVMValueRef) -> ();

  pub fn LLVMAddMetadataToInst(Builder: LLVMBuilderRef, Inst: LLVMValueRef) -> ();

  pub fn LLVMBuilderGetDefaultFPMathTag(Builder: LLVMBuilderRef) -> LLVMMetadataRef;

  pub fn LLVMBuilderSetDefaultFPMathTag(Builder: LLVMBuilderRef, FPMathTag: LLVMMetadataRef) -> ();

  pub fn LLVMGetBuilderContext(Builder: LLVMBuilderRef) -> LLVMContextRef;

  pub fn LLVMSetCurrentDebugLocation(Builder: LLVMBuilderRef, L: LLVMValueRef) -> ();

  pub fn LLVMGetCurrentDebugLocation(Builder: LLVMBuilderRef) -> LLVMValueRef;

  pub fn LLVMBuildRetVoid(_: LLVMBuilderRef) -> LLVMValueRef;

  pub fn LLVMBuildRet(_: LLVMBuilderRef, V: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMBuildAggregateRet(_: LLVMBuilderRef, RetVals: *mut LLVMValueRef, N: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMBuildBr(_: LLVMBuilderRef, Dest: LLVMBasicBlockRef) -> LLVMValueRef;

  pub fn LLVMBuildCondBr(_: LLVMBuilderRef, If: LLVMValueRef, Then: LLVMBasicBlockRef, Else: LLVMBasicBlockRef) -> LLVMValueRef;

  pub fn LLVMBuildSwitch(_: LLVMBuilderRef, V: LLVMValueRef, Else: LLVMBasicBlockRef, NumCases: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMBuildIndirectBr(B: LLVMBuilderRef, Addr: LLVMValueRef, NumDests: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMBuildCallBr(B: LLVMBuilderRef, Ty: LLVMTypeRef, Fn: LLVMValueRef, DefaultDest: LLVMBasicBlockRef, IndirectDests: *mut LLVMBasicBlockRef, NumIndirectDests: std::ffi::c_uint, Args: *mut LLVMValueRef, NumArgs: std::ffi::c_uint, Bundles: *mut LLVMOperandBundleRef, NumBundles: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildInvoke2(_: LLVMBuilderRef, Ty: LLVMTypeRef, Fn: LLVMValueRef, Args: *mut LLVMValueRef, NumArgs: std::ffi::c_uint, Then: LLVMBasicBlockRef, Catch: LLVMBasicBlockRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildInvokeWithOperandBundles(_: LLVMBuilderRef, Ty: LLVMTypeRef, Fn: LLVMValueRef, Args: *mut LLVMValueRef, NumArgs: std::ffi::c_uint, Then: LLVMBasicBlockRef, Catch: LLVMBasicBlockRef, Bundles: *mut LLVMOperandBundleRef, NumBundles: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildUnreachable(_: LLVMBuilderRef) -> LLVMValueRef;

  pub fn LLVMBuildResume(B: LLVMBuilderRef, Exn: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMBuildLandingPad(B: LLVMBuilderRef, Ty: LLVMTypeRef, PersFn: LLVMValueRef, NumClauses: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildCleanupRet(B: LLVMBuilderRef, CatchPad: LLVMValueRef, BB: LLVMBasicBlockRef) -> LLVMValueRef;

  pub fn LLVMBuildCatchRet(B: LLVMBuilderRef, CatchPad: LLVMValueRef, BB: LLVMBasicBlockRef) -> LLVMValueRef;

  pub fn LLVMBuildCatchPad(B: LLVMBuilderRef, ParentPad: LLVMValueRef, Args: *mut LLVMValueRef, NumArgs: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildCleanupPad(B: LLVMBuilderRef, ParentPad: LLVMValueRef, Args: *mut LLVMValueRef, NumArgs: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildCatchSwitch(B: LLVMBuilderRef, ParentPad: LLVMValueRef, UnwindBB: LLVMBasicBlockRef, NumHandlers: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMAddCase(Switch: LLVMValueRef, OnVal: LLVMValueRef, Dest: LLVMBasicBlockRef) -> ();

  pub fn LLVMAddDestination(IndirectBr: LLVMValueRef, Dest: LLVMBasicBlockRef) -> ();

  pub fn LLVMGetNumClauses(LandingPad: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetClause(LandingPad: LLVMValueRef, Idx: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMAddClause(LandingPad: LLVMValueRef, ClauseVal: LLVMValueRef) -> ();

  pub fn LLVMIsCleanup(LandingPad: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetCleanup(LandingPad: LLVMValueRef, Val: std::ffi::c_int) -> ();

  pub fn LLVMAddHandler(CatchSwitch: LLVMValueRef, Dest: LLVMBasicBlockRef) -> ();

  pub fn LLVMGetNumHandlers(CatchSwitch: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetHandlers(CatchSwitch: LLVMValueRef, Handlers: *mut LLVMBasicBlockRef) -> ();

  pub fn LLVMGetArgOperand(Funclet: LLVMValueRef, i: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMSetArgOperand(Funclet: LLVMValueRef, i: std::ffi::c_uint, value: LLVMValueRef) -> ();

  pub fn LLVMGetParentCatchSwitch(CatchPad: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMSetParentCatchSwitch(CatchPad: LLVMValueRef, CatchSwitch: LLVMValueRef) -> ();

  pub fn LLVMBuildAdd(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildNSWAdd(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildNUWAdd(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFAdd(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildSub(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildNSWSub(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildNUWSub(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFSub(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildMul(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildNSWMul(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildNUWMul(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFMul(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildUDiv(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildExactUDiv(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildSDiv(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildExactSDiv(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFDiv(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildURem(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildSRem(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFRem(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildShl(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildLShr(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildAShr(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildAnd(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildOr(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildXor(_: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildBinOp(B: LLVMBuilderRef, Op: std::ffi::c_uint, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildNeg(_: LLVMBuilderRef, V: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildNSWNeg(B: LLVMBuilderRef, V: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildNUWNeg(B: LLVMBuilderRef, V: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFNeg(_: LLVMBuilderRef, V: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildNot(_: LLVMBuilderRef, V: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMGetNUW(ArithInst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetNUW(ArithInst: LLVMValueRef, HasNUW: std::ffi::c_int) -> ();

  pub fn LLVMGetNSW(ArithInst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetNSW(ArithInst: LLVMValueRef, HasNSW: std::ffi::c_int) -> ();

  pub fn LLVMGetExact(DivOrShrInst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetExact(DivOrShrInst: LLVMValueRef, IsExact: std::ffi::c_int) -> ();

  pub fn LLVMGetNNeg(NonNegInst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetNNeg(NonNegInst: LLVMValueRef, IsNonNeg: std::ffi::c_int) -> ();

  pub fn LLVMGetFastMathFlags(FPMathInst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetFastMathFlags(FPMathInst: LLVMValueRef, FMF: std::ffi::c_uint) -> ();

  pub fn LLVMCanValueUseFastMathFlags(Inst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMGetIsDisjoint(Inst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetIsDisjoint(Inst: LLVMValueRef, IsDisjoint: std::ffi::c_int) -> ();

  pub fn LLVMBuildMalloc(_: LLVMBuilderRef, Ty: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildArrayMalloc(_: LLVMBuilderRef, Ty: LLVMTypeRef, Val: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildMemSet(B: LLVMBuilderRef, Ptr: LLVMValueRef, Val: LLVMValueRef, Len: LLVMValueRef, Align: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMBuildMemCpy(B: LLVMBuilderRef, Dst: LLVMValueRef, DstAlign: std::ffi::c_uint, Src: LLVMValueRef, SrcAlign: std::ffi::c_uint, Size: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMBuildMemMove(B: LLVMBuilderRef, Dst: LLVMValueRef, DstAlign: std::ffi::c_uint, Src: LLVMValueRef, SrcAlign: std::ffi::c_uint, Size: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMBuildAlloca(_: LLVMBuilderRef, Ty: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildArrayAlloca(_: LLVMBuilderRef, Ty: LLVMTypeRef, Val: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFree(_: LLVMBuilderRef, PointerVal: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMBuildLoad2(_: LLVMBuilderRef, Ty: LLVMTypeRef, PointerVal: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildStore(_: LLVMBuilderRef, Val: LLVMValueRef, Ptr: LLVMValueRef) -> LLVMValueRef;

  pub fn LLVMBuildGEP2(B: LLVMBuilderRef, Ty: LLVMTypeRef, Pointer: LLVMValueRef, Indices: *mut LLVMValueRef, NumIndices: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildInBoundsGEP2(B: LLVMBuilderRef, Ty: LLVMTypeRef, Pointer: LLVMValueRef, Indices: *mut LLVMValueRef, NumIndices: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildGEPWithNoWrapFlags(B: LLVMBuilderRef, Ty: LLVMTypeRef, Pointer: LLVMValueRef, Indices: *mut LLVMValueRef, NumIndices: std::ffi::c_uint, Name: *const std::ffi::c_char, NoWrapFlags: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMBuildStructGEP2(B: LLVMBuilderRef, Ty: LLVMTypeRef, Pointer: LLVMValueRef, Idx: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildGlobalString(B: LLVMBuilderRef, Str: *const std::ffi::c_char, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildGlobalStringPtr(B: LLVMBuilderRef, Str: *const std::ffi::c_char, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMGetVolatile(MemoryAccessInst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetVolatile(MemoryAccessInst: LLVMValueRef, IsVolatile: std::ffi::c_int) -> ();

  pub fn LLVMGetWeak(CmpXchgInst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetWeak(CmpXchgInst: LLVMValueRef, IsWeak: std::ffi::c_int) -> ();

  pub fn LLVMGetOrdering(MemoryAccessInst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetOrdering(MemoryAccessInst: LLVMValueRef, Ordering: std::ffi::c_uint) -> ();

  pub fn LLVMGetAtomicRMWBinOp(AtomicRMWInst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetAtomicRMWBinOp(AtomicRMWInst: LLVMValueRef, BinOp: std::ffi::c_uint) -> ();

  pub fn LLVMBuildTrunc(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildZExt(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildSExt(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFPToUI(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFPToSI(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildUIToFP(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildSIToFP(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFPTrunc(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFPExt(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildPtrToInt(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildIntToPtr(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildBitCast(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildAddrSpaceCast(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildZExtOrBitCast(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildSExtOrBitCast(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildTruncOrBitCast(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildCast(B: LLVMBuilderRef, Op: std::ffi::c_uint, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildPointerCast(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildIntCast2(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, IsSigned: std::ffi::c_int, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFPCast(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildIntCast(_: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMGetCastOpcode(Src: LLVMValueRef, SrcIsSigned: std::ffi::c_int, DestTy: LLVMTypeRef, DestIsSigned: std::ffi::c_int) -> std::ffi::c_uint;

  pub fn LLVMBuildICmp(_: LLVMBuilderRef, Op: std::ffi::c_uint, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFCmp(_: LLVMBuilderRef, Op: std::ffi::c_uint, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildPhi(_: LLVMBuilderRef, Ty: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildCall2(_: LLVMBuilderRef, _: LLVMTypeRef, Fn: LLVMValueRef, Args: *mut LLVMValueRef, NumArgs: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildCallWithOperandBundles(_: LLVMBuilderRef, _: LLVMTypeRef, Fn: LLVMValueRef, Args: *mut LLVMValueRef, NumArgs: std::ffi::c_uint, Bundles: *mut LLVMOperandBundleRef, NumBundles: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildSelect(_: LLVMBuilderRef, If: LLVMValueRef, Then: LLVMValueRef, Else: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildVAArg(_: LLVMBuilderRef, List: LLVMValueRef, Ty: LLVMTypeRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildExtractElement(_: LLVMBuilderRef, VecVal: LLVMValueRef, Index: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildInsertElement(_: LLVMBuilderRef, VecVal: LLVMValueRef, EltVal: LLVMValueRef, Index: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildShuffleVector(_: LLVMBuilderRef, V1: LLVMValueRef, V2: LLVMValueRef, Mask: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildExtractValue(_: LLVMBuilderRef, AggVal: LLVMValueRef, Index: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildInsertValue(_: LLVMBuilderRef, AggVal: LLVMValueRef, EltVal: LLVMValueRef, Index: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFreeze(_: LLVMBuilderRef, Val: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildIsNull(_: LLVMBuilderRef, Val: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildIsNotNull(_: LLVMBuilderRef, Val: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildPtrDiff2(_: LLVMBuilderRef, ElemTy: LLVMTypeRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFence(B: LLVMBuilderRef, ordering: std::ffi::c_uint, singleThread: std::ffi::c_int, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildFenceSyncScope(B: LLVMBuilderRef, ordering: std::ffi::c_uint, SSID: std::ffi::c_uint, Name: *const std::ffi::c_char) -> LLVMValueRef;

  pub fn LLVMBuildAtomicRMW(B: LLVMBuilderRef, op: std::ffi::c_uint, PTR: LLVMValueRef, Val: LLVMValueRef, ordering: std::ffi::c_uint, singleThread: std::ffi::c_int) -> LLVMValueRef;

  pub fn LLVMBuildAtomicRMWSyncScope(B: LLVMBuilderRef, op: std::ffi::c_uint, PTR: LLVMValueRef, Val: LLVMValueRef, ordering: std::ffi::c_uint, SSID: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMBuildAtomicCmpXchg(B: LLVMBuilderRef, Ptr: LLVMValueRef, Cmp: LLVMValueRef, New: LLVMValueRef, SuccessOrdering: std::ffi::c_uint, FailureOrdering: std::ffi::c_uint, SingleThread: std::ffi::c_int) -> LLVMValueRef;

  pub fn LLVMBuildAtomicCmpXchgSyncScope(B: LLVMBuilderRef, Ptr: LLVMValueRef, Cmp: LLVMValueRef, New: LLVMValueRef, SuccessOrdering: std::ffi::c_uint, FailureOrdering: std::ffi::c_uint, SSID: std::ffi::c_uint) -> LLVMValueRef;

  pub fn LLVMGetNumMaskElements(ShuffleVectorInst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMGetUndefMaskElem() -> std::ffi::c_int;

  pub fn LLVMGetMaskValue(ShuffleVectorInst: LLVMValueRef, Elt: std::ffi::c_uint) -> std::ffi::c_int;

  pub fn LLVMIsAtomicSingleThread(AtomicInst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMSetAtomicSingleThread(AtomicInst: LLVMValueRef, SingleThread: std::ffi::c_int) -> ();

  pub fn LLVMIsAtomic(Inst: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMGetAtomicSyncScopeID(AtomicInst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetAtomicSyncScopeID(AtomicInst: LLVMValueRef, SSID: std::ffi::c_uint) -> ();

  pub fn LLVMGetCmpXchgSuccessOrdering(CmpXchgInst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetCmpXchgSuccessOrdering(CmpXchgInst: LLVMValueRef, Ordering: std::ffi::c_uint) -> ();

  pub fn LLVMGetCmpXchgFailureOrdering(CmpXchgInst: LLVMValueRef) -> std::ffi::c_uint;

  pub fn LLVMSetCmpXchgFailureOrdering(CmpXchgInst: LLVMValueRef, Ordering: std::ffi::c_uint) -> ();

  pub fn LLVMCreateModuleProviderForExistingModule(M: LLVMModuleRef) -> LLVMModuleProviderRef;

  pub fn LLVMDisposeModuleProvider(M: LLVMModuleProviderRef) -> ();

  pub fn LLVMCreateMemoryBufferWithContentsOfFile(Path: *const std::ffi::c_char, OutMemBuf: *mut LLVMMemoryBufferRef, OutMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMCreateMemoryBufferWithSTDIN(OutMemBuf: *mut LLVMMemoryBufferRef, OutMessage: *mut *mut std::ffi::c_char) -> std::ffi::c_int;

  pub fn LLVMCreateMemoryBufferWithMemoryRange(InputData: *const std::ffi::c_char, InputDataLength: std::ffi::c_ulong, BufferName: *const std::ffi::c_char, RequiresNullTerminator: std::ffi::c_int) -> LLVMMemoryBufferRef;

  pub fn LLVMCreateMemoryBufferWithMemoryRangeCopy(InputData: *const std::ffi::c_char, InputDataLength: std::ffi::c_ulong, BufferName: *const std::ffi::c_char) -> LLVMMemoryBufferRef;

  pub fn LLVMGetBufferStart(MemBuf: LLVMMemoryBufferRef) -> *const std::ffi::c_char;

  pub fn LLVMGetBufferSize(MemBuf: LLVMMemoryBufferRef) -> std::ffi::c_ulong;

  pub fn LLVMDisposeMemoryBuffer(MemBuf: LLVMMemoryBufferRef) -> ();

  pub fn LLVMCreatePassManager() -> LLVMPassManagerRef;

  pub fn LLVMCreateFunctionPassManagerForModule(M: LLVMModuleRef) -> LLVMPassManagerRef;

  pub fn LLVMCreateFunctionPassManager(MP: LLVMModuleProviderRef) -> LLVMPassManagerRef;

  pub fn LLVMRunPassManager(PM: LLVMPassManagerRef, M: LLVMModuleRef) -> std::ffi::c_int;

  pub fn LLVMInitializeFunctionPassManager(FPM: LLVMPassManagerRef) -> std::ffi::c_int;

  pub fn LLVMRunFunctionPassManager(FPM: LLVMPassManagerRef, F: LLVMValueRef) -> std::ffi::c_int;

  pub fn LLVMFinalizeFunctionPassManager(FPM: LLVMPassManagerRef) -> std::ffi::c_int;

  pub fn LLVMDisposePassManager(PM: LLVMPassManagerRef) -> ();

  pub fn LLVMStartMultithreaded() -> std::ffi::c_int;

  pub fn LLVMStopMultithreaded() -> ();

  pub fn LLVMIsMultithreaded() -> std::ffi::c_int;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl FFIVoid_ {
  pub unsafe fn LLVMGetVersion<T0_, T1_, T2_>(Major_:  T0_, Minor_:  T1_, Patch_:  T2_)
  where
     T0_: Into<*mut std::ffi::c_uint>,  T1_: Into<*mut std::ffi::c_uint>,  T2_: Into<*mut std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMGetVersion(Into::<*mut std::ffi::c_uint>::into(Major_), Into::<*mut std::ffi::c_uint>::into(Minor_), Into::<*mut std::ffi::c_uint>::into(Patch_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMCreateMessage<T0_>(Message_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateMessage(Into::<*const std::ffi::c_char>::into(Message_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeMessage<T0_>(Message_:  T0_)
  where
     T0_: Into<*mut std::ffi::c_char>
  {
    unsafe {
      crate::Core::LLVMDisposeMessage(Into::<*mut std::ffi::c_char>::into(Message_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMContextRef> {
  pub unsafe fn LLVMContextCreate()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMContextCreate()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMContextRef> {
  pub unsafe fn LLVMGetGlobalContext()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetGlobalContext()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMContextSetDiagnosticHandler<T0_, T1_, T2_>(C_:  T0_, Handler_:  T1_, DiagnosticContext_:  T2_)
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMDiagnosticHandler>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::Core::LLVMContextSetDiagnosticHandler(Into::<LLVMContextRef>::into(C_), Into::<LLVMDiagnosticHandler>::into(Handler_), Into::<*mut std::ffi::c_void>::into(DiagnosticContext_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDiagnosticHandler> {
  pub unsafe fn LLVMContextGetDiagnosticHandler<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMContextGetDiagnosticHandler(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_void> {
  pub unsafe fn LLVMContextGetDiagnosticContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMContextGetDiagnosticContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMContextSetYieldCallback<T0_, T1_, T2_>(C_:  T0_, Callback_:  T1_, OpaqueHandle_:  T2_)
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMYieldCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::Core::LLVMContextSetYieldCallback(Into::<LLVMContextRef>::into(C_), Into::<LLVMYieldCallback>::into(Callback_), Into::<*mut std::ffi::c_void>::into(OpaqueHandle_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMContextShouldDiscardValueNames<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMContextShouldDiscardValueNames(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMContextSetDiscardValueNames<T0_, T1_>(C_:  T0_, Discard_:  T1_)
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMContextSetDiscardValueNames(Into::<LLVMContextRef>::into(C_), Into::<std::ffi::c_int>::into(Discard_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMContextDispose<T0_>(C_:  T0_)
  where
     T0_: Into<LLVMContextRef>
  {
    unsafe {
      crate::Core::LLVMContextDispose(Into::<LLVMContextRef>::into(C_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMGetDiagInfoDescription<T0_>(DI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMDiagnosticInfoRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetDiagInfoDescription(Into::<LLVMDiagnosticInfoRef>::into(DI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetDiagInfoSeverity<T0_>(DI_:  T0_)-> Tret_
  where
     T0_: Into<LLVMDiagnosticInfoRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetDiagInfoSeverity(Into::<LLVMDiagnosticInfoRef>::into(DI_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetMDKindIDInContext<T0_, T1_, T2_>(C_:  T0_, Name_:  T1_, SLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetMDKindIDInContext(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_uint>::into(SLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetMDKindID<T0_, T1_>(Name_:  T0_, SLen_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetMDKindID(Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_uint>::into(SLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetSyncScopeID<T0_, T1_, T2_>(C_:  T0_, Name_:  T1_, SLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetSyncScopeID(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(SLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetEnumAttributeKindForName<T0_, T1_>(Name_:  T0_, SLen_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetEnumAttributeKindForName(Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(SLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetLastEnumAttributeKind()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLastEnumAttributeKind()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMAttributeRef> {
  pub unsafe fn LLVMCreateEnumAttribute<T0_, T1_, T2_>(C_:  T0_, KindID_:  T1_, Val_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateEnumAttribute(Into::<LLVMContextRef>::into(C_), Into::<std::ffi::c_uint>::into(KindID_), Into::<std::ffi::c_ulong>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetEnumAttributeKind<T0_>(A_:  T0_)-> Tret_
  where
     T0_: Into<LLVMAttributeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetEnumAttributeKind(Into::<LLVMAttributeRef>::into(A_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetEnumAttributeValue<T0_>(A_:  T0_)-> Tret_
  where
     T0_: Into<LLVMAttributeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetEnumAttributeValue(Into::<LLVMAttributeRef>::into(A_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMAttributeRef> {
  pub unsafe fn LLVMCreateTypeAttribute<T0_, T1_, T2_>(C_:  T0_, KindID_:  T1_, type_ref_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateTypeAttribute(Into::<LLVMContextRef>::into(C_), Into::<std::ffi::c_uint>::into(KindID_), Into::<LLVMTypeRef>::into(type_ref_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGetTypeAttributeValue<T0_>(A_:  T0_)-> Tret_
  where
     T0_: Into<LLVMAttributeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTypeAttributeValue(Into::<LLVMAttributeRef>::into(A_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMAttributeRef> {
  pub unsafe fn LLVMCreateConstantRangeAttribute<T0_, T1_, T2_, T3_, T4_>(C_:  T0_, KindID_:  T1_, NumBits_:  T2_, LowerWords_:  T3_, UpperWords_:  T4_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<*const std::ffi::c_ulong>,  T4_: Into<*const std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateConstantRangeAttribute(Into::<LLVMContextRef>::into(C_), Into::<std::ffi::c_uint>::into(KindID_), Into::<std::ffi::c_uint>::into(NumBits_), Into::<*const std::ffi::c_ulong>::into(LowerWords_), Into::<*const std::ffi::c_ulong>::into(UpperWords_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMAttributeRef> {
  pub unsafe fn LLVMCreateStringAttribute<T0_, T1_, T2_, T3_, T4_>(C_:  T0_, K_:  T1_, KLength_:  T2_, V_:  T3_, VLength_:  T4_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<*const std::ffi::c_char>,  T4_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateStringAttribute(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(K_), Into::<std::ffi::c_uint>::into(KLength_), Into::<*const std::ffi::c_char>::into(V_), Into::<std::ffi::c_uint>::into(VLength_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetStringAttributeKind<T0_, T1_>(A_:  T0_, Length_:  T1_)-> Tret_
  where
     T0_: Into<LLVMAttributeRef>,  T1_: Into<*mut std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetStringAttributeKind(Into::<LLVMAttributeRef>::into(A_), Into::<*mut std::ffi::c_uint>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetStringAttributeValue<T0_, T1_>(A_:  T0_, Length_:  T1_)-> Tret_
  where
     T0_: Into<LLVMAttributeRef>,  T1_: Into<*mut std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetStringAttributeValue(Into::<LLVMAttributeRef>::into(A_), Into::<*mut std::ffi::c_uint>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsEnumAttribute<T0_>(A_:  T0_)-> Tret_
  where
     T0_: Into<LLVMAttributeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsEnumAttribute(Into::<LLVMAttributeRef>::into(A_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsStringAttribute<T0_>(A_:  T0_)-> Tret_
  where
     T0_: Into<LLVMAttributeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsStringAttribute(Into::<LLVMAttributeRef>::into(A_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsTypeAttribute<T0_>(A_:  T0_)-> Tret_
  where
     T0_: Into<LLVMAttributeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsTypeAttribute(Into::<LLVMAttributeRef>::into(A_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGetTypeByName2<T0_, T1_>(C_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTypeByName2(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMModuleRef> {
  pub unsafe fn LLVMModuleCreateWithName<T0_>(ModuleID_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMModuleCreateWithName(Into::<*const std::ffi::c_char>::into(ModuleID_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMModuleRef> {
  pub unsafe fn LLVMModuleCreateWithNameInContext<T0_, T1_>(ModuleID_:  T0_, C_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMModuleCreateWithNameInContext(Into::<*const std::ffi::c_char>::into(ModuleID_), Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMModuleRef> {
  pub unsafe fn LLVMCloneModule<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCloneModule(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeModule<T0_>(M_:  T0_)
  where
     T0_: Into<LLVMModuleRef>
  {
    unsafe {
      crate::Core::LLVMDisposeModule(Into::<LLVMModuleRef>::into(M_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsNewDbgInfoFormat<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsNewDbgInfoFormat(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetIsNewDbgInfoFormat<T0_, T1_>(M_:  T0_, UseNewFormat_:  T1_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetIsNewDbgInfoFormat(Into::<LLVMModuleRef>::into(M_), Into::<std::ffi::c_int>::into(UseNewFormat_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetModuleIdentifier<T0_, T1_>(M_:  T0_, Len_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetModuleIdentifier(Into::<LLVMModuleRef>::into(M_), Into::<*mut std::ffi::c_ulong>::into(Len_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetModuleIdentifier<T0_, T1_, T2_>(M_:  T0_, Ident_:  T1_, Len_:  T2_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::Core::LLVMSetModuleIdentifier(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Ident_), Into::<std::ffi::c_ulong>::into(Len_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetSourceFileName<T0_, T1_>(M_:  T0_, Len_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetSourceFileName(Into::<LLVMModuleRef>::into(M_), Into::<*mut std::ffi::c_ulong>::into(Len_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetSourceFileName<T0_, T1_, T2_>(M_:  T0_, Name_:  T1_, Len_:  T2_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::Core::LLVMSetSourceFileName(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(Len_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetDataLayoutStr<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetDataLayoutStr(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetDataLayout<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetDataLayout(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetDataLayout<T0_, T1_>(M_:  T0_, DataLayoutStr_:  T1_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Core::LLVMSetDataLayout(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(DataLayoutStr_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetTarget<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTarget(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetTarget<T0_, T1_>(M_:  T0_, Triple_:  T1_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Core::LLVMSetTarget(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Triple_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut LLVMModuleFlagEntry> {
  pub unsafe fn LLVMCopyModuleFlagsMetadata<T0_, T1_>(M_:  T0_, Len_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCopyModuleFlagsMetadata(Into::<LLVMModuleRef>::into(M_), Into::<*mut std::ffi::c_ulong>::into(Len_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeModuleFlagsMetadata<T0_>(Entries_:  T0_)
  where
     T0_: Into<*mut LLVMModuleFlagEntry>
  {
    unsafe {
      crate::Core::LLVMDisposeModuleFlagsMetadata(Into::<*mut LLVMModuleFlagEntry>::into(Entries_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMModuleFlagEntriesGetFlagBehavior<T0_, T1_>(Entries_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<*mut LLVMModuleFlagEntry>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMModuleFlagEntriesGetFlagBehavior(Into::<*mut LLVMModuleFlagEntry>::into(Entries_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMModuleFlagEntriesGetKey<T0_, T1_, T2_>(Entries_:  T0_, Index_:  T1_, Len_:  T2_)-> Tret_
  where
     T0_: Into<*mut LLVMModuleFlagEntry>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMModuleFlagEntriesGetKey(Into::<*mut LLVMModuleFlagEntry>::into(Entries_), Into::<std::ffi::c_uint>::into(Index_), Into::<*mut std::ffi::c_ulong>::into(Len_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMModuleFlagEntriesGetMetadata<T0_, T1_>(Entries_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<*mut LLVMModuleFlagEntry>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMModuleFlagEntriesGetMetadata(Into::<*mut LLVMModuleFlagEntry>::into(Entries_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMGetModuleFlag<T0_, T1_, T2_>(M_:  T0_, Key_:  T1_, KeyLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetModuleFlag(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Key_), Into::<std::ffi::c_ulong>::into(KeyLen_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddModuleFlag<T0_, T1_, T2_, T3_, T4_>(M_:  T0_, Behavior_:  T1_, Key_:  T2_, KeyLen_:  T3_, Val_:  T4_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>
  {
    unsafe {
      crate::Core::LLVMAddModuleFlag(Into::<LLVMModuleRef>::into(M_), Into::<std::ffi::c_uint>::into(Behavior_), Into::<*const std::ffi::c_char>::into(Key_), Into::<std::ffi::c_ulong>::into(KeyLen_), Into::<LLVMMetadataRef>::into(Val_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDumpModule<T0_>(M_:  T0_)
  where
     T0_: Into<LLVMModuleRef>
  {
    unsafe {
      crate::Core::LLVMDumpModule(Into::<LLVMModuleRef>::into(M_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMPrintModuleToFile<T0_, T1_, T2_>(M_:  T0_, Filename_:  T1_, ErrorMessage_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMPrintModuleToFile(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Filename_), Into::<*mut *mut std::ffi::c_char>::into(ErrorMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMPrintModuleToString<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMPrintModuleToString(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetModuleInlineAsm<T0_, T1_>(M_:  T0_, Len_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetModuleInlineAsm(Into::<LLVMModuleRef>::into(M_), Into::<*mut std::ffi::c_ulong>::into(Len_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetModuleInlineAsm2<T0_, T1_, T2_>(M_:  T0_, Asm_:  T1_, Len_:  T2_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::Core::LLVMSetModuleInlineAsm2(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Asm_), Into::<std::ffi::c_ulong>::into(Len_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAppendModuleInlineAsm<T0_, T1_, T2_>(M_:  T0_, Asm_:  T1_, Len_:  T2_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::Core::LLVMAppendModuleInlineAsm(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Asm_), Into::<std::ffi::c_ulong>::into(Len_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetInlineAsm<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_>(Ty_:  T0_, AsmString_:  T1_, AsmStringSize_:  T2_, Constraints_:  T3_, ConstraintsSize_:  T4_, HasSideEffects_:  T5_, IsAlignStack_:  T6_, Dialect_:  T7_, CanThrow_:  T8_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<*const std::ffi::c_char>,  T4_: Into<std::ffi::c_ulong>,  T5_: Into<std::ffi::c_int>,  T6_: Into<std::ffi::c_int>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInlineAsm(Into::<LLVMTypeRef>::into(Ty_), Into::<*const std::ffi::c_char>::into(AsmString_), Into::<std::ffi::c_ulong>::into(AsmStringSize_), Into::<*const std::ffi::c_char>::into(Constraints_), Into::<std::ffi::c_ulong>::into(ConstraintsSize_), Into::<std::ffi::c_int>::into(HasSideEffects_), Into::<std::ffi::c_int>::into(IsAlignStack_), Into::<std::ffi::c_uint>::into(Dialect_), Into::<std::ffi::c_int>::into(CanThrow_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetInlineAsmAsmString<T0_, T1_>(InlineAsmVal_:  T0_, Len_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInlineAsmAsmString(Into::<LLVMValueRef>::into(InlineAsmVal_), Into::<*mut std::ffi::c_ulong>::into(Len_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetInlineAsmConstraintString<T0_, T1_>(InlineAsmVal_:  T0_, Len_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInlineAsmConstraintString(Into::<LLVMValueRef>::into(InlineAsmVal_), Into::<*mut std::ffi::c_ulong>::into(Len_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetInlineAsmDialect<T0_>(InlineAsmVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInlineAsmDialect(Into::<LLVMValueRef>::into(InlineAsmVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGetInlineAsmFunctionType<T0_>(InlineAsmVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInlineAsmFunctionType(Into::<LLVMValueRef>::into(InlineAsmVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetInlineAsmHasSideEffects<T0_>(InlineAsmVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInlineAsmHasSideEffects(Into::<LLVMValueRef>::into(InlineAsmVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetInlineAsmNeedsAlignedStack<T0_>(InlineAsmVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInlineAsmNeedsAlignedStack(Into::<LLVMValueRef>::into(InlineAsmVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetInlineAsmCanUnwind<T0_>(InlineAsmVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInlineAsmCanUnwind(Into::<LLVMValueRef>::into(InlineAsmVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMContextRef> {
  pub unsafe fn LLVMGetModuleContext<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetModuleContext(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGetTypeByName<T0_, T1_>(M_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTypeByName(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMNamedMDNodeRef> {
  pub unsafe fn LLVMGetFirstNamedMetadata<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFirstNamedMetadata(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMNamedMDNodeRef> {
  pub unsafe fn LLVMGetLastNamedMetadata<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLastNamedMetadata(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMNamedMDNodeRef> {
  pub unsafe fn LLVMGetNextNamedMetadata<T0_>(NamedMDNode_:  T0_)-> Tret_
  where
     T0_: Into<LLVMNamedMDNodeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNextNamedMetadata(Into::<LLVMNamedMDNodeRef>::into(NamedMDNode_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMNamedMDNodeRef> {
  pub unsafe fn LLVMGetPreviousNamedMetadata<T0_>(NamedMDNode_:  T0_)-> Tret_
  where
     T0_: Into<LLVMNamedMDNodeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPreviousNamedMetadata(Into::<LLVMNamedMDNodeRef>::into(NamedMDNode_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMNamedMDNodeRef> {
  pub unsafe fn LLVMGetNamedMetadata<T0_, T1_, T2_>(M_:  T0_, Name_:  T1_, NameLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNamedMetadata(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMNamedMDNodeRef> {
  pub unsafe fn LLVMGetOrInsertNamedMetadata<T0_, T1_, T2_>(M_:  T0_, Name_:  T1_, NameLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetOrInsertNamedMetadata(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetNamedMetadataName<T0_, T1_>(NamedMD_:  T0_, NameLen_:  T1_)-> Tret_
  where
     T0_: Into<LLVMNamedMDNodeRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNamedMetadataName(Into::<LLVMNamedMDNodeRef>::into(NamedMD_), Into::<*mut std::ffi::c_ulong>::into(NameLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetNamedMetadataNumOperands<T0_, T1_>(M_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNamedMetadataNumOperands(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGetNamedMetadataOperands<T0_, T1_, T2_>(M_:  T0_, Name_:  T1_, Dest_:  T2_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*mut LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMGetNamedMetadataOperands(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<*mut LLVMValueRef>::into(Dest_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddNamedMetadataOperand<T0_, T1_, T2_>(M_:  T0_, Name_:  T1_, Val_:  T2_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMAddNamedMetadataOperand(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<LLVMValueRef>::into(Val_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetDebugLocDirectory<T0_, T1_>(Val_:  T0_, Length_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetDebugLocDirectory(Into::<LLVMValueRef>::into(Val_), Into::<*mut std::ffi::c_uint>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetDebugLocFilename<T0_, T1_>(Val_:  T0_, Length_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetDebugLocFilename(Into::<LLVMValueRef>::into(Val_), Into::<*mut std::ffi::c_uint>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetDebugLocLine<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetDebugLocLine(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetDebugLocColumn<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetDebugLocColumn(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMAddFunction<T0_, T1_, T2_>(M_:  T0_, Name_:  T1_, FunctionTy_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMAddFunction(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<LLVMTypeRef>::into(FunctionTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNamedFunction<T0_, T1_>(M_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNamedFunction(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNamedFunctionWithLength<T0_, T1_, T2_>(M_:  T0_, Name_:  T1_, Length_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNamedFunctionWithLength(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetFirstFunction<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFirstFunction(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetLastFunction<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLastFunction(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNextFunction<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNextFunction(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetPreviousFunction<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPreviousFunction(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetModuleInlineAsm<T0_, T1_>(M_:  T0_, Asm_:  T1_)
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Core::LLVMSetModuleInlineAsm(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Asm_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetTypeKind<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTypeKind(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMTypeIsSized<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMTypeIsSized(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMContextRef> {
  pub unsafe fn LLVMGetTypeContext<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTypeContext(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDumpType<T0_>(Val_:  T0_)
  where
     T0_: Into<LLVMTypeRef>
  {
    unsafe {
      crate::Core::LLVMDumpType(Into::<LLVMTypeRef>::into(Val_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMPrintTypeToString<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMPrintTypeToString(Into::<LLVMTypeRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt1TypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt1TypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt8TypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt8TypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt16TypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt16TypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt32TypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt32TypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt64TypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt64TypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt128TypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt128TypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMIntTypeInContext<T0_, T1_>(C_:  T0_, NumBits_:  T1_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIntTypeInContext(Into::<LLVMContextRef>::into(C_), Into::<std::ffi::c_uint>::into(NumBits_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt1Type()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt1Type()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt8Type()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt8Type()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt16Type()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt16Type()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt32Type()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt32Type()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt64Type()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt64Type()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMInt128Type()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInt128Type()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMIntType<T0_>(NumBits_:  T0_)-> Tret_
  where
     T0_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIntType(Into::<std::ffi::c_uint>::into(NumBits_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetIntTypeWidth<T0_>(IntegerTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetIntTypeWidth(Into::<LLVMTypeRef>::into(IntegerTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMHalfTypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMHalfTypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMBFloatTypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBFloatTypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMFloatTypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMFloatTypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMDoubleTypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMDoubleTypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMX86FP80TypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMX86FP80TypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMFP128TypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMFP128TypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMPPCFP128TypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMPPCFP128TypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMHalfType()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMHalfType()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMBFloatType()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBFloatType()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMFloatType()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMFloatType()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMDoubleType()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMDoubleType()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMX86FP80Type()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMX86FP80Type()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMFP128Type()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMFP128Type()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMPPCFP128Type()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMPPCFP128Type()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMFunctionType<T0_, T1_, T2_, T3_>(ReturnType_:  T0_, ParamTypes_:  T1_, ParamCount_:  T2_, IsVarArg_:  T3_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*mut LLVMTypeRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMFunctionType(Into::<LLVMTypeRef>::into(ReturnType_), Into::<*mut LLVMTypeRef>::into(ParamTypes_), Into::<std::ffi::c_uint>::into(ParamCount_), Into::<std::ffi::c_int>::into(IsVarArg_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsFunctionVarArg<T0_>(FunctionTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsFunctionVarArg(Into::<LLVMTypeRef>::into(FunctionTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGetReturnType<T0_>(FunctionTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetReturnType(Into::<LLVMTypeRef>::into(FunctionTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMCountParamTypes<T0_>(FunctionTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCountParamTypes(Into::<LLVMTypeRef>::into(FunctionTy_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGetParamTypes<T0_, T1_>(FunctionTy_:  T0_, Dest_:  T1_)
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*mut LLVMTypeRef>
  {
    unsafe {
      crate::Core::LLVMGetParamTypes(Into::<LLVMTypeRef>::into(FunctionTy_), Into::<*mut LLVMTypeRef>::into(Dest_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMStructTypeInContext<T0_, T1_, T2_, T3_>(C_:  T0_, ElementTypes_:  T1_, ElementCount_:  T2_, Packed_:  T3_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*mut LLVMTypeRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMStructTypeInContext(Into::<LLVMContextRef>::into(C_), Into::<*mut LLVMTypeRef>::into(ElementTypes_), Into::<std::ffi::c_uint>::into(ElementCount_), Into::<std::ffi::c_int>::into(Packed_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMStructType<T0_, T1_, T2_>(ElementTypes_:  T0_, ElementCount_:  T1_, Packed_:  T2_)-> Tret_
  where
     T0_: Into<*mut LLVMTypeRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMStructType(Into::<*mut LLVMTypeRef>::into(ElementTypes_), Into::<std::ffi::c_uint>::into(ElementCount_), Into::<std::ffi::c_int>::into(Packed_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMStructCreateNamed<T0_, T1_>(C_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMStructCreateNamed(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetStructName<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetStructName(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMStructSetBody<T0_, T1_, T2_, T3_>(StructTy_:  T0_, ElementTypes_:  T1_, ElementCount_:  T2_, Packed_:  T3_)
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*mut LLVMTypeRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMStructSetBody(Into::<LLVMTypeRef>::into(StructTy_), Into::<*mut LLVMTypeRef>::into(ElementTypes_), Into::<std::ffi::c_uint>::into(ElementCount_), Into::<std::ffi::c_int>::into(Packed_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMCountStructElementTypes<T0_>(StructTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCountStructElementTypes(Into::<LLVMTypeRef>::into(StructTy_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGetStructElementTypes<T0_, T1_>(StructTy_:  T0_, Dest_:  T1_)
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*mut LLVMTypeRef>
  {
    unsafe {
      crate::Core::LLVMGetStructElementTypes(Into::<LLVMTypeRef>::into(StructTy_), Into::<*mut LLVMTypeRef>::into(Dest_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMStructGetTypeAtIndex<T0_, T1_>(StructTy_:  T0_, i_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMStructGetTypeAtIndex(Into::<LLVMTypeRef>::into(StructTy_), Into::<std::ffi::c_uint>::into(i_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsPackedStruct<T0_>(StructTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsPackedStruct(Into::<LLVMTypeRef>::into(StructTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsOpaqueStruct<T0_>(StructTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsOpaqueStruct(Into::<LLVMTypeRef>::into(StructTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsLiteralStruct<T0_>(StructTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsLiteralStruct(Into::<LLVMTypeRef>::into(StructTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGetElementType<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetElementType(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGetSubtypes<T0_, T1_>(Tp_:  T0_, Arr_:  T1_)
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*mut LLVMTypeRef>
  {
    unsafe {
      crate::Core::LLVMGetSubtypes(Into::<LLVMTypeRef>::into(Tp_), Into::<*mut LLVMTypeRef>::into(Arr_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetNumContainedTypes<T0_>(Tp_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNumContainedTypes(Into::<LLVMTypeRef>::into(Tp_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMArrayType<T0_, T1_>(ElementType_:  T0_, ElementCount_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMArrayType(Into::<LLVMTypeRef>::into(ElementType_), Into::<std::ffi::c_uint>::into(ElementCount_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMArrayType2<T0_, T1_>(ElementType_:  T0_, ElementCount_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMArrayType2(Into::<LLVMTypeRef>::into(ElementType_), Into::<std::ffi::c_ulong>::into(ElementCount_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetArrayLength<T0_>(ArrayTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetArrayLength(Into::<LLVMTypeRef>::into(ArrayTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetArrayLength2<T0_>(ArrayTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetArrayLength2(Into::<LLVMTypeRef>::into(ArrayTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMPointerType<T0_, T1_>(ElementType_:  T0_, AddressSpace_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMPointerType(Into::<LLVMTypeRef>::into(ElementType_), Into::<std::ffi::c_uint>::into(AddressSpace_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMPointerTypeIsOpaque<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMPointerTypeIsOpaque(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMPointerTypeInContext<T0_, T1_>(C_:  T0_, AddressSpace_:  T1_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMPointerTypeInContext(Into::<LLVMContextRef>::into(C_), Into::<std::ffi::c_uint>::into(AddressSpace_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetPointerAddressSpace<T0_>(PointerTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPointerAddressSpace(Into::<LLVMTypeRef>::into(PointerTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMVectorType<T0_, T1_>(ElementType_:  T0_, ElementCount_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMVectorType(Into::<LLVMTypeRef>::into(ElementType_), Into::<std::ffi::c_uint>::into(ElementCount_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMScalableVectorType<T0_, T1_>(ElementType_:  T0_, ElementCount_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMScalableVectorType(Into::<LLVMTypeRef>::into(ElementType_), Into::<std::ffi::c_uint>::into(ElementCount_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetVectorSize<T0_>(VectorTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetVectorSize(Into::<LLVMTypeRef>::into(VectorTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetConstantPtrAuthPointer<T0_>(PtrAuth_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetConstantPtrAuthPointer(Into::<LLVMValueRef>::into(PtrAuth_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetConstantPtrAuthKey<T0_>(PtrAuth_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetConstantPtrAuthKey(Into::<LLVMValueRef>::into(PtrAuth_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetConstantPtrAuthDiscriminator<T0_>(PtrAuth_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetConstantPtrAuthDiscriminator(Into::<LLVMValueRef>::into(PtrAuth_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetConstantPtrAuthAddrDiscriminator<T0_>(PtrAuth_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetConstantPtrAuthAddrDiscriminator(Into::<LLVMValueRef>::into(PtrAuth_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMVoidTypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMVoidTypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMLabelTypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMLabelTypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMX86AMXTypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMX86AMXTypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMTokenTypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMTokenTypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMMetadataTypeInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMMetadataTypeInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMVoidType()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMVoidType()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMLabelType()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMLabelType()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMX86AMXType()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMX86AMXType()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMTargetExtTypeInContext<T0_, T1_, T2_, T3_, T4_, T5_>(C_:  T0_, Name_:  T1_, TypeParams_:  T2_, TypeParamCount_:  T3_, IntParams_:  T4_, IntParamCount_:  T5_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*mut LLVMTypeRef>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<*mut std::ffi::c_uint>,  T5_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMTargetExtTypeInContext(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(Name_), Into::<*mut LLVMTypeRef>::into(TypeParams_), Into::<std::ffi::c_uint>::into(TypeParamCount_), Into::<*mut std::ffi::c_uint>::into(IntParams_), Into::<std::ffi::c_uint>::into(IntParamCount_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetTargetExtTypeName<T0_>(TargetExtTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTargetExtTypeName(Into::<LLVMTypeRef>::into(TargetExtTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetTargetExtTypeNumTypeParams<T0_>(TargetExtTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTargetExtTypeNumTypeParams(Into::<LLVMTypeRef>::into(TargetExtTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGetTargetExtTypeTypeParam<T0_, T1_>(TargetExtTy_:  T0_, Idx_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTargetExtTypeTypeParam(Into::<LLVMTypeRef>::into(TargetExtTy_), Into::<std::ffi::c_uint>::into(Idx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetTargetExtTypeNumIntParams<T0_>(TargetExtTy_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTargetExtTypeNumIntParams(Into::<LLVMTypeRef>::into(TargetExtTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetTargetExtTypeIntParam<T0_, T1_>(TargetExtTy_:  T0_, Idx_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTargetExtTypeIntParam(Into::<LLVMTypeRef>::into(TargetExtTy_), Into::<std::ffi::c_uint>::into(Idx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMTypeOf<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMTypeOf(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetValueKind<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetValueKind(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetValueName2<T0_, T1_>(Val_:  T0_, Length_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetValueName2(Into::<LLVMValueRef>::into(Val_), Into::<*mut std::ffi::c_ulong>::into(Length_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetValueName2<T0_, T1_, T2_>(Val_:  T0_, Name_:  T1_, NameLen_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::Core::LLVMSetValueName2(Into::<LLVMValueRef>::into(Val_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDumpValue<T0_>(Val_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMDumpValue(Into::<LLVMValueRef>::into(Val_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMPrintValueToString<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMPrintValueToString(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMContextRef> {
  pub unsafe fn LLVMGetValueContext<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetValueContext(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMPrintDbgRecordToString<T0_>(Record_:  T0_)-> Tret_
  where
     T0_: Into<LLVMDbgRecordRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMPrintDbgRecordToString(Into::<LLVMDbgRecordRef>::into(Record_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMReplaceAllUsesWith<T0_, T1_>(OldVal_:  T0_, NewVal_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMReplaceAllUsesWith(Into::<LLVMValueRef>::into(OldVal_), Into::<LLVMValueRef>::into(NewVal_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsConstant<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsConstant(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsUndef<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsUndef(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsPoison<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsPoison(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAArgument<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAArgument(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsABasicBlock<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsABasicBlock(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAInlineAsm<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAInlineAsm(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAUser<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAUser(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstant<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstant(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsABlockAddress<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsABlockAddress(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantAggregateZero<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantAggregateZero(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantArray<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantArray(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantDataSequential<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantDataSequential(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantDataArray<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantDataArray(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantDataVector<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantDataVector(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantExpr<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantExpr(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantFP<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantFP(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantInt<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantInt(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantPointerNull<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantPointerNull(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantStruct<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantStruct(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantTokenNone<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantTokenNone(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantVector<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantVector(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAConstantPtrAuth<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAConstantPtrAuth(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAGlobalValue<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAGlobalValue(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAGlobalAlias<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAGlobalAlias(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAGlobalObject<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAGlobalObject(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAFunction<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAFunction(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAGlobalVariable<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAGlobalVariable(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAGlobalIFunc<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAGlobalIFunc(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAUndefValue<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAUndefValue(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAPoisonValue<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAPoisonValue(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAInstruction<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAInstruction(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAUnaryOperator<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAUnaryOperator(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsABinaryOperator<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsABinaryOperator(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsACallInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsACallInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAIntrinsicInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAIntrinsicInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsADbgInfoIntrinsic<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsADbgInfoIntrinsic(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsADbgVariableIntrinsic<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsADbgVariableIntrinsic(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsADbgDeclareInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsADbgDeclareInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsADbgLabelInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsADbgLabelInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAMemIntrinsic<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAMemIntrinsic(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAMemCpyInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAMemCpyInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAMemMoveInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAMemMoveInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAMemSetInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAMemSetInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsACmpInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsACmpInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAFCmpInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAFCmpInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAICmpInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAICmpInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAExtractElementInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAExtractElementInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAGetElementPtrInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAGetElementPtrInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAInsertElementInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAInsertElementInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAInsertValueInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAInsertValueInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsALandingPadInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsALandingPadInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAPHINode<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAPHINode(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsASelectInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsASelectInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAShuffleVectorInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAShuffleVectorInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAStoreInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAStoreInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsABranchInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsABranchInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAIndirectBrInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAIndirectBrInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAInvokeInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAInvokeInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAReturnInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAReturnInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsASwitchInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsASwitchInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAUnreachableInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAUnreachableInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAResumeInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAResumeInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsACleanupReturnInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsACleanupReturnInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsACatchReturnInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsACatchReturnInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsACatchSwitchInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsACatchSwitchInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsACallBrInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsACallBrInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAFuncletPadInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAFuncletPadInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsACatchPadInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsACatchPadInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsACleanupPadInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsACleanupPadInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAUnaryInstruction<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAUnaryInstruction(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAAllocaInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAAllocaInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsACastInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsACastInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAAddrSpaceCastInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAAddrSpaceCastInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsABitCastInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsABitCastInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAFPExtInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAFPExtInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAFPToSIInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAFPToSIInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAFPToUIInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAFPToUIInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAFPTruncInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAFPTruncInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAIntToPtrInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAIntToPtrInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAPtrToIntInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAPtrToIntInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsASExtInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsASExtInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsASIToFPInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsASIToFPInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsATruncInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsATruncInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAUIToFPInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAUIToFPInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAZExtInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAZExtInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAExtractValueInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAExtractValueInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsALoadInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsALoadInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAVAArgInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAVAArgInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAFreezeInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAFreezeInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAAtomicCmpXchgInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAAtomicCmpXchgInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAAtomicRMWInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAAtomicRMWInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAFenceInst<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAFenceInst(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAMDNode<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAMDNode(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAValueAsMetadata<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAValueAsMetadata(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsAMDString<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAMDString(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetValueName<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetValueName(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetValueName<T0_, T1_>(Val_:  T0_, Name_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Core::LLVMSetValueName(Into::<LLVMValueRef>::into(Val_), Into::<*const std::ffi::c_char>::into(Name_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMUseRef> {
  pub unsafe fn LLVMGetFirstUse<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFirstUse(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMUseRef> {
  pub unsafe fn LLVMGetNextUse<T0_>(U_:  T0_)-> Tret_
  where
     T0_: Into<LLVMUseRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNextUse(Into::<LLVMUseRef>::into(U_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetUser<T0_>(U_:  T0_)-> Tret_
  where
     T0_: Into<LLVMUseRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetUser(Into::<LLVMUseRef>::into(U_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetUsedValue<T0_>(U_:  T0_)-> Tret_
  where
     T0_: Into<LLVMUseRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetUsedValue(Into::<LLVMUseRef>::into(U_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetOperand<T0_, T1_>(Val_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetOperand(Into::<LLVMValueRef>::into(Val_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMUseRef> {
  pub unsafe fn LLVMGetOperandUse<T0_, T1_>(Val_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetOperandUse(Into::<LLVMValueRef>::into(Val_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetOperand<T0_, T1_, T2_>(User_:  T0_, Index_:  T1_, Val_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetOperand(Into::<LLVMValueRef>::into(User_), Into::<std::ffi::c_uint>::into(Index_), Into::<LLVMValueRef>::into(Val_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetNumOperands<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNumOperands(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNull<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNull(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstAllOnes<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstAllOnes(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetUndef<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetUndef(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetPoison<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPoison(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsNull<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsNull(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstPointerNull<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstPointerNull(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstInt<T0_, T1_, T2_>(IntTy_:  T0_, N_:  T1_, SignExtend_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_ulonglong>,  T2_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstInt(Into::<LLVMTypeRef>::into(IntTy_), Into::<std::ffi::c_ulonglong>::into(N_), Into::<std::ffi::c_int>::into(SignExtend_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstIntOfArbitraryPrecision<T0_, T1_, T2_>(IntTy_:  T0_, NumWords_:  T1_, Words_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*const std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstIntOfArbitraryPrecision(Into::<LLVMTypeRef>::into(IntTy_), Into::<std::ffi::c_uint>::into(NumWords_), Into::<*const std::ffi::c_ulong>::into(Words_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstIntOfString<T0_, T1_, T2_>(IntTy_:  T0_, Text_:  T1_, Radix_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_uchar>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstIntOfString(Into::<LLVMTypeRef>::into(IntTy_), Into::<*const std::ffi::c_char>::into(Text_), Into::<std::ffi::c_uchar>::into(Radix_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstIntOfStringAndSize<T0_, T1_, T2_, T3_>(IntTy_:  T0_, Text_:  T1_, SLen_:  T2_, Radix_:  T3_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<std::ffi::c_uchar>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstIntOfStringAndSize(Into::<LLVMTypeRef>::into(IntTy_), Into::<*const std::ffi::c_char>::into(Text_), Into::<std::ffi::c_uint>::into(SLen_), Into::<std::ffi::c_uchar>::into(Radix_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstReal<T0_, T1_>(RealTy_:  T0_, N_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<f64>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstReal(Into::<LLVMTypeRef>::into(RealTy_), Into::<f64>::into(N_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstRealOfString<T0_, T1_>(RealTy_:  T0_, Text_:  T1_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstRealOfString(Into::<LLVMTypeRef>::into(RealTy_), Into::<*const std::ffi::c_char>::into(Text_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstRealOfStringAndSize<T0_, T1_, T2_>(RealTy_:  T0_, Text_:  T1_, SLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstRealOfStringAndSize(Into::<LLVMTypeRef>::into(RealTy_), Into::<*const std::ffi::c_char>::into(Text_), Into::<std::ffi::c_uint>::into(SLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulonglong> {
  pub unsafe fn LLVMConstIntGetZExtValue<T0_>(ConstantVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstIntGetZExtValue(Into::<LLVMValueRef>::into(ConstantVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_longlong> {
  pub unsafe fn LLVMConstIntGetSExtValue<T0_>(ConstantVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstIntGetSExtValue(Into::<LLVMValueRef>::into(ConstantVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn LLVMConstRealGetDouble<T0_, T1_>(ConstantVal_:  T0_, losesInfo_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstRealGetDouble(Into::<LLVMValueRef>::into(ConstantVal_), Into::<*mut std::ffi::c_int>::into(losesInfo_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstStringInContext<T0_, T1_, T2_, T3_>(C_:  T0_, Str_:  T1_, Length_:  T2_, DontNullTerminate_:  T3_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstStringInContext(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(Str_), Into::<std::ffi::c_uint>::into(Length_), Into::<std::ffi::c_int>::into(DontNullTerminate_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstStringInContext2<T0_, T1_, T2_, T3_>(C_:  T0_, Str_:  T1_, Length_:  T2_, DontNullTerminate_:  T3_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstStringInContext2(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(Str_), Into::<std::ffi::c_ulong>::into(Length_), Into::<std::ffi::c_int>::into(DontNullTerminate_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstString<T0_, T1_, T2_>(Str_:  T0_, Length_:  T1_, DontNullTerminate_:  T2_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstString(Into::<*const std::ffi::c_char>::into(Str_), Into::<std::ffi::c_uint>::into(Length_), Into::<std::ffi::c_int>::into(DontNullTerminate_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsConstantString<T0_>(c_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsConstantString(Into::<LLVMValueRef>::into(c_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetAsString<T0_, T1_>(c_:  T0_, Length_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetAsString(Into::<LLVMValueRef>::into(c_), Into::<*mut std::ffi::c_ulong>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstStructInContext<T0_, T1_, T2_, T3_>(C_:  T0_, ConstantVals_:  T1_, Count_:  T2_, Packed_:  T3_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*mut LLVMValueRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstStructInContext(Into::<LLVMContextRef>::into(C_), Into::<*mut LLVMValueRef>::into(ConstantVals_), Into::<std::ffi::c_uint>::into(Count_), Into::<std::ffi::c_int>::into(Packed_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstStruct<T0_, T1_, T2_>(ConstantVals_:  T0_, Count_:  T1_, Packed_:  T2_)-> Tret_
  where
     T0_: Into<*mut LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstStruct(Into::<*mut LLVMValueRef>::into(ConstantVals_), Into::<std::ffi::c_uint>::into(Count_), Into::<std::ffi::c_int>::into(Packed_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstArray<T0_, T1_, T2_>(ElementTy_:  T0_, ConstantVals_:  T1_, Length_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*mut LLVMValueRef>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstArray(Into::<LLVMTypeRef>::into(ElementTy_), Into::<*mut LLVMValueRef>::into(ConstantVals_), Into::<std::ffi::c_uint>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstArray2<T0_, T1_, T2_>(ElementTy_:  T0_, ConstantVals_:  T1_, Length_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*mut LLVMValueRef>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstArray2(Into::<LLVMTypeRef>::into(ElementTy_), Into::<*mut LLVMValueRef>::into(ConstantVals_), Into::<std::ffi::c_ulong>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNamedStruct<T0_, T1_, T2_>(StructTy_:  T0_, ConstantVals_:  T1_, Count_:  T2_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*mut LLVMValueRef>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNamedStruct(Into::<LLVMTypeRef>::into(StructTy_), Into::<*mut LLVMValueRef>::into(ConstantVals_), Into::<std::ffi::c_uint>::into(Count_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetAggregateElement<T0_, T1_>(C_:  T0_, Idx_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetAggregateElement(Into::<LLVMValueRef>::into(C_), Into::<std::ffi::c_uint>::into(Idx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetElementAsConstant<T0_, T1_>(C_:  T0_, idx_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetElementAsConstant(Into::<LLVMValueRef>::into(C_), Into::<std::ffi::c_uint>::into(idx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstVector<T0_, T1_>(ScalarConstantVals_:  T0_, Size_:  T1_)-> Tret_
  where
     T0_: Into<*mut LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstVector(Into::<*mut LLVMValueRef>::into(ScalarConstantVals_), Into::<std::ffi::c_uint>::into(Size_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstantPtrAuth<T0_, T1_, T2_, T3_>(Ptr_:  T0_, Key_:  T1_, Disc_:  T2_, AddrDisc_:  T3_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstantPtrAuth(Into::<LLVMValueRef>::into(Ptr_), Into::<LLVMValueRef>::into(Key_), Into::<LLVMValueRef>::into(Disc_), Into::<LLVMValueRef>::into(AddrDisc_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetConstOpcode<T0_>(ConstantVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetConstOpcode(Into::<LLVMValueRef>::into(ConstantVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMAlignOf<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMAlignOf(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMSizeOf<T0_>(Ty_:  T0_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMSizeOf(Into::<LLVMTypeRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNeg<T0_>(ConstantVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNeg(Into::<LLVMValueRef>::into(ConstantVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNSWNeg<T0_>(ConstantVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNSWNeg(Into::<LLVMValueRef>::into(ConstantVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNUWNeg<T0_>(ConstantVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNUWNeg(Into::<LLVMValueRef>::into(ConstantVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNot<T0_>(ConstantVal_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNot(Into::<LLVMValueRef>::into(ConstantVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstAdd<T0_, T1_>(LHSConstant_:  T0_, RHSConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstAdd(Into::<LLVMValueRef>::into(LHSConstant_), Into::<LLVMValueRef>::into(RHSConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNSWAdd<T0_, T1_>(LHSConstant_:  T0_, RHSConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNSWAdd(Into::<LLVMValueRef>::into(LHSConstant_), Into::<LLVMValueRef>::into(RHSConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNUWAdd<T0_, T1_>(LHSConstant_:  T0_, RHSConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNUWAdd(Into::<LLVMValueRef>::into(LHSConstant_), Into::<LLVMValueRef>::into(RHSConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstSub<T0_, T1_>(LHSConstant_:  T0_, RHSConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstSub(Into::<LLVMValueRef>::into(LHSConstant_), Into::<LLVMValueRef>::into(RHSConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNSWSub<T0_, T1_>(LHSConstant_:  T0_, RHSConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNSWSub(Into::<LLVMValueRef>::into(LHSConstant_), Into::<LLVMValueRef>::into(RHSConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNUWSub<T0_, T1_>(LHSConstant_:  T0_, RHSConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNUWSub(Into::<LLVMValueRef>::into(LHSConstant_), Into::<LLVMValueRef>::into(RHSConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstMul<T0_, T1_>(LHSConstant_:  T0_, RHSConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstMul(Into::<LLVMValueRef>::into(LHSConstant_), Into::<LLVMValueRef>::into(RHSConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNSWMul<T0_, T1_>(LHSConstant_:  T0_, RHSConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNSWMul(Into::<LLVMValueRef>::into(LHSConstant_), Into::<LLVMValueRef>::into(RHSConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstNUWMul<T0_, T1_>(LHSConstant_:  T0_, RHSConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstNUWMul(Into::<LLVMValueRef>::into(LHSConstant_), Into::<LLVMValueRef>::into(RHSConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstXor<T0_, T1_>(LHSConstant_:  T0_, RHSConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstXor(Into::<LLVMValueRef>::into(LHSConstant_), Into::<LLVMValueRef>::into(RHSConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstGEP2<T0_, T1_, T2_, T3_>(Ty_:  T0_, ConstantVal_:  T1_, ConstantIndices_:  T2_, NumIndices_:  T3_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*mut LLVMValueRef>,  T3_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstGEP2(Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(ConstantVal_), Into::<*mut LLVMValueRef>::into(ConstantIndices_), Into::<std::ffi::c_uint>::into(NumIndices_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstInBoundsGEP2<T0_, T1_, T2_, T3_>(Ty_:  T0_, ConstantVal_:  T1_, ConstantIndices_:  T2_, NumIndices_:  T3_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*mut LLVMValueRef>,  T3_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstInBoundsGEP2(Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(ConstantVal_), Into::<*mut LLVMValueRef>::into(ConstantIndices_), Into::<std::ffi::c_uint>::into(NumIndices_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstGEPWithNoWrapFlags<T0_, T1_, T2_, T3_, T4_>(Ty_:  T0_, ConstantVal_:  T1_, ConstantIndices_:  T2_, NumIndices_:  T3_, NoWrapFlags_:  T4_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*mut LLVMValueRef>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstGEPWithNoWrapFlags(Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(ConstantVal_), Into::<*mut LLVMValueRef>::into(ConstantIndices_), Into::<std::ffi::c_uint>::into(NumIndices_), Into::<std::ffi::c_uint>::into(NoWrapFlags_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstTrunc<T0_, T1_>(ConstantVal_:  T0_, ToType_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstTrunc(Into::<LLVMValueRef>::into(ConstantVal_), Into::<LLVMTypeRef>::into(ToType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstPtrToInt<T0_, T1_>(ConstantVal_:  T0_, ToType_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstPtrToInt(Into::<LLVMValueRef>::into(ConstantVal_), Into::<LLVMTypeRef>::into(ToType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstIntToPtr<T0_, T1_>(ConstantVal_:  T0_, ToType_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstIntToPtr(Into::<LLVMValueRef>::into(ConstantVal_), Into::<LLVMTypeRef>::into(ToType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstBitCast<T0_, T1_>(ConstantVal_:  T0_, ToType_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstBitCast(Into::<LLVMValueRef>::into(ConstantVal_), Into::<LLVMTypeRef>::into(ToType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstAddrSpaceCast<T0_, T1_>(ConstantVal_:  T0_, ToType_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstAddrSpaceCast(Into::<LLVMValueRef>::into(ConstantVal_), Into::<LLVMTypeRef>::into(ToType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstTruncOrBitCast<T0_, T1_>(ConstantVal_:  T0_, ToType_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstTruncOrBitCast(Into::<LLVMValueRef>::into(ConstantVal_), Into::<LLVMTypeRef>::into(ToType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstPointerCast<T0_, T1_>(ConstantVal_:  T0_, ToType_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMTypeRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstPointerCast(Into::<LLVMValueRef>::into(ConstantVal_), Into::<LLVMTypeRef>::into(ToType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstExtractElement<T0_, T1_>(VectorConstant_:  T0_, IndexConstant_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstExtractElement(Into::<LLVMValueRef>::into(VectorConstant_), Into::<LLVMValueRef>::into(IndexConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstInsertElement<T0_, T1_, T2_>(VectorConstant_:  T0_, ElementValueConstant_:  T1_, IndexConstant_:  T2_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstInsertElement(Into::<LLVMValueRef>::into(VectorConstant_), Into::<LLVMValueRef>::into(ElementValueConstant_), Into::<LLVMValueRef>::into(IndexConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstShuffleVector<T0_, T1_, T2_>(VectorAConstant_:  T0_, VectorBConstant_:  T1_, MaskConstant_:  T2_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstShuffleVector(Into::<LLVMValueRef>::into(VectorAConstant_), Into::<LLVMValueRef>::into(VectorBConstant_), Into::<LLVMValueRef>::into(MaskConstant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBlockAddress<T0_, T1_>(F_:  T0_, BB_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBlockAddress(Into::<LLVMValueRef>::into(F_), Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetBlockAddressFunction<T0_>(BlockAddr_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetBlockAddressFunction(Into::<LLVMValueRef>::into(BlockAddr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetBlockAddressBasicBlock<T0_>(BlockAddr_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetBlockAddressBasicBlock(Into::<LLVMValueRef>::into(BlockAddr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMConstInlineAsm<T0_, T1_, T2_, T3_, T4_>(Ty_:  T0_, AsmString_:  T1_, Constraints_:  T2_, HasSideEffects_:  T3_, IsAlignStack_:  T4_)-> Tret_
  where
     T0_: Into<LLVMTypeRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_int>,  T4_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMConstInlineAsm(Into::<LLVMTypeRef>::into(Ty_), Into::<*const std::ffi::c_char>::into(AsmString_), Into::<*const std::ffi::c_char>::into(Constraints_), Into::<std::ffi::c_int>::into(HasSideEffects_), Into::<std::ffi::c_int>::into(IsAlignStack_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMModuleRef> {
  pub unsafe fn LLVMGetGlobalParent<T0_>(Global_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetGlobalParent(Into::<LLVMValueRef>::into(Global_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsDeclaration<T0_>(Global_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsDeclaration(Into::<LLVMValueRef>::into(Global_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetLinkage<T0_>(Global_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLinkage(Into::<LLVMValueRef>::into(Global_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetLinkage<T0_, T1_>(Global_:  T0_, Linkage_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetLinkage(Into::<LLVMValueRef>::into(Global_), Into::<std::ffi::c_uint>::into(Linkage_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetSection<T0_>(Global_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetSection(Into::<LLVMValueRef>::into(Global_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetSection<T0_, T1_>(Global_:  T0_, Section_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Core::LLVMSetSection(Into::<LLVMValueRef>::into(Global_), Into::<*const std::ffi::c_char>::into(Section_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetVisibility<T0_>(Global_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetVisibility(Into::<LLVMValueRef>::into(Global_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetVisibility<T0_, T1_>(Global_:  T0_, Viz_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetVisibility(Into::<LLVMValueRef>::into(Global_), Into::<std::ffi::c_uint>::into(Viz_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetDLLStorageClass<T0_>(Global_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetDLLStorageClass(Into::<LLVMValueRef>::into(Global_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetDLLStorageClass<T0_, T1_>(Global_:  T0_, Class_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetDLLStorageClass(Into::<LLVMValueRef>::into(Global_), Into::<std::ffi::c_uint>::into(Class_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetUnnamedAddress<T0_>(Global_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetUnnamedAddress(Into::<LLVMValueRef>::into(Global_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetUnnamedAddress<T0_, T1_>(Global_:  T0_, UnnamedAddr_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetUnnamedAddress(Into::<LLVMValueRef>::into(Global_), Into::<std::ffi::c_uint>::into(UnnamedAddr_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGlobalGetValueType<T0_>(Global_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGlobalGetValueType(Into::<LLVMValueRef>::into(Global_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMHasUnnamedAddr<T0_>(Global_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMHasUnnamedAddr(Into::<LLVMValueRef>::into(Global_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetUnnamedAddr<T0_, T1_>(Global_:  T0_, HasUnnamedAddr_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetUnnamedAddr(Into::<LLVMValueRef>::into(Global_), Into::<std::ffi::c_int>::into(HasUnnamedAddr_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetAlignment<T0_>(V_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetAlignment(Into::<LLVMValueRef>::into(V_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetAlignment<T0_, T1_>(V_:  T0_, Bytes_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetAlignment(Into::<LLVMValueRef>::into(V_), Into::<std::ffi::c_uint>::into(Bytes_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGlobalSetMetadata<T0_, T1_, T2_>(Global_:  T0_, Kind_:  T1_, MD_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMMetadataRef>
  {
    unsafe {
      crate::Core::LLVMGlobalSetMetadata(Into::<LLVMValueRef>::into(Global_), Into::<std::ffi::c_uint>::into(Kind_), Into::<LLVMMetadataRef>::into(MD_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGlobalEraseMetadata<T0_, T1_>(Global_:  T0_, Kind_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMGlobalEraseMetadata(Into::<LLVMValueRef>::into(Global_), Into::<std::ffi::c_uint>::into(Kind_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGlobalClearMetadata<T0_>(Global_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMGlobalClearMetadata(Into::<LLVMValueRef>::into(Global_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut LLVMValueMetadataEntry> {
  pub unsafe fn LLVMGlobalCopyAllMetadata<T0_, T1_>(Value_:  T0_, NumEntries_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGlobalCopyAllMetadata(Into::<LLVMValueRef>::into(Value_), Into::<*mut std::ffi::c_ulong>::into(NumEntries_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeValueMetadataEntries<T0_>(Entries_:  T0_)
  where
     T0_: Into<*mut LLVMValueMetadataEntry>
  {
    unsafe {
      crate::Core::LLVMDisposeValueMetadataEntries(Into::<*mut LLVMValueMetadataEntry>::into(Entries_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMValueMetadataEntriesGetKind<T0_, T1_>(Entries_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<*mut LLVMValueMetadataEntry>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMValueMetadataEntriesGetKind(Into::<*mut LLVMValueMetadataEntry>::into(Entries_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMValueMetadataEntriesGetMetadata<T0_, T1_>(Entries_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<*mut LLVMValueMetadataEntry>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMValueMetadataEntriesGetMetadata(Into::<*mut LLVMValueMetadataEntry>::into(Entries_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMAddGlobal<T0_, T1_, T2_>(M_:  T0_, Ty_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMAddGlobal(Into::<LLVMModuleRef>::into(M_), Into::<LLVMTypeRef>::into(Ty_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMAddGlobalInAddressSpace<T0_, T1_, T2_, T3_>(M_:  T0_, Ty_:  T1_, Name_:  T2_, AddressSpace_:  T3_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMAddGlobalInAddressSpace(Into::<LLVMModuleRef>::into(M_), Into::<LLVMTypeRef>::into(Ty_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_uint>::into(AddressSpace_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNamedGlobal<T0_, T1_>(M_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNamedGlobal(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNamedGlobalWithLength<T0_, T1_, T2_>(M_:  T0_, Name_:  T1_, Length_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNamedGlobalWithLength(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetFirstGlobal<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFirstGlobal(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetLastGlobal<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLastGlobal(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNextGlobal<T0_>(GlobalVar_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNextGlobal(Into::<LLVMValueRef>::into(GlobalVar_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetPreviousGlobal<T0_>(GlobalVar_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPreviousGlobal(Into::<LLVMValueRef>::into(GlobalVar_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDeleteGlobal<T0_>(GlobalVar_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMDeleteGlobal(Into::<LLVMValueRef>::into(GlobalVar_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetInitializer<T0_>(GlobalVar_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInitializer(Into::<LLVMValueRef>::into(GlobalVar_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetInitializer<T0_, T1_>(GlobalVar_:  T0_, ConstantVal_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetInitializer(Into::<LLVMValueRef>::into(GlobalVar_), Into::<LLVMValueRef>::into(ConstantVal_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsThreadLocal<T0_>(GlobalVar_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsThreadLocal(Into::<LLVMValueRef>::into(GlobalVar_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetThreadLocal<T0_, T1_>(GlobalVar_:  T0_, IsThreadLocal_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetThreadLocal(Into::<LLVMValueRef>::into(GlobalVar_), Into::<std::ffi::c_int>::into(IsThreadLocal_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsGlobalConstant<T0_>(GlobalVar_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsGlobalConstant(Into::<LLVMValueRef>::into(GlobalVar_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetGlobalConstant<T0_, T1_>(GlobalVar_:  T0_, IsConstant_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetGlobalConstant(Into::<LLVMValueRef>::into(GlobalVar_), Into::<std::ffi::c_int>::into(IsConstant_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetThreadLocalMode<T0_>(GlobalVar_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetThreadLocalMode(Into::<LLVMValueRef>::into(GlobalVar_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetThreadLocalMode<T0_, T1_>(GlobalVar_:  T0_, Mode_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetThreadLocalMode(Into::<LLVMValueRef>::into(GlobalVar_), Into::<std::ffi::c_uint>::into(Mode_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsExternallyInitialized<T0_>(GlobalVar_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsExternallyInitialized(Into::<LLVMValueRef>::into(GlobalVar_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetExternallyInitialized<T0_, T1_>(GlobalVar_:  T0_, IsExtInit_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetExternallyInitialized(Into::<LLVMValueRef>::into(GlobalVar_), Into::<std::ffi::c_int>::into(IsExtInit_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMAddAlias2<T0_, T1_, T2_, T3_, T4_>(M_:  T0_, ValueTy_:  T1_, AddrSpace_:  T2_, Aliasee_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<LLVMValueRef>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMAddAlias2(Into::<LLVMModuleRef>::into(M_), Into::<LLVMTypeRef>::into(ValueTy_), Into::<std::ffi::c_uint>::into(AddrSpace_), Into::<LLVMValueRef>::into(Aliasee_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNamedGlobalAlias<T0_, T1_, T2_>(M_:  T0_, Name_:  T1_, NameLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNamedGlobalAlias(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetFirstGlobalAlias<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFirstGlobalAlias(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetLastGlobalAlias<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLastGlobalAlias(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNextGlobalAlias<T0_>(GA_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNextGlobalAlias(Into::<LLVMValueRef>::into(GA_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetPreviousGlobalAlias<T0_>(GA_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPreviousGlobalAlias(Into::<LLVMValueRef>::into(GA_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMAliasGetAliasee<T0_>(Alias_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMAliasGetAliasee(Into::<LLVMValueRef>::into(Alias_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAliasSetAliasee<T0_, T1_>(Alias_:  T0_, Aliasee_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMAliasSetAliasee(Into::<LLVMValueRef>::into(Alias_), Into::<LLVMValueRef>::into(Aliasee_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDeleteFunction<T0_>(Fn_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMDeleteFunction(Into::<LLVMValueRef>::into(Fn_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMHasPersonalityFn<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMHasPersonalityFn(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetPersonalityFn<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPersonalityFn(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetPersonalityFn<T0_, T1_>(Fn_:  T0_, PersonalityFn_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetPersonalityFn(Into::<LLVMValueRef>::into(Fn_), Into::<LLVMValueRef>::into(PersonalityFn_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMLookupIntrinsicID<T0_, T1_>(Name_:  T0_, NameLen_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMLookupIntrinsicID(Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetIntrinsicID<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetIntrinsicID(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetIntrinsicDeclaration<T0_, T1_, T2_, T3_>(Mod_:  T0_, ID_:  T1_, ParamTypes_:  T2_, ParamCount_:  T3_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*mut LLVMTypeRef>,  T3_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetIntrinsicDeclaration(Into::<LLVMModuleRef>::into(Mod_), Into::<std::ffi::c_uint>::into(ID_), Into::<*mut LLVMTypeRef>::into(ParamTypes_), Into::<std::ffi::c_ulong>::into(ParamCount_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMIntrinsicGetType<T0_, T1_, T2_, T3_>(Ctx_:  T0_, ID_:  T1_, ParamTypes_:  T2_, ParamCount_:  T3_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*mut LLVMTypeRef>,  T3_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIntrinsicGetType(Into::<LLVMContextRef>::into(Ctx_), Into::<std::ffi::c_uint>::into(ID_), Into::<*mut LLVMTypeRef>::into(ParamTypes_), Into::<std::ffi::c_ulong>::into(ParamCount_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMIntrinsicGetName<T0_, T1_>(ID_:  T0_, NameLength_:  T1_)-> Tret_
  where
     T0_: Into<std::ffi::c_uint>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIntrinsicGetName(Into::<std::ffi::c_uint>::into(ID_), Into::<*mut std::ffi::c_ulong>::into(NameLength_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMIntrinsicCopyOverloadedName<T0_, T1_, T2_, T3_>(ID_:  T0_, ParamTypes_:  T1_, ParamCount_:  T2_, NameLength_:  T3_)-> Tret_
  where
     T0_: Into<std::ffi::c_uint>,  T1_: Into<*mut LLVMTypeRef>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIntrinsicCopyOverloadedName(Into::<std::ffi::c_uint>::into(ID_), Into::<*mut LLVMTypeRef>::into(ParamTypes_), Into::<std::ffi::c_ulong>::into(ParamCount_), Into::<*mut std::ffi::c_ulong>::into(NameLength_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_char> {
  pub unsafe fn LLVMIntrinsicCopyOverloadedName2<T0_, T1_, T2_, T3_, T4_>(Mod_:  T0_, ID_:  T1_, ParamTypes_:  T2_, ParamCount_:  T3_, NameLength_:  T4_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*mut LLVMTypeRef>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIntrinsicCopyOverloadedName2(Into::<LLVMModuleRef>::into(Mod_), Into::<std::ffi::c_uint>::into(ID_), Into::<*mut LLVMTypeRef>::into(ParamTypes_), Into::<std::ffi::c_ulong>::into(ParamCount_), Into::<*mut std::ffi::c_ulong>::into(NameLength_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIntrinsicIsOverloaded<T0_>(ID_:  T0_)-> Tret_
  where
     T0_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIntrinsicIsOverloaded(Into::<std::ffi::c_uint>::into(ID_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetFunctionCallConv<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFunctionCallConv(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetFunctionCallConv<T0_, T1_>(Fn_:  T0_, CC_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetFunctionCallConv(Into::<LLVMValueRef>::into(Fn_), Into::<std::ffi::c_uint>::into(CC_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetGC<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetGC(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetGC<T0_, T1_>(Fn_:  T0_, Name_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Core::LLVMSetGC(Into::<LLVMValueRef>::into(Fn_), Into::<*const std::ffi::c_char>::into(Name_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetPrefixData<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPrefixData(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMHasPrefixData<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMHasPrefixData(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetPrefixData<T0_, T1_>(Fn_:  T0_, prefixData_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetPrefixData(Into::<LLVMValueRef>::into(Fn_), Into::<LLVMValueRef>::into(prefixData_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetPrologueData<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPrologueData(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMHasPrologueData<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMHasPrologueData(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetPrologueData<T0_, T1_>(Fn_:  T0_, prologueData_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetPrologueData(Into::<LLVMValueRef>::into(Fn_), Into::<LLVMValueRef>::into(prologueData_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddAttributeAtIndex<T0_, T1_, T2_>(F_:  T0_, Idx_:  T1_, A_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMAttributeRef>
  {
    unsafe {
      crate::Core::LLVMAddAttributeAtIndex(Into::<LLVMValueRef>::into(F_), Into::<std::ffi::c_uint>::into(Idx_), Into::<LLVMAttributeRef>::into(A_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetAttributeCountAtIndex<T0_, T1_>(F_:  T0_, Idx_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetAttributeCountAtIndex(Into::<LLVMValueRef>::into(F_), Into::<std::ffi::c_uint>::into(Idx_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGetAttributesAtIndex<T0_, T1_, T2_>(F_:  T0_, Idx_:  T1_, Attrs_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*mut LLVMAttributeRef>
  {
    unsafe {
      crate::Core::LLVMGetAttributesAtIndex(Into::<LLVMValueRef>::into(F_), Into::<std::ffi::c_uint>::into(Idx_), Into::<*mut LLVMAttributeRef>::into(Attrs_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMAttributeRef> {
  pub unsafe fn LLVMGetEnumAttributeAtIndex<T0_, T1_, T2_>(F_:  T0_, Idx_:  T1_, KindID_:  T2_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetEnumAttributeAtIndex(Into::<LLVMValueRef>::into(F_), Into::<std::ffi::c_uint>::into(Idx_), Into::<std::ffi::c_uint>::into(KindID_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMAttributeRef> {
  pub unsafe fn LLVMGetStringAttributeAtIndex<T0_, T1_, T2_, T3_>(F_:  T0_, Idx_:  T1_, K_:  T2_, KLen_:  T3_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetStringAttributeAtIndex(Into::<LLVMValueRef>::into(F_), Into::<std::ffi::c_uint>::into(Idx_), Into::<*const std::ffi::c_char>::into(K_), Into::<std::ffi::c_uint>::into(KLen_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMRemoveEnumAttributeAtIndex<T0_, T1_, T2_>(F_:  T0_, Idx_:  T1_, KindID_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMRemoveEnumAttributeAtIndex(Into::<LLVMValueRef>::into(F_), Into::<std::ffi::c_uint>::into(Idx_), Into::<std::ffi::c_uint>::into(KindID_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMRemoveStringAttributeAtIndex<T0_, T1_, T2_, T3_>(F_:  T0_, Idx_:  T1_, K_:  T2_, KLen_:  T3_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMRemoveStringAttributeAtIndex(Into::<LLVMValueRef>::into(F_), Into::<std::ffi::c_uint>::into(Idx_), Into::<*const std::ffi::c_char>::into(K_), Into::<std::ffi::c_uint>::into(KLen_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddTargetDependentFunctionAttr<T0_, T1_, T2_>(Fn_:  T0_, A_:  T1_, V_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Core::LLVMAddTargetDependentFunctionAttr(Into::<LLVMValueRef>::into(Fn_), Into::<*const std::ffi::c_char>::into(A_), Into::<*const std::ffi::c_char>::into(V_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMCountParams<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCountParams(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGetParams<T0_, T1_>(Fn_:  T0_, Params_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMGetParams(Into::<LLVMValueRef>::into(Fn_), Into::<*mut LLVMValueRef>::into(Params_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetParam<T0_, T1_>(Fn_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetParam(Into::<LLVMValueRef>::into(Fn_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetParamParent<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetParamParent(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetFirstParam<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFirstParam(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetLastParam<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLastParam(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNextParam<T0_>(Arg_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNextParam(Into::<LLVMValueRef>::into(Arg_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetPreviousParam<T0_>(Arg_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPreviousParam(Into::<LLVMValueRef>::into(Arg_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetParamAlignment<T0_, T1_>(Arg_:  T0_, Align_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetParamAlignment(Into::<LLVMValueRef>::into(Arg_), Into::<std::ffi::c_uint>::into(Align_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMAddGlobalIFunc<T0_, T1_, T2_, T3_, T4_, T5_>(M_:  T0_, Name_:  T1_, NameLen_:  T2_, Ty_:  T3_, AddrSpace_:  T4_, Resolver_:  T5_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<LLVMTypeRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMAddGlobalIFunc(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMTypeRef>::into(Ty_), Into::<std::ffi::c_uint>::into(AddrSpace_), Into::<LLVMValueRef>::into(Resolver_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNamedGlobalIFunc<T0_, T1_, T2_>(M_:  T0_, Name_:  T1_, NameLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNamedGlobalIFunc(Into::<LLVMModuleRef>::into(M_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetFirstGlobalIFunc<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFirstGlobalIFunc(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetLastGlobalIFunc<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLastGlobalIFunc(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNextGlobalIFunc<T0_>(IFunc_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNextGlobalIFunc(Into::<LLVMValueRef>::into(IFunc_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetPreviousGlobalIFunc<T0_>(IFunc_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPreviousGlobalIFunc(Into::<LLVMValueRef>::into(IFunc_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetGlobalIFuncResolver<T0_>(IFunc_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetGlobalIFuncResolver(Into::<LLVMValueRef>::into(IFunc_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetGlobalIFuncResolver<T0_, T1_>(IFunc_:  T0_, Resolver_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetGlobalIFuncResolver(Into::<LLVMValueRef>::into(IFunc_), Into::<LLVMValueRef>::into(Resolver_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMEraseGlobalIFunc<T0_>(IFunc_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMEraseGlobalIFunc(Into::<LLVMValueRef>::into(IFunc_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMRemoveGlobalIFunc<T0_>(IFunc_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMRemoveGlobalIFunc(Into::<LLVMValueRef>::into(IFunc_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMMDStringInContext2<T0_, T1_, T2_>(C_:  T0_, Str_:  T1_, SLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMMDStringInContext2(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(Str_), Into::<std::ffi::c_ulong>::into(SLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMMDNodeInContext2<T0_, T1_, T2_>(C_:  T0_, MDs_:  T1_, Count_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*mut LLVMMetadataRef>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMMDNodeInContext2(Into::<LLVMContextRef>::into(C_), Into::<*mut LLVMMetadataRef>::into(MDs_), Into::<std::ffi::c_ulong>::into(Count_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMMetadataAsValue<T0_, T1_>(C_:  T0_, MD_:  T1_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMMetadataAsValue(Into::<LLVMContextRef>::into(C_), Into::<LLVMMetadataRef>::into(MD_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMValueAsMetadata<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMValueAsMetadata(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetMDString<T0_, T1_>(V_:  T0_, Length_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetMDString(Into::<LLVMValueRef>::into(V_), Into::<*mut std::ffi::c_uint>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetMDNodeNumOperands<T0_>(V_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetMDNodeNumOperands(Into::<LLVMValueRef>::into(V_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGetMDNodeOperands<T0_, T1_>(V_:  T0_, Dest_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMGetMDNodeOperands(Into::<LLVMValueRef>::into(V_), Into::<*mut LLVMValueRef>::into(Dest_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMReplaceMDNodeOperandWith<T0_, T1_, T2_>(V_:  T0_, Index_:  T1_, Replacement_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMMetadataRef>
  {
    unsafe {
      crate::Core::LLVMReplaceMDNodeOperandWith(Into::<LLVMValueRef>::into(V_), Into::<std::ffi::c_uint>::into(Index_), Into::<LLVMMetadataRef>::into(Replacement_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMMDStringInContext<T0_, T1_, T2_>(C_:  T0_, Str_:  T1_, SLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMMDStringInContext(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(Str_), Into::<std::ffi::c_uint>::into(SLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMMDString<T0_, T1_>(Str_:  T0_, SLen_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMMDString(Into::<*const std::ffi::c_char>::into(Str_), Into::<std::ffi::c_uint>::into(SLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMMDNodeInContext<T0_, T1_, T2_>(C_:  T0_, Vals_:  T1_, Count_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*mut LLVMValueRef>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMMDNodeInContext(Into::<LLVMContextRef>::into(C_), Into::<*mut LLVMValueRef>::into(Vals_), Into::<std::ffi::c_uint>::into(Count_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMMDNode<T0_, T1_>(Vals_:  T0_, Count_:  T1_)-> Tret_
  where
     T0_: Into<*mut LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMMDNode(Into::<*mut LLVMValueRef>::into(Vals_), Into::<std::ffi::c_uint>::into(Count_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOperandBundleRef> {
  pub unsafe fn LLVMCreateOperandBundle<T0_, T1_, T2_, T3_>(Tag_:  T0_, TagLen_:  T1_, Args_:  T2_, NumArgs_:  T3_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*mut LLVMValueRef>,  T3_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateOperandBundle(Into::<*const std::ffi::c_char>::into(Tag_), Into::<std::ffi::c_ulong>::into(TagLen_), Into::<*mut LLVMValueRef>::into(Args_), Into::<std::ffi::c_uint>::into(NumArgs_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeOperandBundle<T0_>(Bundle_:  T0_)
  where
     T0_: Into<LLVMOperandBundleRef>
  {
    unsafe {
      crate::Core::LLVMDisposeOperandBundle(Into::<LLVMOperandBundleRef>::into(Bundle_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetOperandBundleTag<T0_, T1_>(Bundle_:  T0_, Len_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOperandBundleRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetOperandBundleTag(Into::<LLVMOperandBundleRef>::into(Bundle_), Into::<*mut std::ffi::c_ulong>::into(Len_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetNumOperandBundleArgs<T0_>(Bundle_:  T0_)-> Tret_
  where
     T0_: Into<LLVMOperandBundleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNumOperandBundleArgs(Into::<LLVMOperandBundleRef>::into(Bundle_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetOperandBundleArgAtIndex<T0_, T1_>(Bundle_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<LLVMOperandBundleRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetOperandBundleArgAtIndex(Into::<LLVMOperandBundleRef>::into(Bundle_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBasicBlockAsValue<T0_>(BB_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBasicBlockAsValue(Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMValueIsBasicBlock<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMValueIsBasicBlock(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMValueAsBasicBlock<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMValueAsBasicBlock(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetBasicBlockName<T0_>(BB_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetBasicBlockName(Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetBasicBlockParent<T0_>(BB_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetBasicBlockParent(Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetBasicBlockTerminator<T0_>(BB_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetBasicBlockTerminator(Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMCountBasicBlocks<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCountBasicBlocks(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGetBasicBlocks<T0_, T1_>(Fn_:  T0_, BasicBlocks_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMGetBasicBlocks(Into::<LLVMValueRef>::into(Fn_), Into::<*mut LLVMBasicBlockRef>::into(BasicBlocks_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetFirstBasicBlock<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFirstBasicBlock(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetLastBasicBlock<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLastBasicBlock(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetNextBasicBlock<T0_>(BB_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNextBasicBlock(Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetPreviousBasicBlock<T0_>(BB_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPreviousBasicBlock(Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetEntryBasicBlock<T0_>(Fn_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetEntryBasicBlock(Into::<LLVMValueRef>::into(Fn_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMInsertExistingBasicBlockAfterInsertBlock<T0_, T1_>(Builder_:  T0_, BB_:  T1_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMInsertExistingBasicBlockAfterInsertBlock(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMBasicBlockRef>::into(BB_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAppendExistingBasicBlock<T0_, T1_>(Fn_:  T0_, BB_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMAppendExistingBasicBlock(Into::<LLVMValueRef>::into(Fn_), Into::<LLVMBasicBlockRef>::into(BB_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMCreateBasicBlockInContext<T0_, T1_>(C_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateBasicBlockInContext(Into::<LLVMContextRef>::into(C_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMAppendBasicBlockInContext<T0_, T1_, T2_>(C_:  T0_, Fn_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMAppendBasicBlockInContext(Into::<LLVMContextRef>::into(C_), Into::<LLVMValueRef>::into(Fn_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMAppendBasicBlock<T0_, T1_>(Fn_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMAppendBasicBlock(Into::<LLVMValueRef>::into(Fn_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMInsertBasicBlockInContext<T0_, T1_, T2_>(C_:  T0_, BB_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<LLVMBasicBlockRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInsertBasicBlockInContext(Into::<LLVMContextRef>::into(C_), Into::<LLVMBasicBlockRef>::into(BB_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMInsertBasicBlock<T0_, T1_>(InsertBeforeBB_:  T0_, Name_:  T1_)-> Tret_
  where
     T0_: Into<LLVMBasicBlockRef>,  T1_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInsertBasicBlock(Into::<LLVMBasicBlockRef>::into(InsertBeforeBB_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDeleteBasicBlock<T0_>(BB_:  T0_)
  where
     T0_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMDeleteBasicBlock(Into::<LLVMBasicBlockRef>::into(BB_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMRemoveBasicBlockFromParent<T0_>(BB_:  T0_)
  where
     T0_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMRemoveBasicBlockFromParent(Into::<LLVMBasicBlockRef>::into(BB_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMMoveBasicBlockBefore<T0_, T1_>(BB_:  T0_, MovePos_:  T1_)
  where
     T0_: Into<LLVMBasicBlockRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMMoveBasicBlockBefore(Into::<LLVMBasicBlockRef>::into(BB_), Into::<LLVMBasicBlockRef>::into(MovePos_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMMoveBasicBlockAfter<T0_, T1_>(BB_:  T0_, MovePos_:  T1_)
  where
     T0_: Into<LLVMBasicBlockRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMMoveBasicBlockAfter(Into::<LLVMBasicBlockRef>::into(BB_), Into::<LLVMBasicBlockRef>::into(MovePos_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetFirstInstruction<T0_>(BB_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFirstInstruction(Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetLastInstruction<T0_>(BB_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLastInstruction(Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMHasMetadata<T0_>(Val_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMHasMetadata(Into::<LLVMValueRef>::into(Val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetMetadata<T0_, T1_>(Val_:  T0_, KindID_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetMetadata(Into::<LLVMValueRef>::into(Val_), Into::<std::ffi::c_uint>::into(KindID_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetMetadata<T0_, T1_, T2_>(Val_:  T0_, KindID_:  T1_, Node_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetMetadata(Into::<LLVMValueRef>::into(Val_), Into::<std::ffi::c_uint>::into(KindID_), Into::<LLVMValueRef>::into(Node_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut LLVMValueMetadataEntry> {
  pub unsafe fn LLVMInstructionGetAllMetadataOtherThanDebugLoc<T0_, T1_>(Instr_:  T0_, NumEntries_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInstructionGetAllMetadataOtherThanDebugLoc(Into::<LLVMValueRef>::into(Instr_), Into::<*mut std::ffi::c_ulong>::into(NumEntries_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetInstructionParent<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInstructionParent(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetNextInstruction<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNextInstruction(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetPreviousInstruction<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPreviousInstruction(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMInstructionRemoveFromParent<T0_>(Inst_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMInstructionRemoveFromParent(Into::<LLVMValueRef>::into(Inst_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMInstructionEraseFromParent<T0_>(Inst_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMInstructionEraseFromParent(Into::<LLVMValueRef>::into(Inst_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDeleteInstruction<T0_>(Inst_:  T0_)
  where
     T0_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMDeleteInstruction(Into::<LLVMValueRef>::into(Inst_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetInstructionOpcode<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInstructionOpcode(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetICmpPredicate<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetICmpPredicate(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetFCmpPredicate<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFCmpPredicate(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMInstructionClone<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInstructionClone(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMIsATerminatorInst<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsATerminatorInst(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDbgRecordRef> {
  pub unsafe fn LLVMGetFirstDbgRecord<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFirstDbgRecord(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDbgRecordRef> {
  pub unsafe fn LLVMGetLastDbgRecord<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetLastDbgRecord(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDbgRecordRef> {
  pub unsafe fn LLVMGetNextDbgRecord<T0_>(DbgRecord_:  T0_)-> Tret_
  where
     T0_: Into<LLVMDbgRecordRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNextDbgRecord(Into::<LLVMDbgRecordRef>::into(DbgRecord_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDbgRecordRef> {
  pub unsafe fn LLVMGetPreviousDbgRecord<T0_>(DbgRecord_:  T0_)-> Tret_
  where
     T0_: Into<LLVMDbgRecordRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetPreviousDbgRecord(Into::<LLVMDbgRecordRef>::into(DbgRecord_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetNumArgOperands<T0_>(Instr_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNumArgOperands(Into::<LLVMValueRef>::into(Instr_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetInstructionCallConv<T0_, T1_>(Instr_:  T0_, CC_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetInstructionCallConv(Into::<LLVMValueRef>::into(Instr_), Into::<std::ffi::c_uint>::into(CC_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetInstructionCallConv<T0_>(Instr_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInstructionCallConv(Into::<LLVMValueRef>::into(Instr_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetInstrParamAlignment<T0_, T1_, T2_>(Instr_:  T0_, Idx_:  T1_, Align_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetInstrParamAlignment(Into::<LLVMValueRef>::into(Instr_), Into::<std::ffi::c_uint>::into(Idx_), Into::<std::ffi::c_uint>::into(Align_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddCallSiteAttribute<T0_, T1_, T2_>(C_:  T0_, Idx_:  T1_, A_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMAttributeRef>
  {
    unsafe {
      crate::Core::LLVMAddCallSiteAttribute(Into::<LLVMValueRef>::into(C_), Into::<std::ffi::c_uint>::into(Idx_), Into::<LLVMAttributeRef>::into(A_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetCallSiteAttributeCount<T0_, T1_>(C_:  T0_, Idx_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCallSiteAttributeCount(Into::<LLVMValueRef>::into(C_), Into::<std::ffi::c_uint>::into(Idx_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGetCallSiteAttributes<T0_, T1_, T2_>(C_:  T0_, Idx_:  T1_, Attrs_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*mut LLVMAttributeRef>
  {
    unsafe {
      crate::Core::LLVMGetCallSiteAttributes(Into::<LLVMValueRef>::into(C_), Into::<std::ffi::c_uint>::into(Idx_), Into::<*mut LLVMAttributeRef>::into(Attrs_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMAttributeRef> {
  pub unsafe fn LLVMGetCallSiteEnumAttribute<T0_, T1_, T2_>(C_:  T0_, Idx_:  T1_, KindID_:  T2_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCallSiteEnumAttribute(Into::<LLVMValueRef>::into(C_), Into::<std::ffi::c_uint>::into(Idx_), Into::<std::ffi::c_uint>::into(KindID_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMAttributeRef> {
  pub unsafe fn LLVMGetCallSiteStringAttribute<T0_, T1_, T2_, T3_>(C_:  T0_, Idx_:  T1_, K_:  T2_, KLen_:  T3_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCallSiteStringAttribute(Into::<LLVMValueRef>::into(C_), Into::<std::ffi::c_uint>::into(Idx_), Into::<*const std::ffi::c_char>::into(K_), Into::<std::ffi::c_uint>::into(KLen_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMRemoveCallSiteEnumAttribute<T0_, T1_, T2_>(C_:  T0_, Idx_:  T1_, KindID_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMRemoveCallSiteEnumAttribute(Into::<LLVMValueRef>::into(C_), Into::<std::ffi::c_uint>::into(Idx_), Into::<std::ffi::c_uint>::into(KindID_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMRemoveCallSiteStringAttribute<T0_, T1_, T2_, T3_>(C_:  T0_, Idx_:  T1_, K_:  T2_, KLen_:  T3_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMRemoveCallSiteStringAttribute(Into::<LLVMValueRef>::into(C_), Into::<std::ffi::c_uint>::into(Idx_), Into::<*const std::ffi::c_char>::into(K_), Into::<std::ffi::c_uint>::into(KLen_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGetCalledFunctionType<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCalledFunctionType(Into::<LLVMValueRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetCalledValue<T0_>(Instr_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCalledValue(Into::<LLVMValueRef>::into(Instr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetNumOperandBundles<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNumOperandBundles(Into::<LLVMValueRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMOperandBundleRef> {
  pub unsafe fn LLVMGetOperandBundleAtIndex<T0_, T1_>(C_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetOperandBundleAtIndex(Into::<LLVMValueRef>::into(C_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsTailCall<T0_>(CallInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsTailCall(Into::<LLVMValueRef>::into(CallInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetTailCall<T0_, T1_>(CallInst_:  T0_, IsTailCall_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetTailCall(Into::<LLVMValueRef>::into(CallInst_), Into::<std::ffi::c_int>::into(IsTailCall_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetTailCallKind<T0_>(CallInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetTailCallKind(Into::<LLVMValueRef>::into(CallInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetTailCallKind<T0_, T1_>(CallInst_:  T0_, kind_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetTailCallKind(Into::<LLVMValueRef>::into(CallInst_), Into::<std::ffi::c_uint>::into(kind_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetNormalDest<T0_>(InvokeInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNormalDest(Into::<LLVMValueRef>::into(InvokeInst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetUnwindDest<T0_>(InvokeInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetUnwindDest(Into::<LLVMValueRef>::into(InvokeInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetNormalDest<T0_, T1_>(InvokeInst_:  T0_, B_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMSetNormalDest(Into::<LLVMValueRef>::into(InvokeInst_), Into::<LLVMBasicBlockRef>::into(B_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetUnwindDest<T0_, T1_>(InvokeInst_:  T0_, B_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMSetUnwindDest(Into::<LLVMValueRef>::into(InvokeInst_), Into::<LLVMBasicBlockRef>::into(B_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetCallBrDefaultDest<T0_>(CallBr_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCallBrDefaultDest(Into::<LLVMValueRef>::into(CallBr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetCallBrNumIndirectDests<T0_>(CallBr_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCallBrNumIndirectDests(Into::<LLVMValueRef>::into(CallBr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetCallBrIndirectDest<T0_, T1_>(CallBr_:  T0_, Idx_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCallBrIndirectDest(Into::<LLVMValueRef>::into(CallBr_), Into::<std::ffi::c_uint>::into(Idx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetNumSuccessors<T0_>(Term_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNumSuccessors(Into::<LLVMValueRef>::into(Term_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetSuccessor<T0_, T1_>(Term_:  T0_, i_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetSuccessor(Into::<LLVMValueRef>::into(Term_), Into::<std::ffi::c_uint>::into(i_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetSuccessor<T0_, T1_, T2_>(Term_:  T0_, i_:  T1_, block_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMSetSuccessor(Into::<LLVMValueRef>::into(Term_), Into::<std::ffi::c_uint>::into(i_), Into::<LLVMBasicBlockRef>::into(block_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsConditional<T0_>(Branch_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsConditional(Into::<LLVMValueRef>::into(Branch_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetCondition<T0_>(Branch_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCondition(Into::<LLVMValueRef>::into(Branch_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetCondition<T0_, T1_>(Branch_:  T0_, Cond_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetCondition(Into::<LLVMValueRef>::into(Branch_), Into::<LLVMValueRef>::into(Cond_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetSwitchDefaultDest<T0_>(SwitchInstr_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetSwitchDefaultDest(Into::<LLVMValueRef>::into(SwitchInstr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGetAllocatedType<T0_>(Alloca_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetAllocatedType(Into::<LLVMValueRef>::into(Alloca_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsInBounds<T0_>(GEP_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsInBounds(Into::<LLVMValueRef>::into(GEP_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetIsInBounds<T0_, T1_>(GEP_:  T0_, InBounds_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetIsInBounds(Into::<LLVMValueRef>::into(GEP_), Into::<std::ffi::c_int>::into(InBounds_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMTypeRef> {
  pub unsafe fn LLVMGetGEPSourceElementType<T0_>(GEP_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetGEPSourceElementType(Into::<LLVMValueRef>::into(GEP_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGEPGetNoWrapFlags<T0_>(GEP_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGEPGetNoWrapFlags(Into::<LLVMValueRef>::into(GEP_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGEPSetNoWrapFlags<T0_, T1_>(GEP_:  T0_, NoWrapFlags_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMGEPSetNoWrapFlags(Into::<LLVMValueRef>::into(GEP_), Into::<std::ffi::c_uint>::into(NoWrapFlags_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddIncoming<T0_, T1_, T2_, T3_>(PhiNode_:  T0_, IncomingValues_:  T1_, IncomingBlocks_:  T2_, Count_:  T3_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut LLVMValueRef>,  T2_: Into<*mut LLVMBasicBlockRef>,  T3_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMAddIncoming(Into::<LLVMValueRef>::into(PhiNode_), Into::<*mut LLVMValueRef>::into(IncomingValues_), Into::<*mut LLVMBasicBlockRef>::into(IncomingBlocks_), Into::<std::ffi::c_uint>::into(Count_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMCountIncoming<T0_>(PhiNode_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCountIncoming(Into::<LLVMValueRef>::into(PhiNode_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetIncomingValue<T0_, T1_>(PhiNode_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetIncomingValue(Into::<LLVMValueRef>::into(PhiNode_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetIncomingBlock<T0_, T1_>(PhiNode_:  T0_, Index_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetIncomingBlock(Into::<LLVMValueRef>::into(PhiNode_), Into::<std::ffi::c_uint>::into(Index_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetNumIndices<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNumIndices(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_uint> {
  pub unsafe fn LLVMGetIndices<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetIndices(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBuilderRef> {
  pub unsafe fn LLVMCreateBuilderInContext<T0_>(C_:  T0_)-> Tret_
  where
     T0_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateBuilderInContext(Into::<LLVMContextRef>::into(C_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBuilderRef> {
  pub unsafe fn LLVMCreateBuilder()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateBuilder()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPositionBuilder<T0_, T1_, T2_>(Builder_:  T0_, Block_:  T1_, Instr_:  T2_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMBasicBlockRef>,  T2_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMPositionBuilder(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMBasicBlockRef>::into(Block_), Into::<LLVMValueRef>::into(Instr_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPositionBuilderBeforeDbgRecords<T0_, T1_, T2_>(Builder_:  T0_, Block_:  T1_, Inst_:  T2_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMBasicBlockRef>,  T2_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMPositionBuilderBeforeDbgRecords(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMBasicBlockRef>::into(Block_), Into::<LLVMValueRef>::into(Inst_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPositionBuilderBefore<T0_, T1_>(Builder_:  T0_, Instr_:  T1_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMPositionBuilderBefore(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(Instr_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPositionBuilderBeforeInstrAndDbgRecords<T0_, T1_>(Builder_:  T0_, Instr_:  T1_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMPositionBuilderBeforeInstrAndDbgRecords(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(Instr_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMPositionBuilderAtEnd<T0_, T1_>(Builder_:  T0_, Block_:  T1_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMPositionBuilderAtEnd(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMBasicBlockRef>::into(Block_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMBasicBlockRef> {
  pub unsafe fn LLVMGetInsertBlock<T0_>(Builder_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetInsertBlock(Into::<LLVMBuilderRef>::into(Builder_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMClearInsertionPosition<T0_>(Builder_:  T0_)
  where
     T0_: Into<LLVMBuilderRef>
  {
    unsafe {
      crate::Core::LLVMClearInsertionPosition(Into::<LLVMBuilderRef>::into(Builder_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMInsertIntoBuilder<T0_, T1_>(Builder_:  T0_, Instr_:  T1_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMInsertIntoBuilder(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(Instr_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMInsertIntoBuilderWithName<T0_, T1_, T2_>(Builder_:  T0_, Instr_:  T1_, Name_:  T2_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Core::LLVMInsertIntoBuilderWithName(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(Instr_), Into::<*const std::ffi::c_char>::into(Name_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeBuilder<T0_>(Builder_:  T0_)
  where
     T0_: Into<LLVMBuilderRef>
  {
    unsafe {
      crate::Core::LLVMDisposeBuilder(Into::<LLVMBuilderRef>::into(Builder_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMGetCurrentDebugLocation2<T0_>(Builder_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCurrentDebugLocation2(Into::<LLVMBuilderRef>::into(Builder_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetCurrentDebugLocation2<T0_, T1_>(Builder_:  T0_, Loc_:  T1_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMMetadataRef>
  {
    unsafe {
      crate::Core::LLVMSetCurrentDebugLocation2(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Loc_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetInstDebugLocation<T0_, T1_>(Builder_:  T0_, Inst_:  T1_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetInstDebugLocation(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(Inst_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddMetadataToInst<T0_, T1_>(Builder_:  T0_, Inst_:  T1_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMAddMetadataToInst(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(Inst_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMBuilderGetDefaultFPMathTag<T0_>(Builder_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuilderGetDefaultFPMathTag(Into::<LLVMBuilderRef>::into(Builder_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMBuilderSetDefaultFPMathTag<T0_, T1_>(Builder_:  T0_, FPMathTag_:  T1_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMMetadataRef>
  {
    unsafe {
      crate::Core::LLVMBuilderSetDefaultFPMathTag(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(FPMathTag_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMContextRef> {
  pub unsafe fn LLVMGetBuilderContext<T0_>(Builder_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetBuilderContext(Into::<LLVMBuilderRef>::into(Builder_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetCurrentDebugLocation<T0_, T1_>(Builder_:  T0_, L_:  T1_)
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetCurrentDebugLocation(Into::<LLVMBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(L_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetCurrentDebugLocation<T0_>(Builder_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCurrentDebugLocation(Into::<LLVMBuilderRef>::into(Builder_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildRetVoid<T0_>(arg0_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildRetVoid(Into::<LLVMBuilderRef>::into(arg0_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildRet<T0_, T1_>(arg0_:  T0_, V_:  T1_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildRet(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(V_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildAggregateRet<T0_, T1_, T2_>(arg0_:  T0_, RetVals_:  T1_, N_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<*mut LLVMValueRef>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildAggregateRet(Into::<LLVMBuilderRef>::into(arg0_), Into::<*mut LLVMValueRef>::into(RetVals_), Into::<std::ffi::c_uint>::into(N_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildBr<T0_, T1_>(arg0_:  T0_, Dest_:  T1_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildBr(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMBasicBlockRef>::into(Dest_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildCondBr<T0_, T1_, T2_, T3_>(arg0_:  T0_, If_:  T1_, Then_:  T2_, Else_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMBasicBlockRef>,  T3_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildCondBr(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(If_), Into::<LLVMBasicBlockRef>::into(Then_), Into::<LLVMBasicBlockRef>::into(Else_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildSwitch<T0_, T1_, T2_, T3_>(arg0_:  T0_, V_:  T1_, Else_:  T2_, NumCases_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMBasicBlockRef>,  T3_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildSwitch(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(V_), Into::<LLVMBasicBlockRef>::into(Else_), Into::<std::ffi::c_uint>::into(NumCases_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildIndirectBr<T0_, T1_, T2_>(B_:  T0_, Addr_:  T1_, NumDests_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildIndirectBr(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(Addr_), Into::<std::ffi::c_uint>::into(NumDests_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildCallBr<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_>(B_:  T0_, Ty_:  T1_, Fn_:  T2_, DefaultDest_:  T3_, IndirectDests_:  T4_, NumIndirectDests_:  T5_, Args_:  T6_, NumArgs_:  T7_, Bundles_:  T8_, NumBundles_:  T9_, Name_:  T10_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMBasicBlockRef>,  T4_: Into<*mut LLVMBasicBlockRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<*mut LLVMValueRef>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<*mut LLVMOperandBundleRef>,  T9_: Into<std::ffi::c_uint>,  T10_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildCallBr(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(Fn_), Into::<LLVMBasicBlockRef>::into(DefaultDest_), Into::<*mut LLVMBasicBlockRef>::into(IndirectDests_), Into::<std::ffi::c_uint>::into(NumIndirectDests_), Into::<*mut LLVMValueRef>::into(Args_), Into::<std::ffi::c_uint>::into(NumArgs_), Into::<*mut LLVMOperandBundleRef>::into(Bundles_), Into::<std::ffi::c_uint>::into(NumBundles_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildInvoke2<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_>(arg0_:  T0_, Ty_:  T1_, Fn_:  T2_, Args_:  T3_, NumArgs_:  T4_, Then_:  T5_, Catch_:  T6_, Name_:  T7_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*mut LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<LLVMBasicBlockRef>,  T6_: Into<LLVMBasicBlockRef>,  T7_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildInvoke2(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(Fn_), Into::<*mut LLVMValueRef>::into(Args_), Into::<std::ffi::c_uint>::into(NumArgs_), Into::<LLVMBasicBlockRef>::into(Then_), Into::<LLVMBasicBlockRef>::into(Catch_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildInvokeWithOperandBundles<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_>(arg0_:  T0_, Ty_:  T1_, Fn_:  T2_, Args_:  T3_, NumArgs_:  T4_, Then_:  T5_, Catch_:  T6_, Bundles_:  T7_, NumBundles_:  T8_, Name_:  T9_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*mut LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<LLVMBasicBlockRef>,  T6_: Into<LLVMBasicBlockRef>,  T7_: Into<*mut LLVMOperandBundleRef>,  T8_: Into<std::ffi::c_uint>,  T9_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildInvokeWithOperandBundles(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(Fn_), Into::<*mut LLVMValueRef>::into(Args_), Into::<std::ffi::c_uint>::into(NumArgs_), Into::<LLVMBasicBlockRef>::into(Then_), Into::<LLVMBasicBlockRef>::into(Catch_), Into::<*mut LLVMOperandBundleRef>::into(Bundles_), Into::<std::ffi::c_uint>::into(NumBundles_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildUnreachable<T0_>(arg0_:  T0_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildUnreachable(Into::<LLVMBuilderRef>::into(arg0_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildResume<T0_, T1_>(B_:  T0_, Exn_:  T1_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildResume(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(Exn_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildLandingPad<T0_, T1_, T2_, T3_, T4_>(B_:  T0_, Ty_:  T1_, PersFn_:  T2_, NumClauses_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildLandingPad(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(PersFn_), Into::<std::ffi::c_uint>::into(NumClauses_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildCleanupRet<T0_, T1_, T2_>(B_:  T0_, CatchPad_:  T1_, BB_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildCleanupRet(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(CatchPad_), Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildCatchRet<T0_, T1_, T2_>(B_:  T0_, CatchPad_:  T1_, BB_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildCatchRet(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(CatchPad_), Into::<LLVMBasicBlockRef>::into(BB_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildCatchPad<T0_, T1_, T2_, T3_, T4_>(B_:  T0_, ParentPad_:  T1_, Args_:  T2_, NumArgs_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*mut LLVMValueRef>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildCatchPad(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(ParentPad_), Into::<*mut LLVMValueRef>::into(Args_), Into::<std::ffi::c_uint>::into(NumArgs_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildCleanupPad<T0_, T1_, T2_, T3_, T4_>(B_:  T0_, ParentPad_:  T1_, Args_:  T2_, NumArgs_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*mut LLVMValueRef>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildCleanupPad(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(ParentPad_), Into::<*mut LLVMValueRef>::into(Args_), Into::<std::ffi::c_uint>::into(NumArgs_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildCatchSwitch<T0_, T1_, T2_, T3_, T4_>(B_:  T0_, ParentPad_:  T1_, UnwindBB_:  T2_, NumHandlers_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMBasicBlockRef>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildCatchSwitch(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(ParentPad_), Into::<LLVMBasicBlockRef>::into(UnwindBB_), Into::<std::ffi::c_uint>::into(NumHandlers_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddCase<T0_, T1_, T2_>(Switch_:  T0_, OnVal_:  T1_, Dest_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMAddCase(Into::<LLVMValueRef>::into(Switch_), Into::<LLVMValueRef>::into(OnVal_), Into::<LLVMBasicBlockRef>::into(Dest_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddDestination<T0_, T1_>(IndirectBr_:  T0_, Dest_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMAddDestination(Into::<LLVMValueRef>::into(IndirectBr_), Into::<LLVMBasicBlockRef>::into(Dest_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetNumClauses<T0_>(LandingPad_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNumClauses(Into::<LLVMValueRef>::into(LandingPad_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetClause<T0_, T1_>(LandingPad_:  T0_, Idx_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetClause(Into::<LLVMValueRef>::into(LandingPad_), Into::<std::ffi::c_uint>::into(Idx_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddClause<T0_, T1_>(LandingPad_:  T0_, ClauseVal_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMAddClause(Into::<LLVMValueRef>::into(LandingPad_), Into::<LLVMValueRef>::into(ClauseVal_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsCleanup<T0_>(LandingPad_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsCleanup(Into::<LLVMValueRef>::into(LandingPad_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetCleanup<T0_, T1_>(LandingPad_:  T0_, Val_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetCleanup(Into::<LLVMValueRef>::into(LandingPad_), Into::<std::ffi::c_int>::into(Val_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMAddHandler<T0_, T1_>(CatchSwitch_:  T0_, Dest_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMAddHandler(Into::<LLVMValueRef>::into(CatchSwitch_), Into::<LLVMBasicBlockRef>::into(Dest_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetNumHandlers<T0_>(CatchSwitch_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNumHandlers(Into::<LLVMValueRef>::into(CatchSwitch_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMGetHandlers<T0_, T1_>(CatchSwitch_:  T0_, Handlers_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<*mut LLVMBasicBlockRef>
  {
    unsafe {
      crate::Core::LLVMGetHandlers(Into::<LLVMValueRef>::into(CatchSwitch_), Into::<*mut LLVMBasicBlockRef>::into(Handlers_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetArgOperand<T0_, T1_>(Funclet_:  T0_, i_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetArgOperand(Into::<LLVMValueRef>::into(Funclet_), Into::<std::ffi::c_uint>::into(i_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetArgOperand<T0_, T1_, T2_>(Funclet_:  T0_, i_:  T1_, value_:  T2_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetArgOperand(Into::<LLVMValueRef>::into(Funclet_), Into::<std::ffi::c_uint>::into(i_), Into::<LLVMValueRef>::into(value_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMGetParentCatchSwitch<T0_>(CatchPad_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetParentCatchSwitch(Into::<LLVMValueRef>::into(CatchPad_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetParentCatchSwitch<T0_, T1_>(CatchPad_:  T0_, CatchSwitch_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMValueRef>
  {
    unsafe {
      crate::Core::LLVMSetParentCatchSwitch(Into::<LLVMValueRef>::into(CatchPad_), Into::<LLVMValueRef>::into(CatchSwitch_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildAdd<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildAdd(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildNSWAdd<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildNSWAdd(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildNUWAdd<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildNUWAdd(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFAdd<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFAdd(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildSub<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildSub(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildNSWSub<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildNSWSub(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildNUWSub<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildNUWSub(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFSub<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFSub(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildMul<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildMul(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildNSWMul<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildNSWMul(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildNUWMul<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildNUWMul(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFMul<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFMul(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildUDiv<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildUDiv(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildExactUDiv<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildExactUDiv(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildSDiv<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildSDiv(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildExactSDiv<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildExactSDiv(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFDiv<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFDiv(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildURem<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildURem(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildSRem<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildSRem(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFRem<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFRem(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildShl<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildShl(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildLShr<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildLShr(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildAShr<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildAShr(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildAnd<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildAnd(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildOr<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildOr(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildXor<T0_, T1_, T2_, T3_>(arg0_:  T0_, LHS_:  T1_, RHS_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildXor(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildBinOp<T0_, T1_, T2_, T3_, T4_>(B_:  T0_, Op_:  T1_, LHS_:  T2_, RHS_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildBinOp(Into::<LLVMBuilderRef>::into(B_), Into::<std::ffi::c_uint>::into(Op_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildNeg<T0_, T1_, T2_>(arg0_:  T0_, V_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildNeg(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(V_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildNSWNeg<T0_, T1_, T2_>(B_:  T0_, V_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildNSWNeg(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(V_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildNUWNeg<T0_, T1_, T2_>(B_:  T0_, V_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildNUWNeg(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(V_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFNeg<T0_, T1_, T2_>(arg0_:  T0_, V_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFNeg(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(V_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildNot<T0_, T1_, T2_>(arg0_:  T0_, V_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildNot(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(V_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetNUW<T0_>(ArithInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNUW(Into::<LLVMValueRef>::into(ArithInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetNUW<T0_, T1_>(ArithInst_:  T0_, HasNUW_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetNUW(Into::<LLVMValueRef>::into(ArithInst_), Into::<std::ffi::c_int>::into(HasNUW_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetNSW<T0_>(ArithInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNSW(Into::<LLVMValueRef>::into(ArithInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetNSW<T0_, T1_>(ArithInst_:  T0_, HasNSW_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetNSW(Into::<LLVMValueRef>::into(ArithInst_), Into::<std::ffi::c_int>::into(HasNSW_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetExact<T0_>(DivOrShrInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetExact(Into::<LLVMValueRef>::into(DivOrShrInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetExact<T0_, T1_>(DivOrShrInst_:  T0_, IsExact_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetExact(Into::<LLVMValueRef>::into(DivOrShrInst_), Into::<std::ffi::c_int>::into(IsExact_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetNNeg<T0_>(NonNegInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNNeg(Into::<LLVMValueRef>::into(NonNegInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetNNeg<T0_, T1_>(NonNegInst_:  T0_, IsNonNeg_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetNNeg(Into::<LLVMValueRef>::into(NonNegInst_), Into::<std::ffi::c_int>::into(IsNonNeg_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetFastMathFlags<T0_>(FPMathInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetFastMathFlags(Into::<LLVMValueRef>::into(FPMathInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetFastMathFlags<T0_, T1_>(FPMathInst_:  T0_, FMF_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetFastMathFlags(Into::<LLVMValueRef>::into(FPMathInst_), Into::<std::ffi::c_uint>::into(FMF_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMCanValueUseFastMathFlags<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCanValueUseFastMathFlags(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetIsDisjoint<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetIsDisjoint(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetIsDisjoint<T0_, T1_>(Inst_:  T0_, IsDisjoint_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetIsDisjoint(Into::<LLVMValueRef>::into(Inst_), Into::<std::ffi::c_int>::into(IsDisjoint_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildMalloc<T0_, T1_, T2_>(arg0_:  T0_, Ty_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildMalloc(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(Ty_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildArrayMalloc<T0_, T1_, T2_, T3_>(arg0_:  T0_, Ty_:  T1_, Val_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildArrayMalloc(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(Val_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildMemSet<T0_, T1_, T2_, T3_, T4_>(B_:  T0_, Ptr_:  T1_, Val_:  T2_, Len_:  T3_, Align_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildMemSet(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(Ptr_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMValueRef>::into(Len_), Into::<std::ffi::c_uint>::into(Align_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildMemCpy<T0_, T1_, T2_, T3_, T4_, T5_>(B_:  T0_, Dst_:  T1_, DstAlign_:  T2_, Src_:  T3_, SrcAlign_:  T4_, Size_:  T5_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildMemCpy(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(Dst_), Into::<std::ffi::c_uint>::into(DstAlign_), Into::<LLVMValueRef>::into(Src_), Into::<std::ffi::c_uint>::into(SrcAlign_), Into::<LLVMValueRef>::into(Size_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildMemMove<T0_, T1_, T2_, T3_, T4_, T5_>(B_:  T0_, Dst_:  T1_, DstAlign_:  T2_, Src_:  T3_, SrcAlign_:  T4_, Size_:  T5_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildMemMove(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(Dst_), Into::<std::ffi::c_uint>::into(DstAlign_), Into::<LLVMValueRef>::into(Src_), Into::<std::ffi::c_uint>::into(SrcAlign_), Into::<LLVMValueRef>::into(Size_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildAlloca<T0_, T1_, T2_>(arg0_:  T0_, Ty_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildAlloca(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(Ty_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildArrayAlloca<T0_, T1_, T2_, T3_>(arg0_:  T0_, Ty_:  T1_, Val_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildArrayAlloca(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(Val_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFree<T0_, T1_>(arg0_:  T0_, PointerVal_:  T1_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFree(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(PointerVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildLoad2<T0_, T1_, T2_, T3_>(arg0_:  T0_, Ty_:  T1_, PointerVal_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildLoad2(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(PointerVal_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildStore<T0_, T1_, T2_>(arg0_:  T0_, Val_:  T1_, Ptr_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildStore(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMValueRef>::into(Ptr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildGEP2<T0_, T1_, T2_, T3_, T4_, T5_>(B_:  T0_, Ty_:  T1_, Pointer_:  T2_, Indices_:  T3_, NumIndices_:  T4_, Name_:  T5_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*mut LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildGEP2(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(Pointer_), Into::<*mut LLVMValueRef>::into(Indices_), Into::<std::ffi::c_uint>::into(NumIndices_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildInBoundsGEP2<T0_, T1_, T2_, T3_, T4_, T5_>(B_:  T0_, Ty_:  T1_, Pointer_:  T2_, Indices_:  T3_, NumIndices_:  T4_, Name_:  T5_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*mut LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildInBoundsGEP2(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(Pointer_), Into::<*mut LLVMValueRef>::into(Indices_), Into::<std::ffi::c_uint>::into(NumIndices_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildGEPWithNoWrapFlags<T0_, T1_, T2_, T3_, T4_, T5_, T6_>(B_:  T0_, Ty_:  T1_, Pointer_:  T2_, Indices_:  T3_, NumIndices_:  T4_, Name_:  T5_, NoWrapFlags_:  T6_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*mut LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<*const std::ffi::c_char>,  T6_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildGEPWithNoWrapFlags(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(Pointer_), Into::<*mut LLVMValueRef>::into(Indices_), Into::<std::ffi::c_uint>::into(NumIndices_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_uint>::into(NoWrapFlags_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildStructGEP2<T0_, T1_, T2_, T3_, T4_>(B_:  T0_, Ty_:  T1_, Pointer_:  T2_, Idx_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildStructGEP2(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMTypeRef>::into(Ty_), Into::<LLVMValueRef>::into(Pointer_), Into::<std::ffi::c_uint>::into(Idx_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildGlobalString<T0_, T1_, T2_>(B_:  T0_, Str_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildGlobalString(Into::<LLVMBuilderRef>::into(B_), Into::<*const std::ffi::c_char>::into(Str_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildGlobalStringPtr<T0_, T1_, T2_>(B_:  T0_, Str_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildGlobalStringPtr(Into::<LLVMBuilderRef>::into(B_), Into::<*const std::ffi::c_char>::into(Str_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetVolatile<T0_>(MemoryAccessInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetVolatile(Into::<LLVMValueRef>::into(MemoryAccessInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetVolatile<T0_, T1_>(MemoryAccessInst_:  T0_, IsVolatile_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetVolatile(Into::<LLVMValueRef>::into(MemoryAccessInst_), Into::<std::ffi::c_int>::into(IsVolatile_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetWeak<T0_>(CmpXchgInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetWeak(Into::<LLVMValueRef>::into(CmpXchgInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetWeak<T0_, T1_>(CmpXchgInst_:  T0_, IsWeak_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetWeak(Into::<LLVMValueRef>::into(CmpXchgInst_), Into::<std::ffi::c_int>::into(IsWeak_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetOrdering<T0_>(MemoryAccessInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetOrdering(Into::<LLVMValueRef>::into(MemoryAccessInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetOrdering<T0_, T1_>(MemoryAccessInst_:  T0_, Ordering_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetOrdering(Into::<LLVMValueRef>::into(MemoryAccessInst_), Into::<std::ffi::c_uint>::into(Ordering_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetAtomicRMWBinOp<T0_>(AtomicRMWInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetAtomicRMWBinOp(Into::<LLVMValueRef>::into(AtomicRMWInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetAtomicRMWBinOp<T0_, T1_>(AtomicRMWInst_:  T0_, BinOp_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetAtomicRMWBinOp(Into::<LLVMValueRef>::into(AtomicRMWInst_), Into::<std::ffi::c_uint>::into(BinOp_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildTrunc<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildTrunc(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildZExt<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildZExt(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildSExt<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildSExt(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFPToUI<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFPToUI(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFPToSI<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFPToSI(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildUIToFP<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildUIToFP(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildSIToFP<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildSIToFP(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFPTrunc<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFPTrunc(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFPExt<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFPExt(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildPtrToInt<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildPtrToInt(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildIntToPtr<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildIntToPtr(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildBitCast<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildBitCast(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildAddrSpaceCast<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildAddrSpaceCast(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildZExtOrBitCast<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildZExtOrBitCast(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildSExtOrBitCast<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildSExtOrBitCast(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildTruncOrBitCast<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildTruncOrBitCast(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildCast<T0_, T1_, T2_, T3_, T4_>(B_:  T0_, Op_:  T1_, Val_:  T2_, DestTy_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMTypeRef>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildCast(Into::<LLVMBuilderRef>::into(B_), Into::<std::ffi::c_uint>::into(Op_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildPointerCast<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildPointerCast(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildIntCast2<T0_, T1_, T2_, T3_, T4_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, IsSigned_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<std::ffi::c_int>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildIntCast2(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<std::ffi::c_int>::into(IsSigned_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFPCast<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFPCast(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildIntCast<T0_, T1_, T2_, T3_>(arg0_:  T0_, Val_:  T1_, DestTy_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildIntCast(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMTypeRef>::into(DestTy_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetCastOpcode<T0_, T1_, T2_, T3_>(Src_:  T0_, SrcIsSigned_:  T1_, DestTy_:  T2_, DestIsSigned_:  T3_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>,  T2_: Into<LLVMTypeRef>,  T3_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCastOpcode(Into::<LLVMValueRef>::into(Src_), Into::<std::ffi::c_int>::into(SrcIsSigned_), Into::<LLVMTypeRef>::into(DestTy_), Into::<std::ffi::c_int>::into(DestIsSigned_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildICmp<T0_, T1_, T2_, T3_, T4_>(arg0_:  T0_, Op_:  T1_, LHS_:  T2_, RHS_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildICmp(Into::<LLVMBuilderRef>::into(arg0_), Into::<std::ffi::c_uint>::into(Op_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFCmp<T0_, T1_, T2_, T3_, T4_>(arg0_:  T0_, Op_:  T1_, LHS_:  T2_, RHS_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFCmp(Into::<LLVMBuilderRef>::into(arg0_), Into::<std::ffi::c_uint>::into(Op_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildPhi<T0_, T1_, T2_>(arg0_:  T0_, Ty_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildPhi(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(Ty_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildCall2<T0_, T1_, T2_, T3_, T4_, T5_>(arg0_:  T0_, arg1_:  T1_, Fn_:  T2_, Args_:  T3_, NumArgs_:  T4_, Name_:  T5_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*mut LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildCall2(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(arg1_), Into::<LLVMValueRef>::into(Fn_), Into::<*mut LLVMValueRef>::into(Args_), Into::<std::ffi::c_uint>::into(NumArgs_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildCallWithOperandBundles<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_>(arg0_:  T0_, arg1_:  T1_, Fn_:  T2_, Args_:  T3_, NumArgs_:  T4_, Bundles_:  T5_, NumBundles_:  T6_, Name_:  T7_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*mut LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<*mut LLVMOperandBundleRef>,  T6_: Into<std::ffi::c_uint>,  T7_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildCallWithOperandBundles(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(arg1_), Into::<LLVMValueRef>::into(Fn_), Into::<*mut LLVMValueRef>::into(Args_), Into::<std::ffi::c_uint>::into(NumArgs_), Into::<*mut LLVMOperandBundleRef>::into(Bundles_), Into::<std::ffi::c_uint>::into(NumBundles_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildSelect<T0_, T1_, T2_, T3_, T4_>(arg0_:  T0_, If_:  T1_, Then_:  T2_, Else_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildSelect(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(If_), Into::<LLVMValueRef>::into(Then_), Into::<LLVMValueRef>::into(Else_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildVAArg<T0_, T1_, T2_, T3_>(arg0_:  T0_, List_:  T1_, Ty_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMTypeRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildVAArg(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(List_), Into::<LLVMTypeRef>::into(Ty_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildExtractElement<T0_, T1_, T2_, T3_>(arg0_:  T0_, VecVal_:  T1_, Index_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildExtractElement(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(VecVal_), Into::<LLVMValueRef>::into(Index_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildInsertElement<T0_, T1_, T2_, T3_, T4_>(arg0_:  T0_, VecVal_:  T1_, EltVal_:  T2_, Index_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildInsertElement(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(VecVal_), Into::<LLVMValueRef>::into(EltVal_), Into::<LLVMValueRef>::into(Index_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildShuffleVector<T0_, T1_, T2_, T3_, T4_>(arg0_:  T0_, V1_:  T1_, V2_:  T2_, Mask_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildShuffleVector(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(V1_), Into::<LLVMValueRef>::into(V2_), Into::<LLVMValueRef>::into(Mask_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildExtractValue<T0_, T1_, T2_, T3_>(arg0_:  T0_, AggVal_:  T1_, Index_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildExtractValue(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(AggVal_), Into::<std::ffi::c_uint>::into(Index_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildInsertValue<T0_, T1_, T2_, T3_, T4_>(arg0_:  T0_, AggVal_:  T1_, EltVal_:  T2_, Index_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildInsertValue(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(AggVal_), Into::<LLVMValueRef>::into(EltVal_), Into::<std::ffi::c_uint>::into(Index_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFreeze<T0_, T1_, T2_>(arg0_:  T0_, Val_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFreeze(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildIsNull<T0_, T1_, T2_>(arg0_:  T0_, Val_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildIsNull(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildIsNotNull<T0_, T1_, T2_>(arg0_:  T0_, Val_:  T1_, Name_:  T2_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildIsNotNull(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMValueRef>::into(Val_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildPtrDiff2<T0_, T1_, T2_, T3_, T4_>(arg0_:  T0_, ElemTy_:  T1_, LHS_:  T2_, RHS_:  T3_, Name_:  T4_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMTypeRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildPtrDiff2(Into::<LLVMBuilderRef>::into(arg0_), Into::<LLVMTypeRef>::into(ElemTy_), Into::<LLVMValueRef>::into(LHS_), Into::<LLVMValueRef>::into(RHS_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFence<T0_, T1_, T2_, T3_>(B_:  T0_, ordering_:  T1_, singleThread_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_int>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFence(Into::<LLVMBuilderRef>::into(B_), Into::<std::ffi::c_uint>::into(ordering_), Into::<std::ffi::c_int>::into(singleThread_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildFenceSyncScope<T0_, T1_, T2_, T3_>(B_:  T0_, ordering_:  T1_, SSID_:  T2_, Name_:  T3_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildFenceSyncScope(Into::<LLVMBuilderRef>::into(B_), Into::<std::ffi::c_uint>::into(ordering_), Into::<std::ffi::c_uint>::into(SSID_), Into::<*const std::ffi::c_char>::into(Name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildAtomicRMW<T0_, T1_, T2_, T3_, T4_, T5_>(B_:  T0_, op_:  T1_, PTR_:  T2_, Val_:  T3_, ordering_:  T4_, singleThread_:  T5_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildAtomicRMW(Into::<LLVMBuilderRef>::into(B_), Into::<std::ffi::c_uint>::into(op_), Into::<LLVMValueRef>::into(PTR_), Into::<LLVMValueRef>::into(Val_), Into::<std::ffi::c_uint>::into(ordering_), Into::<std::ffi::c_int>::into(singleThread_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildAtomicRMWSyncScope<T0_, T1_, T2_, T3_, T4_, T5_>(B_:  T0_, op_:  T1_, PTR_:  T2_, Val_:  T3_, ordering_:  T4_, SSID_:  T5_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildAtomicRMWSyncScope(Into::<LLVMBuilderRef>::into(B_), Into::<std::ffi::c_uint>::into(op_), Into::<LLVMValueRef>::into(PTR_), Into::<LLVMValueRef>::into(Val_), Into::<std::ffi::c_uint>::into(ordering_), Into::<std::ffi::c_uint>::into(SSID_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildAtomicCmpXchg<T0_, T1_, T2_, T3_, T4_, T5_, T6_>(B_:  T0_, Ptr_:  T1_, Cmp_:  T2_, New_:  T3_, SuccessOrdering_:  T4_, FailureOrdering_:  T5_, SingleThread_:  T6_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildAtomicCmpXchg(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(Ptr_), Into::<LLVMValueRef>::into(Cmp_), Into::<LLVMValueRef>::into(New_), Into::<std::ffi::c_uint>::into(SuccessOrdering_), Into::<std::ffi::c_uint>::into(FailureOrdering_), Into::<std::ffi::c_int>::into(SingleThread_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMValueRef> {
  pub unsafe fn LLVMBuildAtomicCmpXchgSyncScope<T0_, T1_, T2_, T3_, T4_, T5_, T6_>(B_:  T0_, Ptr_:  T1_, Cmp_:  T2_, New_:  T3_, SuccessOrdering_:  T4_, FailureOrdering_:  T5_, SSID_:  T6_)-> Tret_
  where
     T0_: Into<LLVMBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMValueRef>,  T3_: Into<LLVMValueRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMBuildAtomicCmpXchgSyncScope(Into::<LLVMBuilderRef>::into(B_), Into::<LLVMValueRef>::into(Ptr_), Into::<LLVMValueRef>::into(Cmp_), Into::<LLVMValueRef>::into(New_), Into::<std::ffi::c_uint>::into(SuccessOrdering_), Into::<std::ffi::c_uint>::into(FailureOrdering_), Into::<std::ffi::c_uint>::into(SSID_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetNumMaskElements<T0_>(ShuffleVectorInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetNumMaskElements(Into::<LLVMValueRef>::into(ShuffleVectorInst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetUndefMaskElem()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetUndefMaskElem()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMGetMaskValue<T0_, T1_>(ShuffleVectorInst_:  T0_, Elt_:  T1_)-> Tret_
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetMaskValue(Into::<LLVMValueRef>::into(ShuffleVectorInst_), Into::<std::ffi::c_uint>::into(Elt_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsAtomicSingleThread<T0_>(AtomicInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAtomicSingleThread(Into::<LLVMValueRef>::into(AtomicInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetAtomicSingleThread<T0_, T1_>(AtomicInst_:  T0_, SingleThread_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_int>
  {
    unsafe {
      crate::Core::LLVMSetAtomicSingleThread(Into::<LLVMValueRef>::into(AtomicInst_), Into::<std::ffi::c_int>::into(SingleThread_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsAtomic<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsAtomic(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetAtomicSyncScopeID<T0_>(AtomicInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetAtomicSyncScopeID(Into::<LLVMValueRef>::into(AtomicInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetAtomicSyncScopeID<T0_, T1_>(AtomicInst_:  T0_, SSID_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetAtomicSyncScopeID(Into::<LLVMValueRef>::into(AtomicInst_), Into::<std::ffi::c_uint>::into(SSID_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetCmpXchgSuccessOrdering<T0_>(CmpXchgInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCmpXchgSuccessOrdering(Into::<LLVMValueRef>::into(CmpXchgInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetCmpXchgSuccessOrdering<T0_, T1_>(CmpXchgInst_:  T0_, Ordering_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetCmpXchgSuccessOrdering(Into::<LLVMValueRef>::into(CmpXchgInst_), Into::<std::ffi::c_uint>::into(Ordering_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetCmpXchgFailureOrdering<T0_>(CmpXchgInst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetCmpXchgFailureOrdering(Into::<LLVMValueRef>::into(CmpXchgInst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetCmpXchgFailureOrdering<T0_, T1_>(CmpXchgInst_:  T0_, Ordering_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::Core::LLVMSetCmpXchgFailureOrdering(Into::<LLVMValueRef>::into(CmpXchgInst_), Into::<std::ffi::c_uint>::into(Ordering_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMModuleProviderRef> {
  pub unsafe fn LLVMCreateModuleProviderForExistingModule<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateModuleProviderForExistingModule(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeModuleProvider<T0_>(M_:  T0_)
  where
     T0_: Into<LLVMModuleProviderRef>
  {
    unsafe {
      crate::Core::LLVMDisposeModuleProvider(Into::<LLVMModuleProviderRef>::into(M_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMCreateMemoryBufferWithContentsOfFile<T0_, T1_, T2_>(Path_:  T0_, OutMemBuf_:  T1_, OutMessage_:  T2_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<*mut LLVMMemoryBufferRef>,  T2_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateMemoryBufferWithContentsOfFile(Into::<*const std::ffi::c_char>::into(Path_), Into::<*mut LLVMMemoryBufferRef>::into(OutMemBuf_), Into::<*mut *mut std::ffi::c_char>::into(OutMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMCreateMemoryBufferWithSTDIN<T0_, T1_>(OutMemBuf_:  T0_, OutMessage_:  T1_)-> Tret_
  where
     T0_: Into<*mut LLVMMemoryBufferRef>,  T1_: Into<*mut *mut std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateMemoryBufferWithSTDIN(Into::<*mut LLVMMemoryBufferRef>::into(OutMemBuf_), Into::<*mut *mut std::ffi::c_char>::into(OutMessage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMemoryBufferRef> {
  pub unsafe fn LLVMCreateMemoryBufferWithMemoryRange<T0_, T1_, T2_, T3_>(InputData_:  T0_, InputDataLength_:  T1_, BufferName_:  T2_, RequiresNullTerminator_:  T3_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateMemoryBufferWithMemoryRange(Into::<*const std::ffi::c_char>::into(InputData_), Into::<std::ffi::c_ulong>::into(InputDataLength_), Into::<*const std::ffi::c_char>::into(BufferName_), Into::<std::ffi::c_int>::into(RequiresNullTerminator_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMemoryBufferRef> {
  pub unsafe fn LLVMCreateMemoryBufferWithMemoryRangeCopy<T0_, T1_, T2_>(InputData_:  T0_, InputDataLength_:  T1_, BufferName_:  T2_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateMemoryBufferWithMemoryRangeCopy(Into::<*const std::ffi::c_char>::into(InputData_), Into::<std::ffi::c_ulong>::into(InputDataLength_), Into::<*const std::ffi::c_char>::into(BufferName_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMGetBufferStart<T0_>(MemBuf_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMemoryBufferRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetBufferStart(Into::<LLVMMemoryBufferRef>::into(MemBuf_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMGetBufferSize<T0_>(MemBuf_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMemoryBufferRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMGetBufferSize(Into::<LLVMMemoryBufferRef>::into(MemBuf_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeMemoryBuffer<T0_>(MemBuf_:  T0_)
  where
     T0_: Into<LLVMMemoryBufferRef>
  {
    unsafe {
      crate::Core::LLVMDisposeMemoryBuffer(Into::<LLVMMemoryBufferRef>::into(MemBuf_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMPassManagerRef> {
  pub unsafe fn LLVMCreatePassManager()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreatePassManager()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMPassManagerRef> {
  pub unsafe fn LLVMCreateFunctionPassManagerForModule<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateFunctionPassManagerForModule(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMPassManagerRef> {
  pub unsafe fn LLVMCreateFunctionPassManager<T0_>(MP_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleProviderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMCreateFunctionPassManager(Into::<LLVMModuleProviderRef>::into(MP_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMRunPassManager<T0_, T1_>(PM_:  T0_, M_:  T1_)-> Tret_
  where
     T0_: Into<LLVMPassManagerRef>,  T1_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMRunPassManager(Into::<LLVMPassManagerRef>::into(PM_), Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMInitializeFunctionPassManager<T0_>(FPM_:  T0_)-> Tret_
  where
     T0_: Into<LLVMPassManagerRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMInitializeFunctionPassManager(Into::<LLVMPassManagerRef>::into(FPM_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMRunFunctionPassManager<T0_, T1_>(FPM_:  T0_, F_:  T1_)-> Tret_
  where
     T0_: Into<LLVMPassManagerRef>,  T1_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMRunFunctionPassManager(Into::<LLVMPassManagerRef>::into(FPM_), Into::<LLVMValueRef>::into(F_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMFinalizeFunctionPassManager<T0_>(FPM_:  T0_)-> Tret_
  where
     T0_: Into<LLVMPassManagerRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMFinalizeFunctionPassManager(Into::<LLVMPassManagerRef>::into(FPM_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposePassManager<T0_>(PM_:  T0_)
  where
     T0_: Into<LLVMPassManagerRef>
  {
    unsafe {
      crate::Core::LLVMDisposePassManager(Into::<LLVMPassManagerRef>::into(PM_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMStartMultithreaded()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMStartMultithreaded()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMIsMultithreaded()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Core::LLVMIsMultithreaded()
      }
    )
  }
}

