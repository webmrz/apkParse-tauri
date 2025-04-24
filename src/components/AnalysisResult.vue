<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useApkStore } from '../stores/apkStore';
import { ElMessage, ElNotification } from 'element-plus';
import { 
  Document, 
  Lock, 
  Warning, 
  Key, 
  CopyDocument, 
  Download,
  Collection,
  List,
  CircleCheck,
  Medal,
  Upload
} from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import { copyToClipboard } from '../utils/clipboard';
import { generateReportHTML } from '../utils/reportGenerator';

// ==================== 组件状态管理 ====================
const props = defineProps({
  apkPath: {
    type: String,
    default: ''
  }
});

const apkStore = useApkStore();
const activeTab = ref('security');
const permissionSearchQuery = ref('');
const generatingReport = ref(false);

// ==================== 基础信息计算属性 ====================
const hasApkInfo = computed(() => !!apkStore.apkInfo);
const fileIcon = computed(() => apkStore.apkInfo?.icon_base64 || '');

// ==================== 证书相关计算属性 ====================
const certificateStatus = computed(() => {
  if (!apkStore.apkInfo?.signature_info) return { type: 'info', label: '无证书' };
  return apkStore.isCertificateExpired 
    ? { type: 'danger', label: '已过期' } 
    : { type: 'success', label: '有效' };
});

// 证书有效期进度计算
const certificateProgress = computed(() => {
  if (!apkStore.apkInfo?.signature_info) return 0;
  
  const validFrom = new Date(apkStore.apkInfo.signature_info.valid_from);
  const validTo = new Date(apkStore.apkInfo.signature_info.valid_to);
  const now = new Date();
  
  const totalDuration = validTo.getTime() - validFrom.getTime();
  const elapsed = now.getTime() - validFrom.getTime();
  
  if (elapsed >= totalDuration) return 100;
  return Math.floor((elapsed / totalDuration) * 100);
});

const certificateProgressStatus = computed(() => {
  if (!apkStore.apkInfo?.signature_info) return '';
  if (apkStore.isCertificateExpired) return 'exception';
  if (certificateProgress.value > 80) return 'warning';
  return 'success';
});

// ==================== 权限相关计算属性 ====================
const filteredPermissions = computed(() => {
  if (!apkStore.apkInfo?.permissions) return { normal: [], dangerous: [] };
  
  const query = permissionSearchQuery.value.toLowerCase();
  
  if (!query) {
    return {
      normal: apkStore.apkInfo.permissions.filter(p => !p.is_dangerous),
      dangerous: apkStore.apkInfo.permissions.filter(p => p.is_dangerous)
    };
  }
  
  const filteredPerms = apkStore.apkInfo.permissions.filter(p => 
    p.name.toLowerCase().includes(query)
  );
  
  return {
    normal: filteredPerms.filter(p => !p.is_dangerous),
    dangerous: filteredPerms.filter(p => p.is_dangerous)
  };
});

// ==================== 工具函数 ====================
// 权限名称格式化缓存
const permissionNameCache = new Map<string, string>();
function formatPermissionName(name: string): string {
  if (permissionNameCache.has(name)) {
    return permissionNameCache.get(name)!;
  }
  
  const shortName = name.split('.').pop() || name;
  const formattedName = shortName.replace(/([A-Z])/g, ' $1').trim();
  
  permissionNameCache.set(name, formattedName);
  return formattedName;
}

// ==================== 用户交互方法 ====================
// 复制证书指纹
function copyFingerprint(fingerprint?: string) {
  if (!fingerprint) {
    ElMessage.warning('指纹信息不可用');
    return;
  }
  copyToClipboard(fingerprint, '指纹已复制到剪贴板');
}

// 复制分析结果
function copyAnalysisResult() {
  if (!apkStore.apkInfo) return;
  
  const result = `
APK分析结果:
包名: ${apkStore.apkInfo.package_name}
版本: ${apkStore.formattedVersionInfo}
SDK: ${apkStore.formattedSdkInfo}
权限: 共${apkStore.permissionStats?.total || 0}个 (危险权限: ${apkStore.permissionStats?.dangerous || 0}个)
证书: ${certificateStatus.value.label}
  `;
  
  copyToClipboard(result.trim(), '分析结果已复制到剪贴板');
}

