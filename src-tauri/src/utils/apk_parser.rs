use std::path::Path;
use std::fs::File;
use std::io::{self, Read};
use std::process::Command;
use std::time::Instant;
use zip::ZipArchive;
use serde::{Serialize, Deserialize};
use regex::Regex;
use axmldecoder;
use x509_parser;
use sha1::{self, Sha1, Digest as Sha1Digest};
use sha2::{Sha256};
use x509_parser::prelude::*;
use reqwest;
use md5;
use digest;
use chrono;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use tauri;
use tauri::path::{BaseDirectory, PathResolver};

use crate::utils::error::ApkParserError;
use crate::utils::signature::{SignatureInfo, extract_signature_info};
use crate::utils::permission::Permission;
use crate::utils::file_info::FileInfo;
use crate::utils::security::{PermissionAnalysis, SecurityConfig, SdkFeatures};
use crate::utils::manifest::{extract_manifest_xml, extract_with_aapt2};
use crate::utils::icon::extract_icon;

/// APK解析器的主要结构体
#[derive(Debug)]
pub struct ApkParser {
    // 危险权限列表
    high_risk_permissions: Vec<String>,
    dangerous_permissions: Vec<String>,
    // ZIP存档
    archive: Option<ZipArchive<File>>,
}

/// APK解析上下文，用于存储解析过程中的状态
#[derive(Debug)]
pub struct ApkParsingContext {
    pub archive: ZipArchive<File>,
    pub file_size: u64,
    pub file_md5: String,
    pub file_sha1: String,
    pub file_sha256: String,
}

/// APK信息结构体，包含解析后的所有信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApkInfo {
    /// 包名
    pub package_name: String,
    /// 版本名称
    pub version_name: String,
    /// 版本号
    pub version_code: String,
    /// 最低SDK版本
    pub min_sdk: String,
    /// 目标SDK版本
    pub target_sdk: String,
    /// 签名信息
    pub signature_info: Option<SignatureInfo>,
    /// 权限列表
    pub permissions: Option<Vec<Permission>>,
    /// 文件信息
    pub file_info: Option<FileInfo>,
    /// 主Activity
    pub main_activity: Option<String>,
    /// 应用图标（Base64编码）
    pub icon_base64: Option<String>,
    /// 应用本地化名称
    pub app_labels: Option<std::collections::HashMap<String, String>>,
    /// 权限分析
    pub permission_analysis: Option<PermissionAnalysis>,
    /// 安全配置
    pub security_config: Option<SecurityConfig>,
    /// 目标SDK特性
    pub sdk_features: Option<SdkFeatures>,
}

