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

pub const LLVMDIFlagZero: std::ffi::c_uint = 0;
pub const LLVMDIFlagPrivate: std::ffi::c_uint = 1;
pub const LLVMDIFlagProtected: std::ffi::c_uint = 2;
pub const LLVMDIFlagPublic: std::ffi::c_uint = 3;
pub const LLVMDIFlagFwdDecl: std::ffi::c_uint = 4;
pub const LLVMDIFlagAppleBlock: std::ffi::c_uint = 8;
pub const LLVMDIFlagReservedBit4: std::ffi::c_uint = 16;
pub const LLVMDIFlagVirtual: std::ffi::c_uint = 32;
pub const LLVMDIFlagArtificial: std::ffi::c_uint = 64;
pub const LLVMDIFlagExplicit: std::ffi::c_uint = 128;
pub const LLVMDIFlagPrototyped: std::ffi::c_uint = 256;
pub const LLVMDIFlagObjcClassComplete: std::ffi::c_uint = 512;
pub const LLVMDIFlagObjectPointer: std::ffi::c_uint = 1024;
pub const LLVMDIFlagVector: std::ffi::c_uint = 2048;
pub const LLVMDIFlagStaticMember: std::ffi::c_uint = 4096;
pub const LLVMDIFlagLValueReference: std::ffi::c_uint = 8192;
pub const LLVMDIFlagRValueReference: std::ffi::c_uint = 16384;
pub const LLVMDIFlagReserved: std::ffi::c_uint = 32768;
pub const LLVMDIFlagSingleInheritance: std::ffi::c_uint = 65536;
pub const LLVMDIFlagMultipleInheritance: std::ffi::c_uint = 131072;
pub const LLVMDIFlagVirtualInheritance: std::ffi::c_uint = 196608;
pub const LLVMDIFlagIntroducedVirtual: std::ffi::c_uint = 262144;
pub const LLVMDIFlagBitField: std::ffi::c_uint = 524288;
pub const LLVMDIFlagNoReturn: std::ffi::c_uint = 1048576;
pub const LLVMDIFlagTypePassByValue: std::ffi::c_uint = 4194304;
pub const LLVMDIFlagTypePassByReference: std::ffi::c_uint = 8388608;
pub const LLVMDIFlagEnumClass: std::ffi::c_uint = 16777216;
pub const LLVMDIFlagFixedEnum: std::ffi::c_uint = 16777216;
pub const LLVMDIFlagThunk: std::ffi::c_uint = 33554432;
pub const LLVMDIFlagNonTrivial: std::ffi::c_uint = 67108864;
pub const LLVMDIFlagBigEndian: std::ffi::c_uint = 134217728;
pub const LLVMDIFlagLittleEndian: std::ffi::c_uint = 268435456;
pub const LLVMDIFlagIndirectVirtualBase: std::ffi::c_uint = 36;
pub const LLVMDIFlagAccessibility: std::ffi::c_uint = 3;
pub const LLVMDIFlagPtrToMemberRep: std::ffi::c_uint = 196608;
pub type LLVMDIFlags = std::ffi::c_uint;

pub const LLVMDWARFSourceLanguageC89: std::ffi::c_uint = 0;
pub const LLVMDWARFSourceLanguageC: std::ffi::c_uint = 1;
pub const LLVMDWARFSourceLanguageAda83: std::ffi::c_uint = 2;
pub const LLVMDWARFSourceLanguageC_plus_plus: std::ffi::c_uint = 3;
pub const LLVMDWARFSourceLanguageCobol74: std::ffi::c_uint = 4;
pub const LLVMDWARFSourceLanguageCobol85: std::ffi::c_uint = 5;
pub const LLVMDWARFSourceLanguageFortran77: std::ffi::c_uint = 6;
pub const LLVMDWARFSourceLanguageFortran90: std::ffi::c_uint = 7;
pub const LLVMDWARFSourceLanguagePascal83: std::ffi::c_uint = 8;
pub const LLVMDWARFSourceLanguageModula2: std::ffi::c_uint = 9;
pub const LLVMDWARFSourceLanguageJava: std::ffi::c_uint = 10;
pub const LLVMDWARFSourceLanguageC99: std::ffi::c_uint = 11;
pub const LLVMDWARFSourceLanguageAda95: std::ffi::c_uint = 12;
pub const LLVMDWARFSourceLanguageFortran95: std::ffi::c_uint = 13;
pub const LLVMDWARFSourceLanguagePLI: std::ffi::c_uint = 14;
pub const LLVMDWARFSourceLanguageObjC: std::ffi::c_uint = 15;
pub const LLVMDWARFSourceLanguageObjC_plus_plus: std::ffi::c_uint = 16;
pub const LLVMDWARFSourceLanguageUPC: std::ffi::c_uint = 17;
pub const LLVMDWARFSourceLanguageD: std::ffi::c_uint = 18;
pub const LLVMDWARFSourceLanguagePython: std::ffi::c_uint = 19;
pub const LLVMDWARFSourceLanguageOpenCL: std::ffi::c_uint = 20;
pub const LLVMDWARFSourceLanguageGo: std::ffi::c_uint = 21;
pub const LLVMDWARFSourceLanguageModula3: std::ffi::c_uint = 22;
pub const LLVMDWARFSourceLanguageHaskell: std::ffi::c_uint = 23;
pub const LLVMDWARFSourceLanguageC_plus_plus_03: std::ffi::c_uint = 24;
pub const LLVMDWARFSourceLanguageC_plus_plus_11: std::ffi::c_uint = 25;
pub const LLVMDWARFSourceLanguageOCaml: std::ffi::c_uint = 26;
pub const LLVMDWARFSourceLanguageRust: std::ffi::c_uint = 27;
pub const LLVMDWARFSourceLanguageC11: std::ffi::c_uint = 28;
pub const LLVMDWARFSourceLanguageSwift: std::ffi::c_uint = 29;
pub const LLVMDWARFSourceLanguageJulia: std::ffi::c_uint = 30;
pub const LLVMDWARFSourceLanguageDylan: std::ffi::c_uint = 31;
pub const LLVMDWARFSourceLanguageC_plus_plus_14: std::ffi::c_uint = 32;
pub const LLVMDWARFSourceLanguageFortran03: std::ffi::c_uint = 33;
pub const LLVMDWARFSourceLanguageFortran08: std::ffi::c_uint = 34;
pub const LLVMDWARFSourceLanguageRenderScript: std::ffi::c_uint = 35;
pub const LLVMDWARFSourceLanguageBLISS: std::ffi::c_uint = 36;
pub const LLVMDWARFSourceLanguageKotlin: std::ffi::c_uint = 37;
pub const LLVMDWARFSourceLanguageZig: std::ffi::c_uint = 38;
pub const LLVMDWARFSourceLanguageCrystal: std::ffi::c_uint = 39;
pub const LLVMDWARFSourceLanguageC_plus_plus_17: std::ffi::c_uint = 40;
pub const LLVMDWARFSourceLanguageC_plus_plus_20: std::ffi::c_uint = 41;
pub const LLVMDWARFSourceLanguageC17: std::ffi::c_uint = 42;
pub const LLVMDWARFSourceLanguageFortran18: std::ffi::c_uint = 43;
pub const LLVMDWARFSourceLanguageAda2005: std::ffi::c_uint = 44;
pub const LLVMDWARFSourceLanguageAda2012: std::ffi::c_uint = 45;
pub const LLVMDWARFSourceLanguageHIP: std::ffi::c_uint = 46;
pub const LLVMDWARFSourceLanguageAssembly: std::ffi::c_uint = 47;
pub const LLVMDWARFSourceLanguageC_sharp: std::ffi::c_uint = 48;
pub const LLVMDWARFSourceLanguageMojo: std::ffi::c_uint = 49;
pub const LLVMDWARFSourceLanguageGLSL: std::ffi::c_uint = 50;
pub const LLVMDWARFSourceLanguageGLSL_ES: std::ffi::c_uint = 51;
pub const LLVMDWARFSourceLanguageHLSL: std::ffi::c_uint = 52;
pub const LLVMDWARFSourceLanguageOpenCL_CPP: std::ffi::c_uint = 53;
pub const LLVMDWARFSourceLanguageCPP_for_OpenCL: std::ffi::c_uint = 54;
pub const LLVMDWARFSourceLanguageSYCL: std::ffi::c_uint = 55;
pub const LLVMDWARFSourceLanguageRuby: std::ffi::c_uint = 56;
pub const LLVMDWARFSourceLanguageMove: std::ffi::c_uint = 57;
pub const LLVMDWARFSourceLanguageHylo: std::ffi::c_uint = 58;
pub const LLVMDWARFSourceLanguageMetal: std::ffi::c_uint = 59;
pub const LLVMDWARFSourceLanguageMips_Assembler: std::ffi::c_uint = 60;
pub const LLVMDWARFSourceLanguageGOOGLE_RenderScript: std::ffi::c_uint = 61;
pub const LLVMDWARFSourceLanguageBORLAND_Delphi: std::ffi::c_uint = 62;
pub type LLVMDWARFSourceLanguage = std::ffi::c_uint;

