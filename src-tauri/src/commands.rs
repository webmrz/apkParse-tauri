use serde::{Deserialize, Serialize};
use std::path::{Path};
use std::sync::{Arc, Mutex}; 
use tempfile::NamedTempFile;
use chrono;
use crate::apk_parser::ApkParser;
use sysinfo::{System, CpuRefreshKind, RefreshKind, MemoryRefreshKind, ProcessRefreshKind};

// Constants
const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_MAX_LOG_DAYS: i32 = 7;

// Type definitions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApkInfo {
    pub package_name: String,
    pub version_name: String,
    pub version_code: String,
    pub min_sdk: String,
    pub target_sdk: String,
    pub signature_info: Option<SignatureInfo>,
    pub permissions: Option<Vec<Permission>>,
    pub dangerous_permissions: Vec<Permission>,
    pub permission_stats: PermissionStats,
    pub is_certificate_expired: bool,
    pub formatted_version_info: String,
    pub formatted_sdk_info: String,
    pub file_info: Option<FileInfo>,
    pub main_activity: Option<String>,
    pub icon_base64: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignatureInfo {
    pub issuer: String,
    pub subject: String,
    pub valid_from: String,
    pub valid_to: String,
    pub fingerprint_sha1: Option<String>,
    pub fingerprint_sha256: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Permission {
    pub name: String,
    pub is_dangerous: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionStats {
    pub total: usize,
    pub dangerous: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub file_size: u64,
    pub file_type: String,
    pub entry_count: u32,
}

#[derive(Deserialize)]
pub struct ApkDataParams { 
}

#[derive(Deserialize)]
pub struct TempFileParams { 
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub log_level: String,
    pub max_log_days: i32,
    pub auto_start: bool,
    pub auto_update: bool,
}

#[derive(Serialize)]
pub struct AppInfo {
    pub version: String,
    pub build_time: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub cpu_info: CpuInfo,
    pub memory_info: MemoryInfo,
    pub host_name: String,
    // pub system_uptime: u64,
}

#[derive(Serialize)]
pub struct CpuInfo {
    pub brand: String,
    pub cores_count: usize,
    pub usage_percent: f32,
}

#[derive(Serialize)]
pub struct MemoryInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

// Global state
lazy_static::lazy_static! {
    static ref TEMP_FILES: Arc<Mutex<Vec<NamedTempFile>>> = Arc::new(Mutex::new(Vec::new()));
}

// Implementation of default settings
impl Default for AppSettings {
    fn default() -> Self {
        Self {
            log_level: DEFAULT_LOG_LEVEL.to_string(),
            max_log_days: DEFAULT_MAX_LOG_DAYS,
            auto_start: false,
            auto_update: true,
        }
    }
}

// Tauri commands
#[tauri::command]
pub async fn parse_apk(path: String) -> Result<ApkInfo, String> {
    let path = Path::new(&path);
    let parser_result = ApkParser::parse(path)
        .map_err(|e| e.to_string())?;
    
    let signature_info = parser_result.signature_info.as_ref().map(|sig| SignatureInfo {
        issuer: sig.issuer.clone(),
        subject: sig.subject.clone(),
        valid_from: sig.valid_from.clone(),
        valid_to: sig.valid_to.clone(),
        fingerprint_sha1: sig.fingerprint_sha1.clone(),
        fingerprint_sha256: sig.fingerprint_sha256.clone(),
    });
    
    let permissions = parser_result.permissions.clone().unwrap_or_default();
    
    let dangerous_permissions: Vec<Permission> = permissions.iter()
        .filter(|p| p.is_dangerous)
        .map(|p| Permission {
            name: p.name.clone(),
            is_dangerous: true,
        })
        .collect();
    
    let permission_stats = PermissionStats {
        total: permissions.len(),
        dangerous: dangerous_permissions.len(),
    };
    
    let is_certificate_expired = if let Some(sig) = parser_result.signature_info.as_ref() {
        match chrono::DateTime::parse_from_rfc2822(&sig.valid_to) {
            Ok(expiry_date) => expiry_date < chrono::Utc::now(),
            Err(_) => false
        }
    } else {
        false
    };
    
    let formatted_version_info = format!("{} ({})", 
        parser_result.version_name, 
        parser_result.version_code
    );
    
    let formatted_sdk_info = format!("Min SDK: {}, Target SDK: {}", 
        parser_result.min_sdk, 
        parser_result.target_sdk
    );
    
    let permissions_vec = permissions.into_iter()
        .map(|p| Permission {
            name: p.name,
            is_dangerous: p.is_dangerous,
        })
        .collect();
    
    let file_info = parser_result.file_info.map(|fi| FileInfo {
        md5: fi.md5,
        sha1: fi.sha1,
        sha256: fi.sha256,
        file_size: fi.file_size,
        file_type: fi.file_type,
        entry_count: fi.entry_count,
    });
    
    Ok(ApkInfo {
        package_name: parser_result.package_name,
        version_name: parser_result.version_name,
        version_code: parser_result.version_code,
        min_sdk: parser_result.min_sdk,
        target_sdk: parser_result.target_sdk,
        signature_info,
        permissions: Some(permissions_vec),
        dangerous_permissions,
        permission_stats,
        is_certificate_expired,
        formatted_version_info,
        formatted_sdk_info,
        file_info,
        main_activity: parser_result.main_activity,
        icon_base64: parser_result.icon_base64,
    })
}


#[tauri::command]
pub async fn get_app_info() -> Result<AppInfo, String> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
            .with_processes(ProcessRefreshKind::everything())
    );
    sys.refresh_memory();
    sys.refresh_cpu();
    
    // Get CPU information
    let cpu = sys.cpus().first().unwrap_or_else(|| sys.global_cpu_info());
    let cpu_info = CpuInfo {
        brand: cpu.brand().to_string(),
        cores_count: sys.cpus().len(),
        usage_percent: cpu.cpu_usage(),
    };
    
    // Get memory information
    let memory_info = MemoryInfo {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
    };
    
    Ok(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        build_time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        os_name: System::long_os_version().unwrap_or_else(|| "Unknown".to_string()),
        os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        cpu_info,
        memory_info,
        host_name: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        // system_uptime: sys.boot_time(),
    })
}

#[tauri::command]
pub async fn select_apk_file(window: tauri::Window) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;
    
    let handle = window.dialog();
    let file_dialog = handle.file().add_filter("APK Files", &["apk"]);
    
    // In Tauri 2.x, pick_file is not async and needs a callback
    let (tx, rx) = std::sync::mpsc::channel();
    file_dialog.pick_file(move |file_path| {
        let _ = tx.send(file_path);
    });
    
    // Wait for the result
    match rx.recv() {
        Ok(Some(path)) => Ok(path.to_string()),
        Ok(None) => Err("No file selected".to_string()),
        Err(_) => Err("Failed to get file selection result".to_string())
    }
}

 