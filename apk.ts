import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface SignatureInfo {
  issuer: string
  subject: string
  valid_from: string
  valid_to: string
  fingerprint_sha1?: string
  fingerprint_sha256?: string
}

export interface Permission {
  name: string
  is_dangerous: boolean
}

export interface PermissionStats {
  total: number
  dangerous: number
}

export interface FileInfo {
  md5: string
  sha1: string
  sha256: string
  file_size: number
  file_type: string
  entry_count: number
}

export interface ApkInfo {
  package_name: string
  version_name: string
  version_code: string
  min_sdk: string
  target_sdk: string
  signature_info?: SignatureInfo
  permissions?: Permission[]
  dangerous_permissions: Permission[]
  permission_stats: PermissionStats
  is_certificate_expired: boolean
  formatted_version_info: string
  formatted_sdk_info: string
  file_info?: FileInfo
  main_activity?: string
  icon_base64?: string
}

export interface ApkFile {
  info: ApkInfo
  icon_base64?: string
  file_name: string
  file_size: number
  parsed_at: string
}

export const useApkStore = defineStore('apk', () => {
  const apkInfo = ref<ApkInfo | null>(null)
  const currentApkFile = ref<ApkFile | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // 判断证书是否过期
  const isCertificateExpired = computed(() => {
    if (!apkInfo.value) return false;
    return apkInfo.value.is_certificate_expired;
  });

  async function parseApk(path: string) {
    loading.value = true
    error.value = null
    console.log('开始解析APK文件:', path)
    try {
      console.log('调用Tauri命令parse_apk，路径:', path)
      const result = await invoke<ApkInfo>('parse_apk', { path })
      console.log('Tauri parse_apk命令执行成功，结果:', result)
      apkInfo.value = result
    } catch (e) {
      console.error('解析APK文件失败:', e)
      error.value = e as string
    } finally {
      loading.value = false
    }
  }
  
  /**
   * 解析APK数据
   * @param fileName 文件名
   * @param data APK数据
   */
  async function parseApkData(fileName: string, data: Uint8Array) {
    loading.value = true
    error.value = null
    console.log('开始解析APK数据, 文件名:', fileName, '数据长度:', data.length)
    try {
      loading.value = true
      error.value = null
      console.log('调用Tauri命令parse_apk_data')
      // 直接传递参数，而不是作为一个对象
      const result = await invoke<ApkInfo>('parse_apk_data', { 
        apk_data: Array.from(data), 
        file_name: fileName 
      })
      console.log('Tauri parse_apk_data命令执行成功，结果:', result)
      
      apkInfo.value = result
      currentApkFile.value = {
        info: result,
        file_name: fileName,
        file_size: data.length,
        parsed_at: new Date().toISOString()
      }
      loading.value = false
      return result
    } catch (e) {
      console.error('解析APK数据失败:', e)
      error.value = e as string
      loading.value = false
      throw e
    }
  }
  
  // 直接设置 APK 信息
  function setApkInfo(info: ApkInfo | ApkFile) {
    if ('info' in info) {
      // It's an ApkFile
      apkInfo.value = info.info;
      currentApkFile.value = info;
    } else {
      // It's an ApkInfo
      apkInfo.value = info;
    }
  }

  return {
    apkInfo,
    currentApkFile,
    loading,
    error,
    isCertificateExpired,
    parseApk,
    parseApkData,
    setApkInfo
  }
}) 