// 生成HTML报告
async function generateHtmlReport() {
  if (!apkStore.apkInfo) return;
  
  generatingReport.value = true;
  
  try {
    const info = apkStore.apkInfo;
    const currentFile = apkStore.currentApkFile;
    
    // 构建符合FileInfo接口的对象
    const fileInfo = currentFile ? {
      file_name: currentFile.file_name,
      file_size: currentFile.file_size,
      file_type: info.file_info?.file_type || 'application/vnd.android.package-archive',
      entry_count: info.file_info?.entry_count,
      md5: info.file_info?.md5 || '',
      sha1: info.file_info?.sha1 || '',
      sha256: info.file_info?.sha256 || ''
    } : undefined;
    
    // 使用工具函数生成HTML内容
    const htmlContent = generateReportHTML(info, fileInfo);

    // 创建Blob对象
    const blob = new Blob([htmlContent], { type: 'text/html' });
    const url = URL.createObjectURL(blob);
    
    // 创建下载链接
    const a = document.createElement('a');
    a.href = url;
    a.download = `${info.package_name}_分析报告.html`;
    a.click();
    
    // 释放URL对象
    URL.revokeObjectURL(url);
    ElMessage.success('报告已生成');
  } catch (err) {
    console.error('生成报告失败:', err);
    ElMessage.error('生成报告失败，请查看控制台获取详细信息');
  } finally {
    generatingReport.value = false;
  }
}

// 增加XML查看相关状态
const showManifestDialog = ref(false);
const manifestXml = ref('');
const loadingXml = ref(false);
const xmlError = ref<string | null>(null);

// 优化XML提取逻辑 - 添加更多错误处理和防抖
let xmlExtractionInProgress = false;
// 查看完整AndroidManifest.xml内容
const viewManifestXml = async () => {
  if (!props.apkPath) {
    ElMessage.warning('未找到APK文件路径');
    return;
  }
  
  // 防止重复点击
  if (xmlExtractionInProgress) return;
  
  xmlExtractionInProgress = true;
  loadingXml.value = true;
  xmlError.value = null;
  
  try {
    // 从文件路径获取文件名
    const fileName = props.apkPath.split('/').pop() || '';
    
    // 获取文件的二进制内容
    const response = await fetch(props.apkPath);
    if (!response.ok) {
      throw new Error(`获取APK文件失败: ${response.status} ${response.statusText}`);
    }
    
    const arrayBuffer = await response.arrayBuffer();
    if (!arrayBuffer || arrayBuffer.byteLength === 0) {
      throw new Error('APK文件内容为空');
    }
    
    // 调用Tauri命令提取XML
    manifestXml.value = await invoke('extract_manifest_xml', { 
      fileName: fileName,
      apkData: Array.from(new Uint8Array(arrayBuffer))
    });
    
    if (!manifestXml.value) {
      throw new Error('提取的XML内容为空');
    }
    
    showManifestDialog.value = true;
  } catch (err: any) {
    console.error('提取AndroidManifest.xml失败:', err);
    xmlError.value = err.message || '提取失败，未知错误';
    ElMessage.error(`提取AndroidManifest.xml失败: ${err.message || '未知错误'}`);
  } finally {
    loadingXml.value = false;
    xmlExtractionInProgress = false;
  }
};

// 当组件卸载时清除缓存
onMounted(() => {
  return () => {
    permissionNameCache.clear();
  };
});
</script>

