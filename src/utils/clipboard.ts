import { ElMessage } from 'element-plus';

/**
 * 检查剪贴板API是否可用
 */
const isClipboardApiSupported = (): boolean => {
  return !!navigator.clipboard && typeof navigator.clipboard.writeText === 'function';
};

/**
 * 复制文本到剪贴板
 * @param text 要复制的文本
 * @param successMessage 成功提示信息
 * @returns 复制是否成功
 */
export const copyToClipboard = async (text: string, successMessage: string = '已复制到剪贴板'): Promise<boolean> => {
  if (!text) {
    ElMessage.warning('没有可复制的内容');
    return false;
  }
  
  // 首先尝试使用现代Clipboard API
  if (isClipboardApiSupported()) {
    try {
      await navigator.clipboard.writeText(text);
      ElMessage.success(successMessage);
      return true;
    } catch (err) {
      console.error('使用Clipboard API复制失败:', err);
      // 失败后继续使用备用方法
    }
  }
  
  // 备用复制方法 (execCommand)
  try {
    const textarea = document.createElement('textarea');
    textarea.value = text;
    textarea.style.position = 'fixed';
    textarea.style.left = '-9999px'; // 移到屏幕外
    textarea.style.top = '-9999px';
    textarea.style.opacity = '0';
    textarea.style.zIndex = '-1';
    
    // 添加到DOM
    document.body.appendChild(textarea);
    
    // 检查可见性和焦点问题
    if (getComputedStyle(textarea).display === 'none') {
      textarea.style.display = 'block';
    }
    
    // 选择文本
    textarea.focus();
    textarea.select();
    
    // 尝试执行复制命令
    const successful = document.execCommand('copy');
    
    if (successful) {
      ElMessage.success(successMessage);
      return true;
    } else {
      throw new Error('execCommand返回失败');
    }
  } catch (err) {
    console.error('备用复制方法失败:', err);
    ElMessage.error('复制失败，请手动复制');
    return false;
  } finally {
    // 清理DOM
    const textarea = document.querySelector('textarea[style*="opacity: 0"]');
    if (textarea) {
      document.body.removeChild(textarea);
    }
  }
}; 