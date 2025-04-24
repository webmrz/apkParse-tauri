use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionAnalysis {
    pub total_permissions: u32,
    pub dangerous_permissions: u32,
    pub normal_permissions: u32,
    pub signature_permissions: u32,
    pub other_permissions: u32,
    pub high_risk_permissions: Vec<String>,
    pub risk_level: String,
    pub permission_groups: Option<HashMap<String, Vec<String>>>,
    pub system_permissions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityConfig {
    pub uses_clear_text_traffic: bool,
    pub debuggable: bool,
    pub backup_allowed: bool,
    pub allow_backup: bool,
    pub uses_permission_flags: bool,
    pub has_network_security_config: Option<bool>,
    pub prevents_screenshots: Option<bool>,
    pub uses_encryption: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SdkFeatures {
    pub third_party_sdks: HashMap<String, bool>,
    pub uses_encryption: bool,
    pub uses_root_detection: bool,
    pub uses_certificate_pinning: bool,
    pub is_obfuscated: bool,
    pub detected_sdks: Option<Vec<String>>,
    pub security_features: Option<Vec<String>>,
    pub potentially_obfuscated: Option<bool>,
}

impl fmt::Display for PermissionAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Total Permissions: {}\nDangerous Permissions: {}\nNormal Permissions: {}\nSignature Permissions: {}\nOther Permissions: {}\nRisk Level: {}",
            self.total_permissions,
            self.dangerous_permissions,
            self.normal_permissions,
            self.signature_permissions,
            self.other_permissions,
            self.risk_level
        )
    }
}

impl fmt::Display for SecurityConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Uses Clear Text Traffic: {}\nDebuggable: {}\nBackup Allowed: {}\nAllow Backup: {}\nUses Permission Flags: {}",
            self.uses_clear_text_traffic,
            self.debuggable,
            self.backup_allowed,
            self.allow_backup,
            self.uses_permission_flags
        )
    }
}

impl fmt::Display for SdkFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Uses Encryption: {}\nUses Root Detection: {}\nUses Certificate Pinning: {}\nIs Obfuscated: {}",
            self.uses_encryption,
            self.uses_root_detection,
            self.uses_certificate_pinning,
            self.is_obfuscated
        )
    }
}

impl Default for PermissionAnalysis {
    fn default() -> Self {
        Self {
            total_permissions: 0,
            dangerous_permissions: 0,
            normal_permissions: 0,
            signature_permissions: 0,
            other_permissions: 0,
            high_risk_permissions: Vec::new(),
            risk_level: "LOW".to_string(),
            permission_groups: None,
            system_permissions: None,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            uses_clear_text_traffic: false,
            debuggable: false,
            backup_allowed: true,
            allow_backup: true,
            uses_permission_flags: false,
            has_network_security_config: None,
            prevents_screenshots: None,
            uses_encryption: None,
        }
    }
}

impl Default for SdkFeatures {
    fn default() -> Self {
        Self {
            third_party_sdks: HashMap::new(),
            uses_encryption: false,
            uses_root_detection: false,
            uses_certificate_pinning: false,
            is_obfuscated: false,
            detected_sdks: None,
            security_features: None,
            potentially_obfuscated: None,
        }
    }
} 