<template>
  <div class="analysis-result">
    <div v-if="!hasApkInfo" class="no-result">
      <el-empty description="暂无解析结果" />
    </div>
    
    <template v-else>
      <!-- 基本信息卡片 -->
      <el-card class="info-card">
        <div class="basic-info">
          <div class="app-identity">
            <div class="app-icon" v-if="fileIcon">
              <img :src="`data:image/png;base64,${fileIcon}`" alt="App Icon">
            </div>
            <div class="app-icon placeholder" v-else>
              <el-icon><Document /></el-icon>
            </div>
            
            <div class="app-details">
              <h2 class="package-name">{{ apkStore.apkInfo?.package_name }}</h2>
              <div class="version-info">{{ apkStore.apkInfo?.formatted_version_info }}</div>
              <div class="sdk-info">{{ apkStore.apkInfo?.formatted_sdk_info }}</div>
              
              <!-- 添加更多基本信息 -->
              <div class="additional-info">
                <div class="file-info" v-if="apkStore.currentApkFile?.file_name">
                  <el-icon><Document /></el-icon>
                  <span>文件名: {{ apkStore.currentApkFile.file_name }}</span>
                </div>
                <div class="file-info" v-if="apkStore.apkInfo?.file_info">
                  <el-icon><Collection /></el-icon>
                  <span>大小: {{ (apkStore.apkInfo?.file_info?.file_size / 1024 / 1024).toFixed(2) }} MB</span>
                </div>
                <div class="file-info" v-if="apkStore.apkInfo?.file_info?.entry_count">
                  <el-icon><List /></el-icon>
                  <span>文件数量: {{ apkStore.apkInfo?.file_info?.entry_count }}</span>
                </div>
                <div class="permission-summary" v-if="apkStore.apkInfo?.permissions">
                  <el-icon><CircleCheck /></el-icon>
                  <span>权限: 共{{ apkStore.apkInfo?.permission_stats.total || 0 }}个 (危险: {{ apkStore.apkInfo?.permission_stats.dangerous || 0 }}个)</span>
                </div>
                <div class="apk-signature" v-if="apkStore.apkInfo?.signature_info">
                  <el-icon><Medal /></el-icon>
                  <span>证书状态: <el-tag size="small" :type="certificateStatus.type">{{ certificateStatus.label }}</el-tag></span>
                </div>
              </div>
            </div>
          </div>
          
          <div class="action-buttons">
            <el-button 
              size="small" 
              type="primary" 
              @click="generateHtmlReport" 
              :loading="generatingReport"
              :disabled="!apkStore.apkInfo"
            >
              <el-icon><Upload /></el-icon>
              生成报告
            </el-button>
            <el-button 
              size="small" 
              type="info" 
              @click="copyAnalysisResult" 
              :disabled="!apkStore.apkInfo"
            >
              <el-icon><CopyDocument /></el-icon>
              复制结果
            </el-button> 
          </div>
        </div>
      </el-card>
      
      <!-- 详细信息标签页 -->
      <el-tabs v-model="activeTab" class="detail-tabs">
        <!-- 安全信息标签页 -->
        <el-tab-pane name="security" label="安全信息">
          <template #label>
            <div class="tab-label">
              <el-icon class="tab-icon"><Lock /></el-icon>
              <span>安全信息</span>
            </div>
          </template>
          
          <template #default>
            <div class="security-info" v-if="apkStore.apkInfo?.signature_info">
              <el-descriptions title="证书信息" :column="1" border>
                <el-descriptions-item label="状态">
                  <el-tag :type="certificateStatus.type">
                    {{ certificateStatus.label }}
                  </el-tag>
                </el-descriptions-item>
                
                <el-descriptions-item label="签名方案">
                  <el-tag type="info">V1 签名</el-tag>
                </el-descriptions-item>
                
                <el-descriptions-item label="发行者">
                  {{ apkStore.apkInfo.signature_info.issuer }}
                </el-descriptions-item>
                
                <el-descriptions-item label="主题">
                  {{ apkStore.apkInfo.signature_info.subject }}
                </el-descriptions-item>
                
                <el-descriptions-item label="有效期">
                  <div class="validity-progress">
                    <div class="date-range">
                      <span>{{ apkStore.apkInfo.signature_info.valid_from }}</span>
                      <span>{{ apkStore.apkInfo.signature_info.valid_to }}</span>
                    </div>
                    <el-progress 
                      :percentage="certificateProgress" 
                      :status="certificateProgressStatus"
                    />
                  </div>
                </el-descriptions-item>
                
                <el-descriptions-item v-if="apkStore.apkInfo.signature_info.fingerprint_sha1" label="SHA-1指纹">
                  <div class="fingerprint">
                    <code>{{ apkStore.apkInfo.signature_info.fingerprint_sha1 }}</code>
                    <el-button 
                      type="primary" 
                      link 
                      icon="CopyDocument"
                      @click="copyFingerprint(apkStore.apkInfo.signature_info.fingerprint_sha1)"
                    >
                      复制
                    </el-button>
                  </div>
                </el-descriptions-item>
                
                <el-descriptions-item v-if="apkStore.apkInfo.signature_info.fingerprint_sha256" label="SHA-256指纹">
                  <div class="fingerprint">
                    <code>{{ apkStore.apkInfo.signature_info.fingerprint_sha256 }}</code>
                    <el-button 
                      type="primary" 
                      link 
                      icon="CopyDocument"
                      @click="copyFingerprint(apkStore.apkInfo.signature_info.fingerprint_sha256)"
                    >
                      复制
                    </el-button>
                  </div>
                </el-descriptions-item>
              </el-descriptions>
            </div>
            
            <el-empty v-else description="无证书信息" />
          </template>
        </el-tab-pane>
        
        <!-- 权限标签页 -->
        <el-tab-pane name="permissions" label="权限信息">
          <template #label>
            <div class="tab-label">
              <el-icon class="tab-icon"><Key /></el-icon>
              <span>权限信息</span>
            </div>
          </template>
          
          <template #default>
            <div class="permissions-info" v-if="apkStore.apkInfo?.permissions?.length">
              <div class="permissions-header">
                <el-alert
                  v-if="filteredPermissions.dangerous.length > 0"
                  type="warning"
                  show-icon
                  :closable="false"
                >
                  <el-icon><Warning /></el-icon>
                  发现 {{ filteredPermissions.dangerous.length }} 个危险权限，这些权限可能会获取用户敏感信息
                </el-alert>
                
                <div class="permissions-search">
                  <el-input
                    v-model="permissionSearchQuery"
                    placeholder="搜索权限..."
                    prefix-icon="Search"
                    clearable
                  />
                </div>
              </div>
              
              <!-- 危险权限表格 -->
              <div v-if="filteredPermissions.dangerous.length > 0" class="permission-section">
                <h3>
                  <el-tag type="danger">危险权限</el-tag>
                  <span class="count">({{ filteredPermissions.dangerous.length }})</span>
                </h3>
                
                <el-table :data="filteredPermissions.dangerous" stripe border>
                  <el-table-column label="权限名称" prop="name" min-width="250">
                    <template #default="{ row }">
                      <el-tooltip 
                        effect="dark" 
                        content="点击查看权限详情" 
                        placement="top"
                      >
                        <div class="permission-name">
                          <span class="full-name">{{ row.name }}</span>
                          <span class="short-name">{{ formatPermissionName(row.name) }}</span>
                        </div>
                      </el-tooltip>
                    </template>
                  </el-table-column>
                  
                  <el-table-column label="风险等级" width="100">
                    <template #default>
                      <el-tag type="danger" size="small">危险</el-tag>
                    </template>
                  </el-table-column>
                  
                  <el-table-column label="说明" width="250">
                    <template #default="{ row }">
                      <el-popover trigger="click" width="300">
                        <template #reference>
                          <el-button link type="primary">查看说明</el-button>
                        </template>
                        
                        <template #default>
                          <h4>{{ formatPermissionName(row.name) }}</h4>
                          <p>该权限允许应用访问用户敏感数据或执行敏感操作，需要明确的用户许可。</p>
                          <p>完整权限名：<code>{{ row.name }}</code></p>
                        </template>
                      </el-popover>
                    </template>
                  </el-table-column>
                </el-table>
              </div>
              
              <!-- 普通权限表格 -->
              <div v-if="filteredPermissions.normal.length > 0" class="permission-section">
                <h3>
                  <el-tag type="info">普通权限</el-tag>
                  <span class="count">({{ filteredPermissions.normal.length }})</span>
                </h3>
                
                <el-table :data="filteredPermissions.normal" stripe border>
                  <el-table-column label="权限名称" prop="name" min-width="250">
                    <template #default="{ row }">
                      <div class="permission-name">
                        <span class="full-name">{{ row.name }}</span>
                        <span class="short-name">{{ formatPermissionName(row.name) }}</span>
                      </div>
                    </template>
                  </el-table-column>
                  
                  <el-table-column label="风险等级" width="100">
                    <template #default>
                      <el-tag type="info" size="small">普通</el-tag>
                    </template>
                  </el-table-column>
                  
                  <el-table-column label="说明" width="250">
                    <template #default="{ row }">
                      <el-popover trigger="click" width="300">
                        <template #reference>
                          <el-button link type="primary">查看说明</el-button>
                        </template>
                        
                        <template #default>
                          <h4>{{ formatPermissionName(row.name) }}</h4>
                          <p>此权限为普通权限，系统会自动授予，无需用户明确许可。</p>
                          <p>完整权限名：<code>{{ row.name }}</code></p>
                        </template>
                      </el-popover>
                    </template>
                  </el-table-column>
                </el-table>
              </div>
              
              <el-empty 
                v-if="!filteredPermissions.dangerous.length && !filteredPermissions.normal.length" 
                description="没有找到匹配的权限" 
              />
            </div>
            
            <el-empty v-else description="未声明权限" />
          </template>
        </el-tab-pane>
        
        <!-- 文件信息标签页 -->
        <el-tab-pane name="file_info" label="文件信息">
          <template #label>
            <div class="tab-label">
              <el-icon class="tab-icon"><Document /></el-icon>
              <span>文件信息</span>
            </div>
          </template>
          
          <template #default>
            <div class="file-info-content" v-if="apkStore.apkInfo?.file_info">
              <el-descriptions title="文件信息" :column="1" border>
                <el-descriptions-item label="文件大小">
                  {{ (apkStore.apkInfo.file_info.file_size / 1024 / 1024).toFixed(2) }} MB
                </el-descriptions-item>
                
                <el-descriptions-item label="文件类型">
                  {{ apkStore.apkInfo.file_info.file_type }}
                </el-descriptions-item>
                
                <el-descriptions-item label="MD5">
                  <div class="hash-value">
                    <code>{{ apkStore.apkInfo.file_info.md5 }}</code>
                    <el-button 
                      type="primary" 
                      link 
                      icon="CopyDocument"
                      @click="copyToClipboard(apkStore.apkInfo.file_info.md5, 'MD5已复制')"
                    >
                      复制
                    </el-button>
                  </div>
                </el-descriptions-item>
                
                <el-descriptions-item label="SHA-1">
                  <div class="hash-value">
                    <code>{{ apkStore.apkInfo.file_info.sha1 }}</code>
                    <el-button 
                      type="primary" 
                      link 
                      icon="CopyDocument"
                      @click="copyToClipboard(apkStore.apkInfo.file_info.sha1, 'SHA-1已复制')"
                    >
                      复制
                    </el-button>
                  </div>
                </el-descriptions-item>
                
                <el-descriptions-item label="SHA-256">
                  <div class="hash-value">
                    <code>{{ apkStore.apkInfo.file_info.sha256 }}</code>
                    <el-button 
                      type="primary" 
                      link 
                      icon="CopyDocument"
                      @click="copyToClipboard(apkStore.apkInfo.file_info.sha256, 'SHA-256已复制')"
                    >
                      复制
                    </el-button>
                  </div>
                </el-descriptions-item>
              </el-descriptions>
            </div>
            
            <el-empty v-else description="无文件信息" />
          </template>
        </el-tab-pane>
      </el-tabs>
      
      <!-- AndroidManifest.xml查看对话框 -->
      <el-dialog
        v-model="showManifestDialog"
        title="AndroidManifest.xml"
        width="80%"
        destroy-on-close
      >
        <div v-if="manifestXml" class="manifest-content">
          <el-button 
            size="small" 
            type="info" 
            @click="copyToClipboard(manifestXml, '已复制XML内容')"
            style="margin-bottom: 10px;"
          >
            <el-icon><CopyDocument /></el-icon>
            复制XML
          </el-button>
          <pre>{{ manifestXml }}</pre>
        </div>
        <div v-else-if="loadingXml" class="loading-xml">
          <el-skeleton :rows="10" animated />
        </div>
        <div v-else class="empty-xml">
          <el-empty :description="xmlError || '未能获取XML内容'" />
        </div>
      </el-dialog>
    </template>
  </div>