pub const LLVMDWARFEmissionNone: std::ffi::c_uint = 0;
pub const LLVMDWARFEmissionFull: std::ffi::c_uint = 1;
pub const LLVMDWARFEmissionLineTablesOnly: std::ffi::c_uint = 2;
pub type LLVMDWARFEmissionKind = std::ffi::c_uint;

pub const LLVMMDStringMetadataKind: std::ffi::c_uint = 0;
pub const LLVMConstantAsMetadataMetadataKind: std::ffi::c_uint = 1;
pub const LLVMLocalAsMetadataMetadataKind: std::ffi::c_uint = 2;
pub const LLVMDistinctMDOperandPlaceholderMetadataKind: std::ffi::c_uint = 3;
pub const LLVMMDTupleMetadataKind: std::ffi::c_uint = 4;
pub const LLVMDILocationMetadataKind: std::ffi::c_uint = 5;
pub const LLVMDIExpressionMetadataKind: std::ffi::c_uint = 6;
pub const LLVMDIGlobalVariableExpressionMetadataKind: std::ffi::c_uint = 7;
pub const LLVMGenericDINodeMetadataKind: std::ffi::c_uint = 8;
pub const LLVMDISubrangeMetadataKind: std::ffi::c_uint = 9;
pub const LLVMDIEnumeratorMetadataKind: std::ffi::c_uint = 10;
pub const LLVMDIBasicTypeMetadataKind: std::ffi::c_uint = 11;
pub const LLVMDIDerivedTypeMetadataKind: std::ffi::c_uint = 12;
pub const LLVMDICompositeTypeMetadataKind: std::ffi::c_uint = 13;
pub const LLVMDISubroutineTypeMetadataKind: std::ffi::c_uint = 14;
pub const LLVMDIFileMetadataKind: std::ffi::c_uint = 15;
pub const LLVMDICompileUnitMetadataKind: std::ffi::c_uint = 16;
pub const LLVMDISubprogramMetadataKind: std::ffi::c_uint = 17;
pub const LLVMDILexicalBlockMetadataKind: std::ffi::c_uint = 18;
pub const LLVMDILexicalBlockFileMetadataKind: std::ffi::c_uint = 19;
pub const LLVMDINamespaceMetadataKind: std::ffi::c_uint = 20;
pub const LLVMDIModuleMetadataKind: std::ffi::c_uint = 21;
pub const LLVMDITemplateTypeParameterMetadataKind: std::ffi::c_uint = 22;
pub const LLVMDITemplateValueParameterMetadataKind: std::ffi::c_uint = 23;
pub const LLVMDIGlobalVariableMetadataKind: std::ffi::c_uint = 24;
pub const LLVMDILocalVariableMetadataKind: std::ffi::c_uint = 25;
pub const LLVMDILabelMetadataKind: std::ffi::c_uint = 26;
pub const LLVMDIObjCPropertyMetadataKind: std::ffi::c_uint = 27;
pub const LLVMDIImportedEntityMetadataKind: std::ffi::c_uint = 28;
pub const LLVMDIMacroMetadataKind: std::ffi::c_uint = 29;
pub const LLVMDIMacroFileMetadataKind: std::ffi::c_uint = 30;
pub const LLVMDICommonBlockMetadataKind: std::ffi::c_uint = 31;
pub const LLVMDIStringTypeMetadataKind: std::ffi::c_uint = 32;
pub const LLVMDIGenericSubrangeMetadataKind: std::ffi::c_uint = 33;
pub const LLVMDIArgListMetadataKind: std::ffi::c_uint = 34;
pub const LLVMDIAssignIDMetadataKind: std::ffi::c_uint = 35;
pub type LLVMMetadataKind = std::ffi::c_uint;
pub type LLVMDWARFTypeEncoding = std::ffi::c_uint;

pub const LLVMDWARFMacinfoRecordTypeDefine: std::ffi::c_uint = 1;
pub const LLVMDWARFMacinfoRecordTypeMacro: std::ffi::c_uint = 2;
pub const LLVMDWARFMacinfoRecordTypeStartFile: std::ffi::c_uint = 3;
pub const LLVMDWARFMacinfoRecordTypeEndFile: std::ffi::c_uint = 4;
pub const LLVMDWARFMacinfoRecordTypeVendorExt: std::ffi::c_uint = 255;
pub type LLVMDWARFMacinfoRecordType = std::ffi::c_uint;