impl ApkParser {
    /// 创建一个新的APK解析器实例
    pub fn new() -> Self {
        Self {
            high_risk_permissions: vec![
                "android.permission.READ_CALENDAR".to_string(),
                "android.permission.WRITE_CALENDAR".to_string(),
                "android.permission.CAMERA".to_string(),
                "android.permission.READ_CONTACTS".to_string(),
                "android.permission.WRITE_CONTACTS".to_string(),
                "android.permission.GET_ACCOUNTS".to_string(),
                "android.permission.ACCESS_FINE_LOCATION".to_string(),
                "android.permission.ACCESS_COARSE_LOCATION".to_string(),
                "android.permission.RECORD_AUDIO".to_string(),
                "android.permission.READ_PHONE_STATE".to_string(),
                "android.permission.READ_PHONE_NUMBERS".to_string(),
                "android.permission.CALL_PHONE".to_string(),
                "android.permission.READ_CALL_LOG".to_string(),
                "android.permission.WRITE_CALL_LOG".to_string(),
                "android.permission.ADD_VOICEMAIL".to_string(),
                "android.permission.USE_SIP".to_string(),
                "android.permission.PROCESS_OUTGOING_CALLS".to_string(),
                "android.permission.BODY_SENSORS".to_string(),
                "android.permission.SEND_SMS".to_string(),
                "android.permission.RECEIVE_SMS".to_string(),
                "android.permission.READ_SMS".to_string(),
                "android.permission.RECEIVE_WAP_PUSH".to_string(),
                "android.permission.RECEIVE_MMS".to_string(),
                "android.permission.READ_EXTERNAL_STORAGE".to_string(),
                "android.permission.WRITE_EXTERNAL_STORAGE".to_string(),
            ],
            dangerous_permissions: vec![
                "android.permission.READ_CALENDAR".to_string(),
                "android.permission.WRITE_CALENDAR".to_string(),
                "android.permission.CAMERA".to_string(),
                "android.permission.READ_CONTACTS".to_string(),
                "android.permission.WRITE_CONTACTS".to_string(),
                "android.permission.GET_ACCOUNTS".to_string(),
                "android.permission.ACCESS_FINE_LOCATION".to_string(),
                "android.permission.ACCESS_COARSE_LOCATION".to_string(),
                "android.permission.RECORD_AUDIO".to_string(),
                "android.permission.READ_PHONE_STATE".to_string(),
                "android.permission.READ_PHONE_NUMBERS".to_string(),
                "android.permission.CALL_PHONE".to_string(),
                "android.permission.READ_CALL_LOG".to_string(),
                "android.permission.WRITE_CALL_LOG".to_string(),
                "android.permission.ADD_VOICEMAIL".to_string(),
                "android.permission.USE_SIP".to_string(),
                "android.permission.PROCESS_OUTGOING_CALLS".to_string(),
                "android.permission.BODY_SENSORS".to_string(),
                "android.permission.SEND_SMS".to_string(),
                "android.permission.RECEIVE_SMS".to_string(),
                "android.permission.READ_SMS".to_string(),
                "android.permission.RECEIVE_WAP_PUSH".to_string(),
                "android.permission.RECEIVE_MMS".to_string(),
                "android.permission.READ_EXTERNAL_STORAGE".to_string(),
                "android.permission.WRITE_EXTERNAL_STORAGE".to_string(),
            ],
            archive: None,
        }
    }

    /// 创建APK解析上下文
    pub fn create_parsing_context<P: AsRef<Path>>(&self, apk_path: P) -> Result<ApkParsingContext, String> {
        let file = File::open(apk_path.as_ref()).map_err(|e| e.to_string())?;
        let file_size = file.metadata().map_err(|e| e.to_string())?.len();
        
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        
        // 计算文件哈希
        let file_md5 = self.calculate_md5_hash(apk_path.as_ref()).map_err(|e| e.to_string())?;
        let file_sha1 = self.calculate_file_hash::<Sha1>(apk_path.as_ref()).map_err(|e| e.to_string())?;
        let file_sha256 = self.calculate_file_hash::<Sha256>(apk_path.as_ref()).map_err(|e| e.to_string())?;
        
        Ok(ApkParsingContext {
            archive,
            file_size,
            file_md5,
            file_sha1,
            file_sha256,
        })
    }

    /// 分析权限列表
    pub fn analyze_permissions(&self, permissions: &[String]) -> PermissionAnalysis {
        let mut analysis = PermissionAnalysis::default();
        analysis.total_permissions = permissions.len() as u32;
        
        for permission in permissions {
            if self.dangerous_permissions.contains(permission) {
                analysis.dangerous_permissions += 1;
                if self.high_risk_permissions.contains(permission) {
                    analysis.high_risk_permissions.push(permission.clone());
                }
            } else if permission.starts_with("android.permission.") {
                analysis.normal_permissions += 1;
            } else if permission.starts_with("android.permission.SIGNATURE") {
                analysis.signature_permissions += 1;
            } else {
                analysis.other_permissions += 1;
            }
        }
        
        // 设置风险等级
        analysis.risk_level = if analysis.dangerous_permissions > 5 {
            "HIGH".to_string()
        } else if analysis.dangerous_permissions > 2 {
            "MEDIUM".to_string()
        } else {
            "LOW".to_string()
        };
        
        analysis
    }

