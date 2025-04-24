import { ApkInfo, FileInfo } from '../types/apk';

/**
 * 生成APK分析报告的HTML内容
 * @param info APK信息对象
 * @param fileInfo 文件信息对象（可选）
 * @returns HTML字符串
 */
export function generateReportHTML(info: ApkInfo, fileInfo?: FileInfo): string {
  const formatFileSize = (size: number) => {
    return (size / 1024 / 1024).toFixed(2) + ' MB';
  };

  const formatDate = (dateStr: string) => {
    return new Date(dateStr).toLocaleString();
  };

  let htmlContent = `
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>APK分析报告 - ${info.package_name}</title>
  <style>
    body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 800px; margin: 0 auto; padding: 20px; }
    h1, h2, h3 { color: #2c3e50; }
    .section { margin-bottom: 30px; border: 1px solid #eee; padding: 20px; border-radius: 5px; }
    .danger { color: #f56c6c; }
    .success { color: #67c23a; }
    .warning { color: #e6a23c; }
    table { width: 100%; border-collapse: collapse; margin-bottom: 15px; }
    table, th, td { border: 1px solid #eee; }
    th, td { padding: 10px; text-align: left; }
    th { background-color: #f7f7f7; }
    .app-header { display: flex; align-items: center; gap: 20px; margin-bottom: 20px; }
    .app-icon { width: 64px; height: 64px; border-radius: 8px; }
    .app-info { flex: 1; }
    .hash-value { font-family: monospace; background: #f7f7f7; padding: 5px; border-radius: 4px; }
  </style>
</head>
<body>
  <h1>APK分析报告</h1>
  
  <div class="section app-header">
    ${info.icon_base64 ? 
      `<img src="data:image/png;base64,${info.icon_base64}" alt="App Icon" class="app-icon">` :
      '<div class="app-icon" style="background: #eee;"></div>'
    }
    <div class="app-info">
      <h2 style="margin: 0;">${info.package_name}</h2>
      <p style="margin: 5px 0;">版本: ${info.version_name} (${info.version_code})</p>
      <p style="margin: 5px 0;">SDK: Android ${info.min_sdk} - ${info.target_sdk}</p>
    </div>
  </div>

  <div class="section">
    <h2>文件信息</h2>
    <table>
      <tr><th>文件名</th><td>${fileInfo?.file_name || '未知'}</td></tr>
      <tr><th>文件大小</th><td>${formatFileSize(info.file_info?.file_size || 0)}</td></tr>
      <tr><th>文件类型</th><td>${info.file_info?.file_type || '未知'}</td></tr>
      <tr><th>MD5</th><td><span class="hash-value">${info.file_info?.md5 || '未知'}</span></td></tr>
      <tr><th>SHA-1</th><td><span class="hash-value">${info.file_info?.sha1 || '未知'}</span></td></tr>
      <tr><th>SHA-256</th><td><span class="hash-value">${info.file_info?.sha256 || '未知'}</span></td></tr>
    </table>
  </div>`;

  if (info.signature_info) {
    htmlContent += `
  <div class="section">
    <h2>证书信息</h2>
    <table>
      <tr><th>发行者</th><td>${info.signature_info.issuer}</td></tr>
      <tr><th>主题</th><td>${info.signature_info.subject}</td></tr>
      <tr><th>有效期从</th><td>${formatDate(info.signature_info.valid_from)}</td></tr>
      <tr><th>有效期至</th><td>${formatDate(info.signature_info.valid_to)}</td></tr>
      <tr><th>SHA-1指纹</th><td><span class="hash-value">${info.signature_info.fingerprint_sha1 || '未知'}</span></td></tr>
      <tr><th>SHA-256指纹</th><td><span class="hash-value">${info.signature_info.fingerprint_sha256 || '未知'}</span></td></tr>
    </table>
  </div>`;
  }

  if (info.permissions && info.permissions.length > 0) {
    const dangerousPerms = info.permissions.filter(p => p.is_dangerous);
    const normalPerms = info.permissions.filter(p => !p.is_dangerous);

    htmlContent += `
  <div class="section">
    <h2>权限信息</h2>
    <p>总计: ${info.permissions.length}个权限 (危险权限: ${dangerousPerms.length}个)</p>
    
    ${dangerousPerms.length > 0 ? `
    <h3>危险权限</h3>
    <ul class="danger">
      ${dangerousPerms.map(p => `<li>${p.name}</li>`).join('')}
    </ul>` : ''}
    
    ${normalPerms.length > 0 ? `
    <h3>普通权限</h3>
    <ul>
      ${normalPerms.map(p => `<li>${p.name}</li>`).join('')}
    </ul>` : ''}
  </div>`;
  }

  htmlContent += `
  <div class="section">
    <p style="color: #999;">生成时间: ${new Date().toLocaleString()}</p>
  </div>
</body>
</html>`;

  return htmlContent;
} 