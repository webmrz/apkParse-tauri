use std::path::Path;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::utils::error::ApkParserError;

pub fn extract_icon<P: AsRef<Path>>(apk_path: P) -> Result<Option<String>, ApkParserError> {
    println!("INFO: 尝试提取应用图标...");
    let file = File::open(apk_path.as_ref())?;
    let mut archive = ZipArchive::new(file)?;
    
    // 按优先级尝试提取不同分辨率的图标
    let icon_paths = vec![
        // 标准mipmap目录 (Android推荐)
        "res/mipmap-xxxhdpi/ic_launcher.png",
        "res/mipmap-xxhdpi/ic_launcher.png",
        "res/mipmap-xhdpi/ic_launcher.png",
        "res/mipmap-hdpi/ic_launcher.png",
        "res/mipmap-mdpi/ic_launcher.png",
        
        // 圆形图标 (Android 8.0+)
        "res/mipmap-xxxhdpi/ic_launcher_round.png",
        "res/mipmap-xxhdpi/ic_launcher_round.png",
        "res/mipmap-xhdpi/ic_launcher_round.png",
        "res/mipmap-hdpi/ic_launcher_round.png",
        "res/mipmap-mdpi/ic_launcher_round.png",
        
        // 前景图标 (自适应图标)
        "res/mipmap-xxxhdpi/ic_launcher_foreground.png",
        "res/mipmap-xxhdpi/ic_launcher_foreground.png",
        "res/mipmap-xhdpi/ic_launcher_foreground.png",
        "res/mipmap-hdpi/ic_launcher_foreground.png",
        "res/mipmap-mdpi/ic_launcher_foreground.png",
        
        // 标准drawable目录
        "res/drawable/ic_launcher.png",
        "res/drawable-xxxhdpi/ic_launcher.png",
        "res/drawable-xxhdpi/ic_launcher.png",
        "res/drawable-xhdpi/ic_launcher.png",
        "res/drawable-hdpi/ic_launcher.png",
        "res/drawable-mdpi/ic_launcher.png",
    ];
    
    // 尝试提取第一个找到的图标
    for path in icon_paths {
        if let Ok(mut file) = archive.by_name(path) {
            let mut buffer = Vec::new();
            if file.read_to_end(&mut buffer).is_ok() {
                println!("INFO: 成功提取图标: {}", path);
                // 转换为base64
                return Ok(Some(BASE64.encode(&buffer)));
            }
        }
    }
    
    println!("WARNING: 未找到应用图标");
    Ok(None)
} 