    /// 分析安全配置
    pub fn analyze_security_config<P: AsRef<Path>>(&self, apk_path: P) -> Result<SecurityConfig, String> {
        let manifest = self.extract_manifest_xml()?;
        
        let mut config = SecurityConfig::default();
        
        // 检查是否使用明文流量
        config.uses_clear_text_traffic = manifest.contains("android:usesCleartextTraffic=\"true\"");
        
        // 检查是否可调试
        config.debuggable = manifest.contains("android:debuggable=\"true\"");
        
        // 检查备份设置
        config.backup_allowed = !manifest.contains("android:allowBackup=\"false\"");
        config.allow_backup = manifest.contains("android:allowBackup=\"true\"");
        
        // 检查权限标志
        config.uses_permission_flags = manifest.contains("android:protectionLevel=");
        
        // 检查网络安全配置
        config.has_network_security_config = Some(manifest.contains("android:networkSecurityConfig="));
        
        // 检查是否阻止截图
        config.prevents_screenshots = Some(manifest.contains("android:preventScreenshots=\"true\""));
        
        // 检查是否使用加密
        config.uses_encryption = Some(manifest.contains("android:encryption=\"true\""));
        
        Ok(config)
    }

    /// 解析APK文件
    pub fn parse_apk<P: AsRef<Path>>(&self, apk_path: P) -> Result<serde_json::Value, String> {
        let start_time = Instant::now();
        println!("INFO: 开始解析APK: {:?}", apk_path.as_ref());
        
        // 创建解析上下文
        let context = self.create_parsing_context(&apk_path)?;
        
        // 提取清单文件
        let manifest = extract_manifest_xml(&apk_path)?;
        
        // 解析包信息
        let (package_name, version_name, version_code, min_sdk, target_sdk, main_activity) = 
            self.parse_package_info(&manifest)?;
        
        // 提取签名信息
        let signature_info = extract_signature_info(&apk_path)?;
        
        // 提取权限
        let permissions = self.parse_permissions(&manifest)?;
        
        // 分析权限
        let permission_analysis = self.analyze_permissions(&permissions.iter().map(|p| p.name.clone()).collect::<Vec<_>>());
        
        // 分析安全配置
        let security_config = self.analyze_security_config(&apk_path)?;
        
        // 分析SDK特性
        let sdk_features = context.analyze_dex_files();
        
        // 提取图标
        let icon_base64 = extract_icon(&apk_path);
        
        // 创建APK信息对象
        let apk_info = ApkInfo {
            package_name,
            version_name,
            version_code,
            min_sdk,
            target_sdk,
            signature_info: Some(signature_info),
            permissions: Some(permissions),
            file_info: Some(FileInfo {
                md5: context.file_md5,
                sha1: context.file_sha1,
                sha256: context.file_sha256,
                file_size: context.file_size,
                file_type: "application/vnd.android.package-archive".to_string(),
                entry_count: context.archive.len() as u32,
            }),
            main_activity,
            icon_base64,
            app_labels: None,
            permission_analysis: Some(permission_analysis),
            security_config: Some(security_config),
            sdk_features: Some(sdk_features),
        };
        
        let elapsed = start_time.elapsed();
        println!("INFO: APK解析完成，耗时: {:?}", elapsed);
        
        // 转换为JSON
        Ok(serde_json::to_value(apk_info).map_err(|e| e.to_string())?)
    }