#[link(name = "LLVM")]
extern {

  pub fn LLVMDebugMetadataVersion() -> std::ffi::c_uint;

  pub fn LLVMGetModuleDebugMetadataVersion(Module: LLVMModuleRef) -> std::ffi::c_uint;

  pub fn LLVMStripModuleDebugInfo(Module: LLVMModuleRef) -> std::ffi::c_int;

  pub fn LLVMCreateDIBuilderDisallowUnresolved(M: LLVMModuleRef) -> LLVMDIBuilderRef;

  pub fn LLVMCreateDIBuilder(M: LLVMModuleRef) -> LLVMDIBuilderRef;

  pub fn LLVMDisposeDIBuilder(Builder: LLVMDIBuilderRef) -> ();

  pub fn LLVMDIBuilderFinalize(Builder: LLVMDIBuilderRef) -> ();

  pub fn LLVMDIBuilderFinalizeSubprogram(Builder: LLVMDIBuilderRef, Subprogram: LLVMMetadataRef) -> ();

  pub fn LLVMDIBuilderCreateCompileUnit(Builder: LLVMDIBuilderRef, Lang: std::ffi::c_uint, FileRef: LLVMMetadataRef, Producer: *const std::ffi::c_char, ProducerLen: std::ffi::c_ulong, isOptimized: std::ffi::c_int, Flags: *const std::ffi::c_char, FlagsLen: std::ffi::c_ulong, RuntimeVer: std::ffi::c_uint, SplitName: *const std::ffi::c_char, SplitNameLen: std::ffi::c_ulong, Kind: std::ffi::c_uint, DWOId: std::ffi::c_uint, SplitDebugInlining: std::ffi::c_int, DebugInfoForProfiling: std::ffi::c_int, SysRoot: *const std::ffi::c_char, SysRootLen: std::ffi::c_ulong, SDK: *const std::ffi::c_char, SDKLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateFile(Builder: LLVMDIBuilderRef, Filename: *const std::ffi::c_char, FilenameLen: std::ffi::c_ulong, Directory: *const std::ffi::c_char, DirectoryLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateModule(Builder: LLVMDIBuilderRef, ParentScope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, ConfigMacros: *const std::ffi::c_char, ConfigMacrosLen: std::ffi::c_ulong, IncludePath: *const std::ffi::c_char, IncludePathLen: std::ffi::c_ulong, APINotesFile: *const std::ffi::c_char, APINotesFileLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateNameSpace(Builder: LLVMDIBuilderRef, ParentScope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, ExportSymbols: std::ffi::c_int) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateFunction(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, LinkageName: *const std::ffi::c_char, LinkageNameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNo: std::ffi::c_uint, Ty: LLVMMetadataRef, IsLocalToUnit: std::ffi::c_int, IsDefinition: std::ffi::c_int, ScopeLine: std::ffi::c_uint, Flags: std::ffi::c_uint, IsOptimized: std::ffi::c_int) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateLexicalBlock(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, File: LLVMMetadataRef, Line: std::ffi::c_uint, Column: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateLexicalBlockFile(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, File: LLVMMetadataRef, Discriminator: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateImportedModuleFromNamespace(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, NS: LLVMMetadataRef, File: LLVMMetadataRef, Line: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateImportedModuleFromAlias(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, ImportedEntity: LLVMMetadataRef, File: LLVMMetadataRef, Line: std::ffi::c_uint, Elements: *mut LLVMMetadataRef, NumElements: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateImportedModuleFromModule(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, M: LLVMMetadataRef, File: LLVMMetadataRef, Line: std::ffi::c_uint, Elements: *mut LLVMMetadataRef, NumElements: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateImportedDeclaration(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Decl: LLVMMetadataRef, File: LLVMMetadataRef, Line: std::ffi::c_uint, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, Elements: *mut LLVMMetadataRef, NumElements: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateDebugLocation(Ctx: LLVMContextRef, Line: std::ffi::c_uint, Column: std::ffi::c_uint, Scope: LLVMMetadataRef, InlinedAt: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDILocationGetLine(Location: LLVMMetadataRef) -> std::ffi::c_uint;

  pub fn LLVMDILocationGetColumn(Location: LLVMMetadataRef) -> std::ffi::c_uint;

  pub fn LLVMDILocationGetScope(Location: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDILocationGetInlinedAt(Location: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIScopeGetFile(Scope: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIFileGetDirectory(File: LLVMMetadataRef, Len: *mut std::ffi::c_uint) -> *const std::ffi::c_char;

  pub fn LLVMDIFileGetFilename(File: LLVMMetadataRef, Len: *mut std::ffi::c_uint) -> *const std::ffi::c_char;

  pub fn LLVMDIFileGetSource(File: LLVMMetadataRef, Len: *mut std::ffi::c_uint) -> *const std::ffi::c_char;

  pub fn LLVMDIBuilderGetOrCreateTypeArray(Builder: LLVMDIBuilderRef, Data: *mut LLVMMetadataRef, NumElements: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateSubroutineType(Builder: LLVMDIBuilderRef, File: LLVMMetadataRef, ParameterTypes: *mut LLVMMetadataRef, NumParameterTypes: std::ffi::c_uint, Flags: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateMacro(Builder: LLVMDIBuilderRef, ParentMacroFile: LLVMMetadataRef, Line: std::ffi::c_uint, RecordType: std::ffi::c_uint, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, Value: *const std::ffi::c_char, ValueLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateTempMacroFile(Builder: LLVMDIBuilderRef, ParentMacroFile: LLVMMetadataRef, Line: std::ffi::c_uint, File: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateEnumerator(Builder: LLVMDIBuilderRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, Value: std::ffi::c_long, IsUnsigned: std::ffi::c_int) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateEnumerationType(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNumber: std::ffi::c_uint, SizeInBits: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, Elements: *mut LLVMMetadataRef, NumElements: std::ffi::c_uint, ClassTy: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateUnionType(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNumber: std::ffi::c_uint, SizeInBits: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, Flags: std::ffi::c_uint, Elements: *mut LLVMMetadataRef, NumElements: std::ffi::c_uint, RunTimeLang: std::ffi::c_uint, UniqueId: *const std::ffi::c_char, UniqueIdLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateArrayType(Builder: LLVMDIBuilderRef, Size: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, Ty: LLVMMetadataRef, Subscripts: *mut LLVMMetadataRef, NumSubscripts: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateVectorType(Builder: LLVMDIBuilderRef, Size: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, Ty: LLVMMetadataRef, Subscripts: *mut LLVMMetadataRef, NumSubscripts: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateUnspecifiedType(Builder: LLVMDIBuilderRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateBasicType(Builder: LLVMDIBuilderRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, SizeInBits: std::ffi::c_ulong, Encoding: std::ffi::c_uint, Flags: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreatePointerType(Builder: LLVMDIBuilderRef, PointeeTy: LLVMMetadataRef, SizeInBits: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, AddressSpace: std::ffi::c_uint, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateStructType(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNumber: std::ffi::c_uint, SizeInBits: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, Flags: std::ffi::c_uint, DerivedFrom: LLVMMetadataRef, Elements: *mut LLVMMetadataRef, NumElements: std::ffi::c_uint, RunTimeLang: std::ffi::c_uint, VTableHolder: LLVMMetadataRef, UniqueId: *const std::ffi::c_char, UniqueIdLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateMemberType(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNo: std::ffi::c_uint, SizeInBits: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, OffsetInBits: std::ffi::c_ulong, Flags: std::ffi::c_uint, Ty: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateStaticMemberType(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNumber: std::ffi::c_uint, Type: LLVMMetadataRef, Flags: std::ffi::c_uint, ConstantVal: LLVMValueRef, AlignInBits: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateMemberPointerType(Builder: LLVMDIBuilderRef, PointeeType: LLVMMetadataRef, ClassType: LLVMMetadataRef, SizeInBits: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, Flags: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateObjCIVar(Builder: LLVMDIBuilderRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNo: std::ffi::c_uint, SizeInBits: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, OffsetInBits: std::ffi::c_ulong, Flags: std::ffi::c_uint, Ty: LLVMMetadataRef, PropertyNode: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateObjCProperty(Builder: LLVMDIBuilderRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNo: std::ffi::c_uint, GetterName: *const std::ffi::c_char, GetterNameLen: std::ffi::c_ulong, SetterName: *const std::ffi::c_char, SetterNameLen: std::ffi::c_ulong, PropertyAttributes: std::ffi::c_uint, Ty: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateObjectPointerType(Builder: LLVMDIBuilderRef, Type: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateQualifiedType(Builder: LLVMDIBuilderRef, Tag: std::ffi::c_uint, Type: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateReferenceType(Builder: LLVMDIBuilderRef, Tag: std::ffi::c_uint, Type: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateNullPtrType(Builder: LLVMDIBuilderRef) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateTypedef(Builder: LLVMDIBuilderRef, Type: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNo: std::ffi::c_uint, Scope: LLVMMetadataRef, AlignInBits: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateInheritance(Builder: LLVMDIBuilderRef, Ty: LLVMMetadataRef, BaseTy: LLVMMetadataRef, BaseOffset: std::ffi::c_ulong, VBPtrOffset: std::ffi::c_uint, Flags: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateForwardDecl(Builder: LLVMDIBuilderRef, Tag: std::ffi::c_uint, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, Scope: LLVMMetadataRef, File: LLVMMetadataRef, Line: std::ffi::c_uint, RuntimeLang: std::ffi::c_uint, SizeInBits: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, UniqueIdentifier: *const std::ffi::c_char, UniqueIdentifierLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateReplaceableCompositeType(Builder: LLVMDIBuilderRef, Tag: std::ffi::c_uint, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, Scope: LLVMMetadataRef, File: LLVMMetadataRef, Line: std::ffi::c_uint, RuntimeLang: std::ffi::c_uint, SizeInBits: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, Flags: std::ffi::c_uint, UniqueIdentifier: *const std::ffi::c_char, UniqueIdentifierLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateBitFieldMemberType(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNumber: std::ffi::c_uint, SizeInBits: std::ffi::c_ulong, OffsetInBits: std::ffi::c_ulong, StorageOffsetInBits: std::ffi::c_ulong, Flags: std::ffi::c_uint, Type: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateClassType(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNumber: std::ffi::c_uint, SizeInBits: std::ffi::c_ulong, AlignInBits: std::ffi::c_uint, OffsetInBits: std::ffi::c_ulong, Flags: std::ffi::c_uint, DerivedFrom: LLVMMetadataRef, Elements: *mut LLVMMetadataRef, NumElements: std::ffi::c_uint, VTableHolder: LLVMMetadataRef, TemplateParamsNode: LLVMMetadataRef, UniqueIdentifier: *const std::ffi::c_char, UniqueIdentifierLen: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateArtificialType(Builder: LLVMDIBuilderRef, Type: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDITypeGetName(DType: LLVMMetadataRef, Length: *mut std::ffi::c_ulong) -> *const std::ffi::c_char;

  pub fn LLVMDITypeGetSizeInBits(DType: LLVMMetadataRef) -> std::ffi::c_ulong;

  pub fn LLVMDITypeGetOffsetInBits(DType: LLVMMetadataRef) -> std::ffi::c_ulong;

  pub fn LLVMDITypeGetAlignInBits(DType: LLVMMetadataRef) -> std::ffi::c_uint;

  pub fn LLVMDITypeGetLine(DType: LLVMMetadataRef) -> std::ffi::c_uint;

  pub fn LLVMDITypeGetFlags(DType: LLVMMetadataRef) -> std::ffi::c_uint;

  pub fn LLVMDIBuilderGetOrCreateSubrange(Builder: LLVMDIBuilderRef, LowerBound: std::ffi::c_long, Count: std::ffi::c_long) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderGetOrCreateArray(Builder: LLVMDIBuilderRef, Data: *mut LLVMMetadataRef, NumElements: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateExpression(Builder: LLVMDIBuilderRef, Addr: *mut std::ffi::c_ulong, Length: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateConstantValueExpression(Builder: LLVMDIBuilderRef, Value: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateGlobalVariableExpression(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, Linkage: *const std::ffi::c_char, LinkLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNo: std::ffi::c_uint, Ty: LLVMMetadataRef, LocalToUnit: std::ffi::c_int, Expr: LLVMMetadataRef, Decl: LLVMMetadataRef, AlignInBits: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMGetDINodeTag(MD: LLVMMetadataRef) -> std::ffi::c_ushort;

  pub fn LLVMDIGlobalVariableExpressionGetVariable(GVE: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIGlobalVariableExpressionGetExpression(GVE: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIVariableGetFile(Var: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIVariableGetScope(Var: LLVMMetadataRef) -> LLVMMetadataRef;

  pub fn LLVMDIVariableGetLine(Var: LLVMMetadataRef) -> std::ffi::c_uint;

  pub fn LLVMTemporaryMDNode(Ctx: LLVMContextRef, Data: *mut LLVMMetadataRef, NumElements: std::ffi::c_ulong) -> LLVMMetadataRef;

  pub fn LLVMDisposeTemporaryMDNode(TempNode: LLVMMetadataRef) -> ();

  pub fn LLVMMetadataReplaceAllUsesWith(TempTargetMetadata: LLVMMetadataRef, Replacement: LLVMMetadataRef) -> ();

  pub fn LLVMDIBuilderCreateTempGlobalVariableFwdDecl(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, Linkage: *const std::ffi::c_char, LnkLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNo: std::ffi::c_uint, Ty: LLVMMetadataRef, LocalToUnit: std::ffi::c_int, Decl: LLVMMetadataRef, AlignInBits: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderInsertDeclareRecordBefore(Builder: LLVMDIBuilderRef, Storage: LLVMValueRef, VarInfo: LLVMMetadataRef, Expr: LLVMMetadataRef, DebugLoc: LLVMMetadataRef, Instr: LLVMValueRef) -> LLVMDbgRecordRef;

  pub fn LLVMDIBuilderInsertDeclareRecordAtEnd(Builder: LLVMDIBuilderRef, Storage: LLVMValueRef, VarInfo: LLVMMetadataRef, Expr: LLVMMetadataRef, DebugLoc: LLVMMetadataRef, Block: LLVMBasicBlockRef) -> LLVMDbgRecordRef;

  pub fn LLVMDIBuilderInsertDbgValueRecordBefore(Builder: LLVMDIBuilderRef, Val: LLVMValueRef, VarInfo: LLVMMetadataRef, Expr: LLVMMetadataRef, DebugLoc: LLVMMetadataRef, Instr: LLVMValueRef) -> LLVMDbgRecordRef;

  pub fn LLVMDIBuilderInsertDbgValueRecordAtEnd(Builder: LLVMDIBuilderRef, Val: LLVMValueRef, VarInfo: LLVMMetadataRef, Expr: LLVMMetadataRef, DebugLoc: LLVMMetadataRef, Block: LLVMBasicBlockRef) -> LLVMDbgRecordRef;

  pub fn LLVMDIBuilderCreateAutoVariable(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNo: std::ffi::c_uint, Ty: LLVMMetadataRef, AlwaysPreserve: std::ffi::c_int, Flags: std::ffi::c_uint, AlignInBits: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderCreateParameterVariable(Builder: LLVMDIBuilderRef, Scope: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, ArgNo: std::ffi::c_uint, File: LLVMMetadataRef, LineNo: std::ffi::c_uint, Ty: LLVMMetadataRef, AlwaysPreserve: std::ffi::c_int, Flags: std::ffi::c_uint) -> LLVMMetadataRef;

  pub fn LLVMGetSubprogram(Func: LLVMValueRef) -> LLVMMetadataRef;

  pub fn LLVMSetSubprogram(Func: LLVMValueRef, SP: LLVMMetadataRef) -> ();

  pub fn LLVMDISubprogramGetLine(Subprogram: LLVMMetadataRef) -> std::ffi::c_uint;

  pub fn LLVMInstructionGetDebugLoc(Inst: LLVMValueRef) -> LLVMMetadataRef;

  pub fn LLVMInstructionSetDebugLoc(Inst: LLVMValueRef, Loc: LLVMMetadataRef) -> ();

  pub fn LLVMDIBuilderCreateLabel(Builder: LLVMDIBuilderRef, Context: LLVMMetadataRef, Name: *const std::ffi::c_char, NameLen: std::ffi::c_ulong, File: LLVMMetadataRef, LineNo: std::ffi::c_uint, AlwaysPreserve: std::ffi::c_int) -> LLVMMetadataRef;

  pub fn LLVMDIBuilderInsertLabelBefore(Builder: LLVMDIBuilderRef, LabelInfo: LLVMMetadataRef, Location: LLVMMetadataRef, InsertBefore: LLVMValueRef) -> LLVMDbgRecordRef;

  pub fn LLVMDIBuilderInsertLabelAtEnd(Builder: LLVMDIBuilderRef, LabelInfo: LLVMMetadataRef, Location: LLVMMetadataRef, InsertAtEnd: LLVMBasicBlockRef) -> LLVMDbgRecordRef;

  pub fn LLVMGetMetadataKind(Metadata: LLVMMetadataRef) -> std::ffi::c_uint;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMDebugMetadataVersion()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDebugMetadataVersion()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetModuleDebugMetadataVersion<T0_>(Module_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMGetModuleDebugMetadataVersion(Into::<LLVMModuleRef>::into(Module_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn LLVMStripModuleDebugInfo<T0_>(Module_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMStripModuleDebugInfo(Into::<LLVMModuleRef>::into(Module_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDIBuilderRef> {
  pub unsafe fn LLVMCreateDIBuilderDisallowUnresolved<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMCreateDIBuilderDisallowUnresolved(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDIBuilderRef> {
  pub unsafe fn LLVMCreateDIBuilder<T0_>(M_:  T0_)-> Tret_
  where
     T0_: Into<LLVMModuleRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMCreateDIBuilder(Into::<LLVMModuleRef>::into(M_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeDIBuilder<T0_>(Builder_:  T0_)
  where
     T0_: Into<LLVMDIBuilderRef>
  {
    unsafe {
      crate::DebugInfo::LLVMDisposeDIBuilder(Into::<LLVMDIBuilderRef>::into(Builder_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDIBuilderFinalize<T0_>(Builder_:  T0_)
  where
     T0_: Into<LLVMDIBuilderRef>
  {
    unsafe {
      crate::DebugInfo::LLVMDIBuilderFinalize(Into::<LLVMDIBuilderRef>::into(Builder_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDIBuilderFinalizeSubprogram<T0_, T1_>(Builder_:  T0_, Subprogram_:  T1_)
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>
  {
    unsafe {
      crate::DebugInfo::LLVMDIBuilderFinalizeSubprogram(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Subprogram_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateCompileUnit<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_, T12_, T13_, T14_, T15_, T16_, T17_, T18_>(Builder_:  T0_, Lang_:  T1_, FileRef_:  T2_, Producer_:  T3_, ProducerLen_:  T4_, isOptimized_:  T5_, Flags_:  T6_, FlagsLen_:  T7_, RuntimeVer_:  T8_, SplitName_:  T9_, SplitNameLen_:  T10_, Kind_:  T11_, DWOId_:  T12_, SplitDebugInlining_:  T13_, DebugInfoForProfiling_:  T14_, SysRoot_:  T15_, SysRootLen_:  T16_, SDK_:  T17_, SDKLen_:  T18_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<*const std::ffi::c_char>,  T4_: Into<std::ffi::c_ulong>,  T5_: Into<std::ffi::c_int>,  T6_: Into<*const std::ffi::c_char>,  T7_: Into<std::ffi::c_ulong>,  T8_: Into<std::ffi::c_uint>,  T9_: Into<*const std::ffi::c_char>,  T10_: Into<std::ffi::c_ulong>,  T11_: Into<std::ffi::c_uint>,  T12_: Into<std::ffi::c_uint>,  T13_: Into<std::ffi::c_int>,  T14_: Into<std::ffi::c_int>,  T15_: Into<*const std::ffi::c_char>,  T16_: Into<std::ffi::c_ulong>,  T17_: Into<*const std::ffi::c_char>,  T18_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateCompileUnit(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<std::ffi::c_uint>::into(Lang_), Into::<LLVMMetadataRef>::into(FileRef_), Into::<*const std::ffi::c_char>::into(Producer_), Into::<std::ffi::c_ulong>::into(ProducerLen_), Into::<std::ffi::c_int>::into(isOptimized_), Into::<*const std::ffi::c_char>::into(Flags_), Into::<std::ffi::c_ulong>::into(FlagsLen_), Into::<std::ffi::c_uint>::into(RuntimeVer_), Into::<*const std::ffi::c_char>::into(SplitName_), Into::<std::ffi::c_ulong>::into(SplitNameLen_), Into::<std::ffi::c_uint>::into(Kind_), Into::<std::ffi::c_uint>::into(DWOId_), Into::<std::ffi::c_int>::into(SplitDebugInlining_), Into::<std::ffi::c_int>::into(DebugInfoForProfiling_), Into::<*const std::ffi::c_char>::into(SysRoot_), Into::<std::ffi::c_ulong>::into(SysRootLen_), Into::<*const std::ffi::c_char>::into(SDK_), Into::<std::ffi::c_ulong>::into(SDKLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateFile<T0_, T1_, T2_, T3_, T4_>(Builder_:  T0_, Filename_:  T1_, FilenameLen_:  T2_, Directory_:  T3_, DirectoryLen_:  T4_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<*const std::ffi::c_char>,  T4_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateFile(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<*const std::ffi::c_char>::into(Filename_), Into::<std::ffi::c_ulong>::into(FilenameLen_), Into::<*const std::ffi::c_char>::into(Directory_), Into::<std::ffi::c_ulong>::into(DirectoryLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateModule<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_>(Builder_:  T0_, ParentScope_:  T1_, Name_:  T2_, NameLen_:  T3_, ConfigMacros_:  T4_, ConfigMacrosLen_:  T5_, IncludePath_:  T6_, IncludePathLen_:  T7_, APINotesFile_:  T8_, APINotesFileLen_:  T9_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<*const std::ffi::c_char>,  T5_: Into<std::ffi::c_ulong>,  T6_: Into<*const std::ffi::c_char>,  T7_: Into<std::ffi::c_ulong>,  T8_: Into<*const std::ffi::c_char>,  T9_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateModule(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(ParentScope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<*const std::ffi::c_char>::into(ConfigMacros_), Into::<std::ffi::c_ulong>::into(ConfigMacrosLen_), Into::<*const std::ffi::c_char>::into(IncludePath_), Into::<std::ffi::c_ulong>::into(IncludePathLen_), Into::<*const std::ffi::c_char>::into(APINotesFile_), Into::<std::ffi::c_ulong>::into(APINotesFileLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateNameSpace<T0_, T1_, T2_, T3_, T4_>(Builder_:  T0_, ParentScope_:  T1_, Name_:  T2_, NameLen_:  T3_, ExportSymbols_:  T4_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateNameSpace(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(ParentScope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<std::ffi::c_int>::into(ExportSymbols_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateFunction<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_, T12_, T13_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, LinkageName_:  T4_, LinkageNameLen_:  T5_, File_:  T6_, LineNo_:  T7_, Ty_:  T8_, IsLocalToUnit_:  T9_, IsDefinition_:  T10_, ScopeLine_:  T11_, Flags_:  T12_, IsOptimized_:  T13_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<*const std::ffi::c_char>,  T5_: Into<std::ffi::c_ulong>,  T6_: Into<LLVMMetadataRef>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<LLVMMetadataRef>,  T9_: Into<std::ffi::c_int>,  T10_: Into<std::ffi::c_int>,  T11_: Into<std::ffi::c_uint>,  T12_: Into<std::ffi::c_uint>,  T13_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateFunction(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<*const std::ffi::c_char>::into(LinkageName_), Into::<std::ffi::c_ulong>::into(LinkageNameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNo_), Into::<LLVMMetadataRef>::into(Ty_), Into::<std::ffi::c_int>::into(IsLocalToUnit_), Into::<std::ffi::c_int>::into(IsDefinition_), Into::<std::ffi::c_uint>::into(ScopeLine_), Into::<std::ffi::c_uint>::into(Flags_), Into::<std::ffi::c_int>::into(IsOptimized_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateLexicalBlock<T0_, T1_, T2_, T3_, T4_>(Builder_:  T0_, Scope_:  T1_, File_:  T2_, Line_:  T3_, Column_:  T4_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateLexicalBlock(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(Line_), Into::<std::ffi::c_uint>::into(Column_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateLexicalBlockFile<T0_, T1_, T2_, T3_>(Builder_:  T0_, Scope_:  T1_, File_:  T2_, Discriminator_:  T3_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateLexicalBlockFile(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(Discriminator_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateImportedModuleFromNamespace<T0_, T1_, T2_, T3_, T4_>(Builder_:  T0_, Scope_:  T1_, NS_:  T2_, File_:  T3_, Line_:  T4_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateImportedModuleFromNamespace(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<LLVMMetadataRef>::into(NS_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(Line_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateImportedModuleFromAlias<T0_, T1_, T2_, T3_, T4_, T5_, T6_>(Builder_:  T0_, Scope_:  T1_, ImportedEntity_:  T2_, File_:  T3_, Line_:  T4_, Elements_:  T5_, NumElements_:  T6_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<*mut LLVMMetadataRef>,  T6_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateImportedModuleFromAlias(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<LLVMMetadataRef>::into(ImportedEntity_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(Line_), Into::<*mut LLVMMetadataRef>::into(Elements_), Into::<std::ffi::c_uint>::into(NumElements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateImportedModuleFromModule<T0_, T1_, T2_, T3_, T4_, T5_, T6_>(Builder_:  T0_, Scope_:  T1_, M_:  T2_, File_:  T3_, Line_:  T4_, Elements_:  T5_, NumElements_:  T6_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<*mut LLVMMetadataRef>,  T6_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateImportedModuleFromModule(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<LLVMMetadataRef>::into(M_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(Line_), Into::<*mut LLVMMetadataRef>::into(Elements_), Into::<std::ffi::c_uint>::into(NumElements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateImportedDeclaration<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_>(Builder_:  T0_, Scope_:  T1_, Decl_:  T2_, File_:  T3_, Line_:  T4_, Name_:  T5_, NameLen_:  T6_, Elements_:  T7_, NumElements_:  T8_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<*const std::ffi::c_char>,  T6_: Into<std::ffi::c_ulong>,  T7_: Into<*mut LLVMMetadataRef>,  T8_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateImportedDeclaration(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<LLVMMetadataRef>::into(Decl_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(Line_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<*mut LLVMMetadataRef>::into(Elements_), Into::<std::ffi::c_uint>::into(NumElements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateDebugLocation<T0_, T1_, T2_, T3_, T4_>(Ctx_:  T0_, Line_:  T1_, Column_:  T2_, Scope_:  T3_, InlinedAt_:  T4_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateDebugLocation(Into::<LLVMContextRef>::into(Ctx_), Into::<std::ffi::c_uint>::into(Line_), Into::<std::ffi::c_uint>::into(Column_), Into::<LLVMMetadataRef>::into(Scope_), Into::<LLVMMetadataRef>::into(InlinedAt_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMDILocationGetLine<T0_>(Location_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDILocationGetLine(Into::<LLVMMetadataRef>::into(Location_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMDILocationGetColumn<T0_>(Location_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDILocationGetColumn(Into::<LLVMMetadataRef>::into(Location_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDILocationGetScope<T0_>(Location_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDILocationGetScope(Into::<LLVMMetadataRef>::into(Location_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDILocationGetInlinedAt<T0_>(Location_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDILocationGetInlinedAt(Into::<LLVMMetadataRef>::into(Location_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIScopeGetFile<T0_>(Scope_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIScopeGetFile(Into::<LLVMMetadataRef>::into(Scope_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMDIFileGetDirectory<T0_, T1_>(File_:  T0_, Len_:  T1_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>,  T1_: Into<*mut std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIFileGetDirectory(Into::<LLVMMetadataRef>::into(File_), Into::<*mut std::ffi::c_uint>::into(Len_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMDIFileGetFilename<T0_, T1_>(File_:  T0_, Len_:  T1_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>,  T1_: Into<*mut std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIFileGetFilename(Into::<LLVMMetadataRef>::into(File_), Into::<*mut std::ffi::c_uint>::into(Len_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMDIFileGetSource<T0_, T1_>(File_:  T0_, Len_:  T1_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>,  T1_: Into<*mut std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIFileGetSource(Into::<LLVMMetadataRef>::into(File_), Into::<*mut std::ffi::c_uint>::into(Len_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderGetOrCreateTypeArray<T0_, T1_, T2_>(Builder_:  T0_, Data_:  T1_, NumElements_:  T2_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<*mut LLVMMetadataRef>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderGetOrCreateTypeArray(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<*mut LLVMMetadataRef>::into(Data_), Into::<std::ffi::c_ulong>::into(NumElements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateSubroutineType<T0_, T1_, T2_, T3_, T4_>(Builder_:  T0_, File_:  T1_, ParameterTypes_:  T2_, NumParameterTypes_:  T3_, Flags_:  T4_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*mut LLVMMetadataRef>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateSubroutineType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(File_), Into::<*mut LLVMMetadataRef>::into(ParameterTypes_), Into::<std::ffi::c_uint>::into(NumParameterTypes_), Into::<std::ffi::c_uint>::into(Flags_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateMacro<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_>(Builder_:  T0_, ParentMacroFile_:  T1_, Line_:  T2_, RecordType_:  T3_, Name_:  T4_, NameLen_:  T5_, Value_:  T6_, ValueLen_:  T7_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<*const std::ffi::c_char>,  T5_: Into<std::ffi::c_ulong>,  T6_: Into<*const std::ffi::c_char>,  T7_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateMacro(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(ParentMacroFile_), Into::<std::ffi::c_uint>::into(Line_), Into::<std::ffi::c_uint>::into(RecordType_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<*const std::ffi::c_char>::into(Value_), Into::<std::ffi::c_ulong>::into(ValueLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateTempMacroFile<T0_, T1_, T2_, T3_>(Builder_:  T0_, ParentMacroFile_:  T1_, Line_:  T2_, File_:  T3_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateTempMacroFile(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(ParentMacroFile_), Into::<std::ffi::c_uint>::into(Line_), Into::<LLVMMetadataRef>::into(File_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateEnumerator<T0_, T1_, T2_, T3_, T4_>(Builder_:  T0_, Name_:  T1_, NameLen_:  T2_, Value_:  T3_, IsUnsigned_:  T4_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<std::ffi::c_long>,  T4_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateEnumerator(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<std::ffi::c_long>::into(Value_), Into::<std::ffi::c_int>::into(IsUnsigned_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateEnumerationType<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, File_:  T4_, LineNumber_:  T5_, SizeInBits_:  T6_, AlignInBits_:  T7_, Elements_:  T8_, NumElements_:  T9_, ClassTy_:  T10_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_ulong>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<*mut LLVMMetadataRef>,  T9_: Into<std::ffi::c_uint>,  T10_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateEnumerationType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNumber_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<*mut LLVMMetadataRef>::into(Elements_), Into::<std::ffi::c_uint>::into(NumElements_), Into::<LLVMMetadataRef>::into(ClassTy_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateUnionType<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_, T12_, T13_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, File_:  T4_, LineNumber_:  T5_, SizeInBits_:  T6_, AlignInBits_:  T7_, Flags_:  T8_, Elements_:  T9_, NumElements_:  T10_, RunTimeLang_:  T11_, UniqueId_:  T12_, UniqueIdLen_:  T13_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_ulong>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<std::ffi::c_uint>,  T9_: Into<*mut LLVMMetadataRef>,  T10_: Into<std::ffi::c_uint>,  T11_: Into<std::ffi::c_uint>,  T12_: Into<*const std::ffi::c_char>,  T13_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateUnionType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNumber_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<std::ffi::c_uint>::into(Flags_), Into::<*mut LLVMMetadataRef>::into(Elements_), Into::<std::ffi::c_uint>::into(NumElements_), Into::<std::ffi::c_uint>::into(RunTimeLang_), Into::<*const std::ffi::c_char>::into(UniqueId_), Into::<std::ffi::c_ulong>::into(UniqueIdLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateArrayType<T0_, T1_, T2_, T3_, T4_, T5_>(Builder_:  T0_, Size_:  T1_, AlignInBits_:  T2_, Ty_:  T3_, Subscripts_:  T4_, NumSubscripts_:  T5_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<*mut LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateArrayType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<std::ffi::c_ulong>::into(Size_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<LLVMMetadataRef>::into(Ty_), Into::<*mut LLVMMetadataRef>::into(Subscripts_), Into::<std::ffi::c_uint>::into(NumSubscripts_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateVectorType<T0_, T1_, T2_, T3_, T4_, T5_>(Builder_:  T0_, Size_:  T1_, AlignInBits_:  T2_, Ty_:  T3_, Subscripts_:  T4_, NumSubscripts_:  T5_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<*mut LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateVectorType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<std::ffi::c_ulong>::into(Size_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<LLVMMetadataRef>::into(Ty_), Into::<*mut LLVMMetadataRef>::into(Subscripts_), Into::<std::ffi::c_uint>::into(NumSubscripts_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateUnspecifiedType<T0_, T1_, T2_>(Builder_:  T0_, Name_:  T1_, NameLen_:  T2_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateUnspecifiedType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateBasicType<T0_, T1_, T2_, T3_, T4_, T5_>(Builder_:  T0_, Name_:  T1_, NameLen_:  T2_, SizeInBits_:  T3_, Encoding_:  T4_, Flags_:  T5_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateBasicType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(Encoding_), Into::<std::ffi::c_uint>::into(Flags_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreatePointerType<T0_, T1_, T2_, T3_, T4_, T5_, T6_>(Builder_:  T0_, PointeeTy_:  T1_, SizeInBits_:  T2_, AlignInBits_:  T3_, AddressSpace_:  T4_, Name_:  T5_, NameLen_:  T6_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<*const std::ffi::c_char>,  T6_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreatePointerType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(PointeeTy_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<std::ffi::c_uint>::into(AddressSpace_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateStructType<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_, T12_, T13_, T14_, T15_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, File_:  T4_, LineNumber_:  T5_, SizeInBits_:  T6_, AlignInBits_:  T7_, Flags_:  T8_, DerivedFrom_:  T9_, Elements_:  T10_, NumElements_:  T11_, RunTimeLang_:  T12_, VTableHolder_:  T13_, UniqueId_:  T14_, UniqueIdLen_:  T15_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_ulong>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<std::ffi::c_uint>,  T9_: Into<LLVMMetadataRef>,  T10_: Into<*mut LLVMMetadataRef>,  T11_: Into<std::ffi::c_uint>,  T12_: Into<std::ffi::c_uint>,  T13_: Into<LLVMMetadataRef>,  T14_: Into<*const std::ffi::c_char>,  T15_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateStructType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNumber_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<std::ffi::c_uint>::into(Flags_), Into::<LLVMMetadataRef>::into(DerivedFrom_), Into::<*mut LLVMMetadataRef>::into(Elements_), Into::<std::ffi::c_uint>::into(NumElements_), Into::<std::ffi::c_uint>::into(RunTimeLang_), Into::<LLVMMetadataRef>::into(VTableHolder_), Into::<*const std::ffi::c_char>::into(UniqueId_), Into::<std::ffi::c_ulong>::into(UniqueIdLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateMemberType<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, File_:  T4_, LineNo_:  T5_, SizeInBits_:  T6_, AlignInBits_:  T7_, OffsetInBits_:  T8_, Flags_:  T9_, Ty_:  T10_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_ulong>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<std::ffi::c_ulong>,  T9_: Into<std::ffi::c_uint>,  T10_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateMemberType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNo_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<std::ffi::c_ulong>::into(OffsetInBits_), Into::<std::ffi::c_uint>::into(Flags_), Into::<LLVMMetadataRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateStaticMemberType<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, File_:  T4_, LineNumber_:  T5_, Type_:  T6_, Flags_:  T7_, ConstantVal_:  T8_, AlignInBits_:  T9_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<LLVMMetadataRef>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<LLVMValueRef>,  T9_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateStaticMemberType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNumber_), Into::<LLVMMetadataRef>::into(Type_), Into::<std::ffi::c_uint>::into(Flags_), Into::<LLVMValueRef>::into(ConstantVal_), Into::<std::ffi::c_uint>::into(AlignInBits_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateMemberPointerType<T0_, T1_, T2_, T3_, T4_, T5_>(Builder_:  T0_, PointeeType_:  T1_, ClassType_:  T2_, SizeInBits_:  T3_, AlignInBits_:  T4_, Flags_:  T5_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateMemberPointerType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(PointeeType_), Into::<LLVMMetadataRef>::into(ClassType_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<std::ffi::c_uint>::into(Flags_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateObjCIVar<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_>(Builder_:  T0_, Name_:  T1_, NameLen_:  T2_, File_:  T3_, LineNo_:  T4_, SizeInBits_:  T5_, AlignInBits_:  T6_, OffsetInBits_:  T7_, Flags_:  T8_, Ty_:  T9_, PropertyNode_:  T10_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<std::ffi::c_ulong>,  T6_: Into<std::ffi::c_uint>,  T7_: Into<std::ffi::c_ulong>,  T8_: Into<std::ffi::c_uint>,  T9_: Into<LLVMMetadataRef>,  T10_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateObjCIVar(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNo_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<std::ffi::c_ulong>::into(OffsetInBits_), Into::<std::ffi::c_uint>::into(Flags_), Into::<LLVMMetadataRef>::into(Ty_), Into::<LLVMMetadataRef>::into(PropertyNode_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateObjCProperty<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_>(Builder_:  T0_, Name_:  T1_, NameLen_:  T2_, File_:  T3_, LineNo_:  T4_, GetterName_:  T5_, GetterNameLen_:  T6_, SetterName_:  T7_, SetterNameLen_:  T8_, PropertyAttributes_:  T9_, Ty_:  T10_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<*const std::ffi::c_char>,  T2_: Into<std::ffi::c_ulong>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<*const std::ffi::c_char>,  T6_: Into<std::ffi::c_ulong>,  T7_: Into<*const std::ffi::c_char>,  T8_: Into<std::ffi::c_ulong>,  T9_: Into<std::ffi::c_uint>,  T10_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateObjCProperty(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNo_), Into::<*const std::ffi::c_char>::into(GetterName_), Into::<std::ffi::c_ulong>::into(GetterNameLen_), Into::<*const std::ffi::c_char>::into(SetterName_), Into::<std::ffi::c_ulong>::into(SetterNameLen_), Into::<std::ffi::c_uint>::into(PropertyAttributes_), Into::<LLVMMetadataRef>::into(Ty_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateObjectPointerType<T0_, T1_>(Builder_:  T0_, Type_:  T1_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateObjectPointerType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateQualifiedType<T0_, T1_, T2_>(Builder_:  T0_, Tag_:  T1_, Type_:  T2_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateQualifiedType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<std::ffi::c_uint>::into(Tag_), Into::<LLVMMetadataRef>::into(Type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateReferenceType<T0_, T1_, T2_>(Builder_:  T0_, Tag_:  T1_, Type_:  T2_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateReferenceType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<std::ffi::c_uint>::into(Tag_), Into::<LLVMMetadataRef>::into(Type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateNullPtrType<T0_>(Builder_:  T0_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateNullPtrType(Into::<LLVMDIBuilderRef>::into(Builder_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateTypedef<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_>(Builder_:  T0_, Type_:  T1_, Name_:  T2_, NameLen_:  T3_, File_:  T4_, LineNo_:  T5_, Scope_:  T6_, AlignInBits_:  T7_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<LLVMMetadataRef>,  T7_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateTypedef(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Type_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNo_), Into::<LLVMMetadataRef>::into(Scope_), Into::<std::ffi::c_uint>::into(AlignInBits_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateInheritance<T0_, T1_, T2_, T3_, T4_, T5_>(Builder_:  T0_, Ty_:  T1_, BaseTy_:  T2_, BaseOffset_:  T3_, VBPtrOffset_:  T4_, Flags_:  T5_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateInheritance(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Ty_), Into::<LLVMMetadataRef>::into(BaseTy_), Into::<std::ffi::c_ulong>::into(BaseOffset_), Into::<std::ffi::c_uint>::into(VBPtrOffset_), Into::<std::ffi::c_uint>::into(Flags_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateForwardDecl<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_>(Builder_:  T0_, Tag_:  T1_, Name_:  T2_, NameLen_:  T3_, Scope_:  T4_, File_:  T5_, Line_:  T6_, RuntimeLang_:  T7_, SizeInBits_:  T8_, AlignInBits_:  T9_, UniqueIdentifier_:  T10_, UniqueIdentifierLen_:  T11_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<LLVMMetadataRef>,  T6_: Into<std::ffi::c_uint>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<std::ffi::c_ulong>,  T9_: Into<std::ffi::c_uint>,  T10_: Into<*const std::ffi::c_char>,  T11_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateForwardDecl(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<std::ffi::c_uint>::into(Tag_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(Scope_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(Line_), Into::<std::ffi::c_uint>::into(RuntimeLang_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<*const std::ffi::c_char>::into(UniqueIdentifier_), Into::<std::ffi::c_ulong>::into(UniqueIdentifierLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateReplaceableCompositeType<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_, T12_>(Builder_:  T0_, Tag_:  T1_, Name_:  T2_, NameLen_:  T3_, Scope_:  T4_, File_:  T5_, Line_:  T6_, RuntimeLang_:  T7_, SizeInBits_:  T8_, AlignInBits_:  T9_, Flags_:  T10_, UniqueIdentifier_:  T11_, UniqueIdentifierLen_:  T12_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<LLVMMetadataRef>,  T6_: Into<std::ffi::c_uint>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<std::ffi::c_ulong>,  T9_: Into<std::ffi::c_uint>,  T10_: Into<std::ffi::c_uint>,  T11_: Into<*const std::ffi::c_char>,  T12_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateReplaceableCompositeType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<std::ffi::c_uint>::into(Tag_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(Scope_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(Line_), Into::<std::ffi::c_uint>::into(RuntimeLang_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<std::ffi::c_uint>::into(Flags_), Into::<*const std::ffi::c_char>::into(UniqueIdentifier_), Into::<std::ffi::c_ulong>::into(UniqueIdentifierLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateBitFieldMemberType<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, File_:  T4_, LineNumber_:  T5_, SizeInBits_:  T6_, OffsetInBits_:  T7_, StorageOffsetInBits_:  T8_, Flags_:  T9_, Type_:  T10_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_ulong>,  T7_: Into<std::ffi::c_ulong>,  T8_: Into<std::ffi::c_ulong>,  T9_: Into<std::ffi::c_uint>,  T10_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateBitFieldMemberType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNumber_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_ulong>::into(OffsetInBits_), Into::<std::ffi::c_ulong>::into(StorageOffsetInBits_), Into::<std::ffi::c_uint>::into(Flags_), Into::<LLVMMetadataRef>::into(Type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateClassType<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_, T12_, T13_, T14_, T15_, T16_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, File_:  T4_, LineNumber_:  T5_, SizeInBits_:  T6_, AlignInBits_:  T7_, OffsetInBits_:  T8_, Flags_:  T9_, DerivedFrom_:  T10_, Elements_:  T11_, NumElements_:  T12_, VTableHolder_:  T13_, TemplateParamsNode_:  T14_, UniqueIdentifier_:  T15_, UniqueIdentifierLen_:  T16_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_ulong>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<std::ffi::c_ulong>,  T9_: Into<std::ffi::c_uint>,  T10_: Into<LLVMMetadataRef>,  T11_: Into<*mut LLVMMetadataRef>,  T12_: Into<std::ffi::c_uint>,  T13_: Into<LLVMMetadataRef>,  T14_: Into<LLVMMetadataRef>,  T15_: Into<*const std::ffi::c_char>,  T16_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateClassType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNumber_), Into::<std::ffi::c_ulong>::into(SizeInBits_), Into::<std::ffi::c_uint>::into(AlignInBits_), Into::<std::ffi::c_ulong>::into(OffsetInBits_), Into::<std::ffi::c_uint>::into(Flags_), Into::<LLVMMetadataRef>::into(DerivedFrom_), Into::<*mut LLVMMetadataRef>::into(Elements_), Into::<std::ffi::c_uint>::into(NumElements_), Into::<LLVMMetadataRef>::into(VTableHolder_), Into::<LLVMMetadataRef>::into(TemplateParamsNode_), Into::<*const std::ffi::c_char>::into(UniqueIdentifier_), Into::<std::ffi::c_ulong>::into(UniqueIdentifierLen_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateArtificialType<T0_, T1_>(Builder_:  T0_, Type_:  T1_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateArtificialType(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_char> {
  pub unsafe fn LLVMDITypeGetName<T0_, T1_>(DType_:  T0_, Length_:  T1_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>,  T1_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDITypeGetName(Into::<LLVMMetadataRef>::into(DType_), Into::<*mut std::ffi::c_ulong>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMDITypeGetSizeInBits<T0_>(DType_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDITypeGetSizeInBits(Into::<LLVMMetadataRef>::into(DType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn LLVMDITypeGetOffsetInBits<T0_>(DType_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDITypeGetOffsetInBits(Into::<LLVMMetadataRef>::into(DType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMDITypeGetAlignInBits<T0_>(DType_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDITypeGetAlignInBits(Into::<LLVMMetadataRef>::into(DType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMDITypeGetLine<T0_>(DType_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDITypeGetLine(Into::<LLVMMetadataRef>::into(DType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMDITypeGetFlags<T0_>(DType_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDITypeGetFlags(Into::<LLVMMetadataRef>::into(DType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderGetOrCreateSubrange<T0_, T1_, T2_>(Builder_:  T0_, LowerBound_:  T1_, Count_:  T2_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<std::ffi::c_long>,  T2_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderGetOrCreateSubrange(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<std::ffi::c_long>::into(LowerBound_), Into::<std::ffi::c_long>::into(Count_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderGetOrCreateArray<T0_, T1_, T2_>(Builder_:  T0_, Data_:  T1_, NumElements_:  T2_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<*mut LLVMMetadataRef>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderGetOrCreateArray(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<*mut LLVMMetadataRef>::into(Data_), Into::<std::ffi::c_ulong>::into(NumElements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateExpression<T0_, T1_, T2_>(Builder_:  T0_, Addr_:  T1_, Length_:  T2_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<*mut std::ffi::c_ulong>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateExpression(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<*mut std::ffi::c_ulong>::into(Addr_), Into::<std::ffi::c_ulong>::into(Length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateConstantValueExpression<T0_, T1_>(Builder_:  T0_, Value_:  T1_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateConstantValueExpression(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<std::ffi::c_ulong>::into(Value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateGlobalVariableExpression<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_, T12_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, Linkage_:  T4_, LinkLen_:  T5_, File_:  T6_, LineNo_:  T7_, Ty_:  T8_, LocalToUnit_:  T9_, Expr_:  T10_, Decl_:  T11_, AlignInBits_:  T12_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<*const std::ffi::c_char>,  T5_: Into<std::ffi::c_ulong>,  T6_: Into<LLVMMetadataRef>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<LLVMMetadataRef>,  T9_: Into<std::ffi::c_int>,  T10_: Into<LLVMMetadataRef>,  T11_: Into<LLVMMetadataRef>,  T12_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateGlobalVariableExpression(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<*const std::ffi::c_char>::into(Linkage_), Into::<std::ffi::c_ulong>::into(LinkLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNo_), Into::<LLVMMetadataRef>::into(Ty_), Into::<std::ffi::c_int>::into(LocalToUnit_), Into::<LLVMMetadataRef>::into(Expr_), Into::<LLVMMetadataRef>::into(Decl_), Into::<std::ffi::c_uint>::into(AlignInBits_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ushort> {
  pub unsafe fn LLVMGetDINodeTag<T0_>(MD_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMGetDINodeTag(Into::<LLVMMetadataRef>::into(MD_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIGlobalVariableExpressionGetVariable<T0_>(GVE_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIGlobalVariableExpressionGetVariable(Into::<LLVMMetadataRef>::into(GVE_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIGlobalVariableExpressionGetExpression<T0_>(GVE_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIGlobalVariableExpressionGetExpression(Into::<LLVMMetadataRef>::into(GVE_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIVariableGetFile<T0_>(Var_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIVariableGetFile(Into::<LLVMMetadataRef>::into(Var_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIVariableGetScope<T0_>(Var_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIVariableGetScope(Into::<LLVMMetadataRef>::into(Var_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMDIVariableGetLine<T0_>(Var_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIVariableGetLine(Into::<LLVMMetadataRef>::into(Var_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMTemporaryMDNode<T0_, T1_, T2_>(Ctx_:  T0_, Data_:  T1_, NumElements_:  T2_)-> Tret_
  where
     T0_: Into<LLVMContextRef>,  T1_: Into<*mut LLVMMetadataRef>,  T2_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMTemporaryMDNode(Into::<LLVMContextRef>::into(Ctx_), Into::<*mut LLVMMetadataRef>::into(Data_), Into::<std::ffi::c_ulong>::into(NumElements_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMDisposeTemporaryMDNode<T0_>(TempNode_:  T0_)
  where
     T0_: Into<LLVMMetadataRef>
  {
    unsafe {
      crate::DebugInfo::LLVMDisposeTemporaryMDNode(Into::<LLVMMetadataRef>::into(TempNode_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMMetadataReplaceAllUsesWith<T0_, T1_>(TempTargetMetadata_:  T0_, Replacement_:  T1_)
  where
     T0_: Into<LLVMMetadataRef>,  T1_: Into<LLVMMetadataRef>
  {
    unsafe {
      crate::DebugInfo::LLVMMetadataReplaceAllUsesWith(Into::<LLVMMetadataRef>::into(TempTargetMetadata_), Into::<LLVMMetadataRef>::into(Replacement_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateTempGlobalVariableFwdDecl<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, Linkage_:  T4_, LnkLen_:  T5_, File_:  T6_, LineNo_:  T7_, Ty_:  T8_, LocalToUnit_:  T9_, Decl_:  T10_, AlignInBits_:  T11_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<*const std::ffi::c_char>,  T5_: Into<std::ffi::c_ulong>,  T6_: Into<LLVMMetadataRef>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<LLVMMetadataRef>,  T9_: Into<std::ffi::c_int>,  T10_: Into<LLVMMetadataRef>,  T11_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateTempGlobalVariableFwdDecl(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<*const std::ffi::c_char>::into(Linkage_), Into::<std::ffi::c_ulong>::into(LnkLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNo_), Into::<LLVMMetadataRef>::into(Ty_), Into::<std::ffi::c_int>::into(LocalToUnit_), Into::<LLVMMetadataRef>::into(Decl_), Into::<std::ffi::c_uint>::into(AlignInBits_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDbgRecordRef> {
  pub unsafe fn LLVMDIBuilderInsertDeclareRecordBefore<T0_, T1_, T2_, T3_, T4_, T5_>(Builder_:  T0_, Storage_:  T1_, VarInfo_:  T2_, Expr_:  T3_, DebugLoc_:  T4_, Instr_:  T5_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderInsertDeclareRecordBefore(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(Storage_), Into::<LLVMMetadataRef>::into(VarInfo_), Into::<LLVMMetadataRef>::into(Expr_), Into::<LLVMMetadataRef>::into(DebugLoc_), Into::<LLVMValueRef>::into(Instr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDbgRecordRef> {
  pub unsafe fn LLVMDIBuilderInsertDeclareRecordAtEnd<T0_, T1_, T2_, T3_, T4_, T5_>(Builder_:  T0_, Storage_:  T1_, VarInfo_:  T2_, Expr_:  T3_, DebugLoc_:  T4_, Block_:  T5_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderInsertDeclareRecordAtEnd(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(Storage_), Into::<LLVMMetadataRef>::into(VarInfo_), Into::<LLVMMetadataRef>::into(Expr_), Into::<LLVMMetadataRef>::into(DebugLoc_), Into::<LLVMBasicBlockRef>::into(Block_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDbgRecordRef> {
  pub unsafe fn LLVMDIBuilderInsertDbgValueRecordBefore<T0_, T1_, T2_, T3_, T4_, T5_>(Builder_:  T0_, Val_:  T1_, VarInfo_:  T2_, Expr_:  T3_, DebugLoc_:  T4_, Instr_:  T5_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderInsertDbgValueRecordBefore(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMMetadataRef>::into(VarInfo_), Into::<LLVMMetadataRef>::into(Expr_), Into::<LLVMMetadataRef>::into(DebugLoc_), Into::<LLVMValueRef>::into(Instr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDbgRecordRef> {
  pub unsafe fn LLVMDIBuilderInsertDbgValueRecordAtEnd<T0_, T1_, T2_, T3_, T4_, T5_>(Builder_:  T0_, Val_:  T1_, VarInfo_:  T2_, Expr_:  T3_, DebugLoc_:  T4_, Block_:  T5_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMValueRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<LLVMMetadataRef>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderInsertDbgValueRecordAtEnd(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMValueRef>::into(Val_), Into::<LLVMMetadataRef>::into(VarInfo_), Into::<LLVMMetadataRef>::into(Expr_), Into::<LLVMMetadataRef>::into(DebugLoc_), Into::<LLVMBasicBlockRef>::into(Block_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateAutoVariable<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, File_:  T4_, LineNo_:  T5_, Ty_:  T6_, AlwaysPreserve_:  T7_, Flags_:  T8_, AlignInBits_:  T9_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<LLVMMetadataRef>,  T7_: Into<std::ffi::c_int>,  T8_: Into<std::ffi::c_uint>,  T9_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateAutoVariable(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNo_), Into::<LLVMMetadataRef>::into(Ty_), Into::<std::ffi::c_int>::into(AlwaysPreserve_), Into::<std::ffi::c_uint>::into(Flags_), Into::<std::ffi::c_uint>::into(AlignInBits_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateParameterVariable<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_>(Builder_:  T0_, Scope_:  T1_, Name_:  T2_, NameLen_:  T3_, ArgNo_:  T4_, File_:  T5_, LineNo_:  T6_, Ty_:  T7_, AlwaysPreserve_:  T8_, Flags_:  T9_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<LLVMMetadataRef>,  T6_: Into<std::ffi::c_uint>,  T7_: Into<LLVMMetadataRef>,  T8_: Into<std::ffi::c_int>,  T9_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateParameterVariable(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Scope_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<std::ffi::c_uint>::into(ArgNo_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNo_), Into::<LLVMMetadataRef>::into(Ty_), Into::<std::ffi::c_int>::into(AlwaysPreserve_), Into::<std::ffi::c_uint>::into(Flags_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMGetSubprogram<T0_>(Func_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMGetSubprogram(Into::<LLVMValueRef>::into(Func_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMSetSubprogram<T0_, T1_>(Func_:  T0_, SP_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMMetadataRef>
  {
    unsafe {
      crate::DebugInfo::LLVMSetSubprogram(Into::<LLVMValueRef>::into(Func_), Into::<LLVMMetadataRef>::into(SP_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMDISubprogramGetLine<T0_>(Subprogram_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDISubprogramGetLine(Into::<LLVMMetadataRef>::into(Subprogram_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMInstructionGetDebugLoc<T0_>(Inst_:  T0_)-> Tret_
  where
     T0_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMInstructionGetDebugLoc(Into::<LLVMValueRef>::into(Inst_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn LLVMInstructionSetDebugLoc<T0_, T1_>(Inst_:  T0_, Loc_:  T1_)
  where
     T0_: Into<LLVMValueRef>,  T1_: Into<LLVMMetadataRef>
  {
    unsafe {
      crate::DebugInfo::LLVMInstructionSetDebugLoc(Into::<LLVMValueRef>::into(Inst_), Into::<LLVMMetadataRef>::into(Loc_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMMetadataRef> {
  pub unsafe fn LLVMDIBuilderCreateLabel<T0_, T1_, T2_, T3_, T4_, T5_, T6_>(Builder_:  T0_, Context_:  T1_, Name_:  T2_, NameLen_:  T3_, File_:  T4_, LineNo_:  T5_, AlwaysPreserve_:  T6_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<*const std::ffi::c_char>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<LLVMMetadataRef>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderCreateLabel(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(Context_), Into::<*const std::ffi::c_char>::into(Name_), Into::<std::ffi::c_ulong>::into(NameLen_), Into::<LLVMMetadataRef>::into(File_), Into::<std::ffi::c_uint>::into(LineNo_), Into::<std::ffi::c_int>::into(AlwaysPreserve_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDbgRecordRef> {
  pub unsafe fn LLVMDIBuilderInsertLabelBefore<T0_, T1_, T2_, T3_>(Builder_:  T0_, LabelInfo_:  T1_, Location_:  T2_, InsertBefore_:  T3_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<LLVMValueRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderInsertLabelBefore(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(LabelInfo_), Into::<LLVMMetadataRef>::into(Location_), Into::<LLVMValueRef>::into(InsertBefore_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMDbgRecordRef> {
  pub unsafe fn LLVMDIBuilderInsertLabelAtEnd<T0_, T1_, T2_, T3_>(Builder_:  T0_, LabelInfo_:  T1_, Location_:  T2_, InsertAtEnd_:  T3_)-> Tret_
  where
     T0_: Into<LLVMDIBuilderRef>,  T1_: Into<LLVMMetadataRef>,  T2_: Into<LLVMMetadataRef>,  T3_: Into<LLVMBasicBlockRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMDIBuilderInsertLabelAtEnd(Into::<LLVMDIBuilderRef>::into(Builder_), Into::<LLVMMetadataRef>::into(LabelInfo_), Into::<LLVMMetadataRef>::into(Location_), Into::<LLVMBasicBlockRef>::into(InsertAtEnd_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn LLVMGetMetadataKind<T0_>(Metadata_:  T0_)-> Tret_
  where
     T0_: Into<LLVMMetadataRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::DebugInfo::LLVMGetMetadataKind(Into::<LLVMMetadataRef>::into(Metadata_))
      }
    )
  }
}

