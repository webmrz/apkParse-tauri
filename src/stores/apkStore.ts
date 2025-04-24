import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

// 类型定义
export interface Permission {
  name: string;
  is_dangerous: boolean;
}

export interface SignatureInfo {
  issuer: string;
  subject: string;
  valid_from: string;
  valid_to: string;
  fingerprint_sha1?: string;
  fingerprint_sha256?: string;
}

export interface PermissionStats {
  total: number;
  dangerous: number;
}

export interface ApkInfo {
  package_name: string;
  version_name: string;
  version_code: string;
  min_sdk: string;
  target_sdk: string;
  permissions?: Permission[];
  signature_info?: SignatureInfo;
  dangerous_permissions: Permission[];
  permission_stats: PermissionStats;
  is_certificate_expired: boolean;
  formatted_version_info: string;
  formatted_sdk_info: string;
  file_info: any;
  icon_base64?: string;
}

export interface ApkFileInfo {
  file_name: string;
  file_path: string;
  file_size: number;
  icon_base64?: string;
}

export interface ApkHistoryItem {
  id: string; 
  apk_info: ApkInfo;
  analyzed_at: string;
  file_info?: ApkFileInfo;
}

// 本地存储键名
const STORAGE_KEY_HISTORY = 'apk-analyzer-history';
const STORAGE_KEY_LAST_ANALYSIS = 'apk-analyzer-last-analysis';

// Store 定义
export const useApkStore = defineStore('apk', () => {
  // 状态定义
  const currentApkFile = ref<ApkFileInfo | null>(null);
  const apkInfo = ref<ApkInfo | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const historyItems = ref<ApkHistoryItem[]>([]);

  // =============== 核心功能方法 ===============

  /**
   * 初始化store
   */
  function initialize() {
    loadHistory();
    loadLastAnalysis();
  }

  /**
   * 解析APK文件
   */
  async function parseApk(path: string) {
    loading.value = true;
    error.value = null;
    console.log('开始解析APK文件:', path);
    try {
      console.log('调用Tauri命令parse_apk，路径:', path);
      const result = await invoke<ApkInfo>('parse_apk', { path });
      console.log('Tauri parse_apk命令执行成功，结果:', result); 
     
      
      // 设置解析结果
      apkInfo.value = result;
      
      // 添加到历史记录
      addToHistory();
      
      // 保存为最近分析
      saveLastAnalysis();
      
      return result;
    } catch (e) {
      console.error('解析APK文件失败:', e);
      error.value = e as string;
      throw e;
    } finally {
      loading.value = false;
    }
  }

 

  // =============== 历史记录相关方法 ===============

  /**
   * 从localStorage加载历史记录
   */
  function loadHistory() {
    try {
      const storedHistory = localStorage.getItem(STORAGE_KEY_HISTORY);
      if (storedHistory) {
        historyItems.value = JSON.parse(storedHistory);
      }
    } catch (err) {
      console.error('从localStorage加载历史记录失败:', err);
    }
  }

  /**
   * 保存历史记录到localStorage
   */
  function saveHistory() {
    try {
      localStorage.setItem(STORAGE_KEY_HISTORY, JSON.stringify(historyItems.value));
    } catch (err) {
      console.error('保存历史记录到localStorage失败:', err);
    }
  }

  /**
   * 生成唯一ID
   */
  function generateId() {
    return Date.now().toString(36) + Math.random().toString(36).substring(2);
  }

  /**
   * 添加当前分析结果到历史记录
   */
  function addToHistory() {
    if ( !apkInfo.value) {
      return;
    }

    const newItem: ApkHistoryItem = {
      id: generateId(), 
      apk_info: { ...apkInfo.value },
      analyzed_at: new Date().toISOString(),
    };

    const existingIndex = historyItems.value.findIndex(
      (item) =>
        item.apk_info.package_name === apkInfo.value?.package_name &&
        item.apk_info.version_name === apkInfo.value?.version_name
    );

    if (existingIndex !== -1) {
      historyItems.value[existingIndex] = newItem;
    } else {
      historyItems.value.unshift(newItem);
      if (historyItems.value.length > 10) {
        historyItems.value = historyItems.value.slice(0, 10);
      }
    }

    saveHistory();
  }

  /**
   * 从历史记录加载特定项
   */
  function loadFromHistory(historyId: string) {
    try {
      const item = historyItems.value.find((h) => h.id === historyId);
      if (!item) {
        throw new Error(`历史记录项不存在: ${historyId}`);
      }
      
      // Create a file info object from apk_info if file_info doesn't exist
      if (!item.file_info) {
        const apkInfo = item.apk_info;
        currentApkFile.value = {
          file_name: `${apkInfo.package_name}-${apkInfo.version_name}.apk`,
          file_path: '',
          file_size: 0,
          icon_base64: apkInfo.icon_base64
        };
      } else {
        currentApkFile.value = { ...item.file_info };
      }
      
      apkInfo.value = { ...item.apk_info };
      saveLastAnalysis();
      return true;
    } catch (error) {
      console.error('从历史记录加载失败:', error);
      return false;
    }
  }

  /**
   * 从历史记录中删除特定项
   */
  function removeFromHistory(historyId: string) {
    historyItems.value = historyItems.value.filter((item) => item.id !== historyId);
    saveHistory();
  }

  /**
   * 清空历史记录
   */
  function clearHistory() {
    historyItems.value = [];
    saveHistory();
  }

  // =============== 最近分析相关方法 ===============

  /**
   * 加载上次分析结果
   */
  function loadLastAnalysis() {
    try {
      const lastAnalysis = localStorage.getItem(STORAGE_KEY_LAST_ANALYSIS);
      if (lastAnalysis) {
        const data = JSON.parse(lastAnalysis);
        currentApkFile.value = data.fileInfo;
        apkInfo.value = data.apkInfo;
      }
    } catch (err) {
      console.error('加载上次分析结果失败:', err);
    }
  }

  /**
   * 保存当前分析为最近分析
   */
  function saveLastAnalysis() {
    if (currentApkFile.value && apkInfo.value) {
      try {
        localStorage.setItem(
          STORAGE_KEY_LAST_ANALYSIS,
          JSON.stringify({
            fileInfo: currentApkFile.value,
            apkInfo: apkInfo.value,
          })
        );
      } catch (err) {
        console.error('保存最近分析失败:', err);
      }
    }
  }

  /**
   * 清除当前分析结果
   */
  function clearCurrentAnalysis() {
    currentApkFile.value = null;
    apkInfo.value = null;
    error.value = null;
    localStorage.removeItem(STORAGE_KEY_LAST_ANALYSIS);
  }

 

  // 初始化
  initialize();

  // 返回store接口
  return {
    // 状态
    currentApkFile,
    apkInfo,
    loading,
    error,
    historyItems,

    // 方法
    parseApk,  
    loadFromHistory,
    removeFromHistory,
    clearHistory,
    clearCurrentAnalysis,

    // 计算属性
    dangerousPermissions: computed(() => apkInfo.value?.dangerous_permissions || []),
    isCertificateExpired: computed(() => apkInfo.value?.is_certificate_expired || false),
    formattedVersionInfo: computed(() => apkInfo.value?.formatted_version_info || ''),
    hasAnalysisResult: computed(() => !!apkInfo.value),
    formattedSdkInfo: computed(() => apkInfo.value?.formatted_sdk_info || ''),
    permissionStats: computed(() => apkInfo.value?.permission_stats || { total: 0, dangerous: 0 }),
  };
}); 