    /// 解析包信息
    pub fn parse_package_info(&self, manifest: &str) -> Result<(String, String, String, String, String, Option<String>), String> {
        // 使用正则表达式提取关键属性
        let package_regex = Regex::new(r#"package="([^"]+)"#).unwrap();
        let version_name_regex = Regex::new(r#"android:versionName="([^"]+)"#).unwrap();
        let version_code_regex = Regex::new(r#"android:versionCode="([^"]+)"#).unwrap();
        let min_sdk_regex = Regex::new(r#"android:minSdkVersion="([^"]+)"#).unwrap();
        let target_sdk_regex = Regex::new(r#"android:targetSdkVersion="([^"]+)"#).unwrap();
        let main_activity_regex = Regex::new(r#"<activity[^>]*android:name="([^"]+)"[^>]*>.*?<intent-filter>.*?<action android:name="android.intent.action.MAIN".*?>.*?<category android:name="android.intent.category.LAUNCHER".*?>.*?</intent-filter>"#).unwrap();

        let package_name = package_regex.captures(manifest)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        let version_name = version_name_regex.captures(manifest)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_else(|| "1.0".to_string());

        let version_code = version_code_regex.captures(manifest)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_else(|| "1".to_string());

        let min_sdk = min_sdk_regex.captures(manifest)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_else(|| "1".to_string());

        let target_sdk = target_sdk_regex.captures(manifest)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_else(|| min_sdk.clone());

        let main_activity = main_activity_regex.captures(manifest)
            .and_then(|caps| caps.get(1).map(|m| {
                let activity = m.as_str().to_string();
                if !activity.contains(".") && !activity.starts_with(&package_name) {
                    format!("{}.{}", package_name, activity)
                } else {
                    activity
                }
            }));

        Ok((package_name, version_name, version_code, min_sdk, target_sdk, main_activity))
    }

    /// 解析权限列表
    pub fn parse_permissions(&self, manifest: &str) -> Result<Vec<Permission>, String> {
        let permissions_regex = Regex::new(r#"<uses-permission[^>]*android:name="([^"]+)"[^>]*/?>"#).unwrap();
        let mut permissions = Vec::new();

        for cap in permissions_regex.captures_iter(manifest) {
            if let Some(perm) = cap.get(1) {
                let permission_name = perm.as_str().to_string();
                let is_dangerous = self.dangerous_permissions.contains(&permission_name);
                permissions.push(Permission {
                    name: permission_name,
                    is_dangerous,
                });
            }
        }

        Ok(permissions)
    }

    /// 从清单中提取特定标签的属性
    pub fn extract_from_manifest(manifest: &str, tag: &str, attribute: &str) -> Option<String> {
        let tag_pattern = format!(r#"<{}\s+[^>]*{}\s*=\s*"([^"]+)""#, tag, attribute);
        let regex = Regex::new(&tag_pattern).ok()?;
        regex.captures(manifest).and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
    }

    // 辅助方法：计算文件哈希
    fn calculate_file_hash<D>(&self, path: &Path) -> Result<String, ApkParserError> 
    where
        D: digest::Digest + Default,
        digest::Output<D>: core::fmt::LowerHex,
    {
        let mut file = File::open(path)?;
        let mut hasher = D::default();
        let mut buffer = [0; 1024];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    // 辅助方法：计算MD5哈希
    fn calculate_md5_hash(&self, path: &Path) -> Result<String, ApkParserError> {
        let mut file = File::open(path)?;
        let mut context = md5::Context::new();
        let mut buffer = [0; 1024];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            context.consume(&buffer[..bytes_read]);
        }
        
        let digest = context.compute();
        Ok(format!("{:x}", digest))
    }

    pub fn set_archive(&mut self, archive: ZipArchive<File>) {
        self.archive = Some(archive);
    }

    // 从aapt2输出中提取信息
    pub fn extract_from_aapt_output(output: &str, pattern: &str) -> Option<String> {
        let regex = Regex::new(pattern).ok()?;
        regex.captures(output)
            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
    }

    // 提取权限信息
    pub fn extract_permissions(&self, manifest_xml: &str) -> Vec<Permission> {
        let mut permissions = Vec::new();
        let permission_regex = Regex::new(r#"<uses-permission[^>]*android:name="([^"]+)"[^>]*/?>"#).unwrap();
        
        for cap in permission_regex.captures_iter(manifest_xml) {
            if let Some(perm) = cap.get(1) {
                let name = perm.as_str().to_string();
                let is_dangerous = self.dangerous_permissions.contains(&name);
                permissions.push(Permission {
                    name: name.clone(),
                    protection_level: if is_dangerous { "dangerous".to_string() } else { "normal".to_string() },
                    description: format!("Permission: {}", name),
                    group: "android".to_string(),
                    is_dangerous,
                });
            }
        }
        
        permissions
    }
}

impl ApkParsingContext {
    /// 分析DEX文件以提取SDK特性
    pub fn analyze_dex_files(&mut self) -> SdkFeatures {
        let mut features = SdkFeatures::default();
        
        // 这里实现DEX文件分析逻辑
        // 由于DEX分析比较复杂，这里只提供基本框架
        
        features
    }
} 