pub mod error;
pub mod signature;
pub mod icon;
pub mod manifest;
pub mod security;
pub mod file_info;
pub mod permission;
pub mod apk_parser;

// 重新导出常用的类型和函数
pub use error::ApkParserError;
pub use signature::{SignatureInfo, extract_signature_info};
pub use icon::extract_icon;
pub use manifest::{extract_manifest_xml, extract_with_aapt2, ensure_aapt2_available, is_placeholder_aapt2};
pub use security::{PermissionAnalysis, SecurityConfig, SdkFeatures};
pub use file_info::FileInfo;
pub use permission::Permission;
pub use apk_parser::{ApkParser, ApkInfo, ApkParsingContext}; 