</template>

<style lang="scss" scoped>
.analysis-result {
  width: 100%;
  
  .no-result {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 300px;
    background-color: var(--el-bg-color-overlay);
    border-radius: 8px;
    padding: 30px;
  }
  
  .info-card {
    margin-bottom: 20px;
    
    .basic-info {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      
      @media (max-width: 768px) {
        flex-direction: column;
        gap: 20px;
      }
      
      .app-identity {
        display: flex;
        align-items: center;
        gap: 16px;
        
        .app-icon {
          width: 64px;
          height: 64px;
          border-radius: 12px;
          overflow: hidden;
          display: flex;
          align-items: center;
          justify-content: center;
          
          img {
            width: 100%;
            height: 100%;
            object-fit: contain;
          }
          
          &.placeholder {
            background-color: var(--el-fill-color-light);
            color: var(--el-text-color-secondary);
            font-size: 24px;
          }
        }
        
        .app-details {
          .package-name {
            margin: 0 0 8px 0;
            font-size: 18px;
            word-break: break-all;
          }
          
          .version-info, .sdk-info {
            color: var(--el-text-color-secondary);
            margin-bottom: 4px;
            font-size: 14px;
          }
          
          .additional-info {
            margin-top: 12px;
            display: flex;
            flex-direction: column;
            gap: 4px;
            
            .file-info, .permission-summary, .apk-signature {
              display: flex;
              align-items: center;
              gap: 8px;
              color: var(--el-text-color-regular);
              font-size: 14px;
              
              .el-icon {
                color: var(--el-text-color-secondary);
                width: 16px;
                height: 16px;
              }
            }
          }
        }
      }
      
      .action-buttons {
        display: flex;
        gap: 8px;
        
        @media (max-width: 768px) {
          width: 100%;
          justify-content: center;
        }
      }
    }
  }
  
  .detail-tabs {
    .tab-label {
      display: flex;
      align-items: center;
      
      .tab-icon {
        margin-right: 6px;
      }
    }
    
    .security-info, .permissions-info {
      margin-top: 20px;
      
      .validity-progress {
        width: 100%;
        
        .date-range {
          display: flex;
          justify-content: space-between;
          margin-bottom: 8px;
          font-size: 12px;
          color: var(--el-text-color-secondary);
        }
      }
      
      .fingerprint {
        display: flex;
        align-items: center;
        
        code {
          background-color: var(--el-fill-color-light);
          padding: 6px;
          border-radius: 4px;
          font-family: monospace;
          margin-right: 10px;
          font-size: 12px;
          word-break: break-all;
        }
      }
    }
    
    .permissions-info {
      .permissions-header {
        margin-bottom: 20px;
        
        .permissions-search {
          margin-top: 16px;
        }
      }
      
      .permission-section {
        margin-bottom: 30px;
        
        h3 {
          display: flex;
          align-items: center;
          margin-bottom: 16px;
          
          .count {
            margin-left: 8px;
            font-weight: normal;
            color: var(--el-text-color-secondary);
          }
        }
        
        .permission-name {
          display: flex;
          flex-direction: column;
          
          .full-name {
            font-size: 12px;
            color: var(--el-text-color-secondary);
            word-break: break-all;
          }
          
          .short-name {
            font-size: 14px;
            margin-top: 4px;
          }
        }
      }
    }
  }
  
  .manifest-content {
    max-height: 70vh;
    overflow-y: auto;
    
    pre {
      white-space: pre-wrap;
      background-color: var(--el-fill-color-light);
      padding: 16px;
      border-radius: 8px;
      font-family: monospace;
      font-size: 14px;
      line-height: 1.5;
      overflow-x: auto;
    }
  }
  
  .loading-xml {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 20px;
  }
  
  .empty-xml {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 20px;
  }

  .file-info-content {
    margin-top: 20px;
    
    .hash-value {
      display: flex;
      align-items: center;
      gap: 8px;
      
      code {
        background-color: var(--el-fill-color-light);
        padding: 6px;
        border-radius: 4px;
        font-family: monospace;
        font-size: 12px;
        word-break: break-all;
      }
    }
  }
}
</style> 