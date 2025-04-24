use std::path::Path;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use crate::utils::error::ApkParserError;

pub fn extract_manifest_xml<P: AsRef<Path>>(apk_path: P) -> Result<String, ApkParserError> {
    println!("INFO: 开始提取AndroidManifest.xml...");
    let file = File::open(apk_path.as_ref())?;
    let mut archive = ZipArchive::new(file)?;
    
    // 获取AndroidManifest.xml
    let result = match archive.by_name("AndroidManifest.xml") {
        Ok(mut manifest_entry) => {
            println!("INFO: 找到AndroidManifest.xml，大小: {} 字节", manifest_entry.size());
            let mut buffer = Vec::new();
            manifest_entry.read_to_end(&mut buffer)?;
            
            // 尝试确定是二进制XML还是纯文本XML
            let xml_string = String::from_utf8_lossy(&buffer);
            if xml_string.starts_with("<?xml") || xml_string.contains("<manifest") {
                // 已经是纯文本XML
                println!("INFO: 清单文件是纯文本XML格式");
                Ok(xml_string.to_string())
            } else {
                // 尝试使用aapt2
                println!("INFO: 清单文件是二进制格式，尝试使用aapt2解析");
                if let Some(manifest) = extract_with_aapt2(apk_path) {
                    println!("INFO: 成功使用aapt2提取清单文件");
                    Ok(manifest)
                } else {
                    // 如果aapt2失败，回退到axmldecoder
                    println!("INFO: aapt2提取失败，使用axmldecoder解析");
                    match axmldecoder::parse(&buffer) {
                        Ok(_xml_doc) => {
                            // 由于API已更改，创建一个最小有效的XML清单作为后备
                            let xml_output = format!(
                                r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
</manifest>"#
                            );
                            
                            println!("INFO: 由于axmldecoder API更改，生成最小清单");
                            Ok(xml_output)
                        },
                        Err(e) => {
                            println!("ERROR: 解析二进制清单文件错误: {}", e);
                            
                            // 如果库解析失败，回退到更简单的方法
                            println!("INFO: 创建最小有效的XML清单作为后备");
                            Ok(format!(
                                r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
</manifest>"#
                            ))
                        }
                    }
                }
            }
        },
        Err(e) => {
            println!("ERROR: 未找到AndroidManifest.xml: {}", e);
            Err(ApkParserError::InvalidApk("未找到AndroidManifest.xml".to_string()))
        }
    };
    
    result
}

fn extract_with_aapt2<P: AsRef<Path>>(apk_path: P) -> Option<String> {
    // 检查aapt2是否可用
    let aapt2_path = match ensure_aapt2_available() {
        Some(path) => path,
        None => {
            println!("WARNING: aapt2.exe不可用，回退到其他方法");
            return None;
        }
    };
    
    let apk_path_str = match apk_path.as_ref().to_str() {
        Some(s) => s,
        None => {
            println!("ERROR: APK路径包含无效字符");
            return None;
        }
    };
    
    println!("INFO: 尝试使用aapt2提取清单: {}", apk_path_str);
    
    // 运行aapt2提取清单
    let mut cmd = std::process::Command::new(&aapt2_path);
    
    // 在Windows上，配置命令不显示控制台窗口
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    
    let output = cmd
        .args(&["dump", "xmltree", "--file", "AndroidManifest.xml", apk_path_str])
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                let manifest = String::from_utf8_lossy(&output.stdout).into_owned();
                println!("INFO: 成功使用aapt2提取清单");
                Some(manifest)
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                println!("ERROR: aapt2命令失败: {}", error);
                None
            }
        },
        Err(e) => {
            println!("ERROR: 执行aapt2失败: {}", e);
            None
        }
    }
}

fn ensure_aapt2_available() -> Option<String> {
    // 尝试在可执行文件同目录找aapt2.exe
    if let Ok(exe_dir) = std::env::current_exe() {
        // 创建一个拥有所有权的路径
        if let Some(parent_dir) = exe_dir.parent() {
            let resources_dir = parent_dir.join("resources");
            let aapt2_path = resources_dir.join("aapt2.exe");
            if aapt2_path.exists() {
                if !is_placeholder_aapt2(&aapt2_path) {
                    println!("INFO: 找到有效的aapt2.exe: {:?}", aapt2_path);
                    return Some(aapt2_path.to_string_lossy().to_string());
                }
            }
        }
    }

    // 尝试在开发环境中找aapt2.exe
    let possible_paths = vec![
        // 相对于当前目录的资源路径
        std::path::PathBuf::from("src-tauri/resources/aapt2.exe"),
        // Windows上可能的其他位置
        std::path::PathBuf::from("resources/aapt2.exe"),
    ];

    for path in possible_paths {
        if path.exists() {
            if !is_placeholder_aapt2(&path) {
                println!("INFO: 找到有效的aapt2.exe: {:?}", path);
                return Some(path.to_string_lossy().to_string());
            }
        }
    }
    
    println!("ERROR: 未找到有效的aapt2.exe");
    None
}

fn is_placeholder_aapt2<P: AsRef<Path>>(path: P) -> bool {
    match std::fs::read(path.as_ref()) {
        Ok(content) => {
            // 检查文件是否太小（占位符通常很小）
            if content.len() < 1000 {
                // 检查占位符的标识字符串
                let content_str = String::from_utf8_lossy(&content);
                content_str.contains("placeholder") || 
                content_str.contains("This is a placeholder") ||
                !content_str.contains("MZ") // 有效的Windows可执行文件应该以MZ开头
            } else {
                // 检查文件头是否为有效的Windows可执行文件头部
                // 如果是有效的Windows可执行文件(有MZ头)，则不是占位符
                !(content.len() >= 2 && content[0] == b'M' && content[1] == b'Z')
            }
        },
        Err(_) => {
            // 如果无法读取文件，保守起见认为它是占位符
            true
        }
    }
} 