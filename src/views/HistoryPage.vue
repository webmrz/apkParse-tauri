<template>
  <div class="history-page" :class="{ 'page-loaded': pageLoaded, 'dark-mode': isDarkMode }">
    <div class="background-decoration"></div>
    
    <h1 class="page-title">
      解析<span class="highlight">历史记录</span>
      <div class="title-decoration"></div>
    </h1>
    
    <div class="page-content">
      <div class="history-section glass-effect">
        <div class="section-decoration"></div>
        <div class="section-header">
          <h2 class="section-title">历史记录 ({{ historyItems.length }})</h2>
          <div class="header-actions">
            <el-button 
              type="danger" 
              size="small" 
              @click="clearAllHistory" 
              :disabled="historyItems.length === 0"
            >
              <el-icon><Delete /></el-icon>
              清空历史
            </el-button>
          </div>
        </div>

        <div v-if="historyItems.length === 0" class="empty-state">
          <el-empty description="暂无解析历史记录"></el-empty>
        </div>

        <div v-else class="history-list">
          <el-table :data="historyItems" style="width: 100%" v-loading="loading">
            <el-table-column label="应用" min-width="250">
              <template #default="{ row }">
                <div class="app-info">
                  <div class="app-icon" v-if="row.apk_info.icon_base64">
                    <el-image 
                      :src="'data:image/png;base64,' + row.apk_info.icon_base64" 
                      alt="App icon"
                      :preview-src-list="['data:image/png;base64,' + row.apk_info.icon_base64]"
                      :initial-index="0"
                      fit="contain"
                      hide-on-click-modal
                      preview-teleported
                    />
                  </div>
                  <div class="app-icon placeholder" v-else>
                    <el-icon><Box /></el-icon>
                  </div>
                  <div class="app-details">
                    <div class="app-name">{{ row.apk_info.package_name }}</div>
                    <div class="app-version">{{ row.apk_info.version_name }}</div>
                  </div>
                </div>
              </template>
            </el-table-column>
            
            <el-table-column label="分析时间" width="180">
              <template #default="{ row }">
                {{ formatDate(row.analyzed_at) }}
              </template>
            </el-table-column>
            
            <el-table-column label="权限" width="180">
              <template #default="{ row }">
                <template v-if="row.apk_info?.permission_stats">
                  <el-tag type="info">总数: {{ row.apk_info.permission_stats.total || 0 }}</el-tag>
                  <el-tag 
                    type="danger" 
                    v-if="row.apk_info.permission_stats.dangerous > 0" 
                    class="ml-2"
                  >
                    危险: {{ row.apk_info.permission_stats.dangerous }}
                  </el-tag>
                </template>
                <template v-else>
                  <el-tag type="info">数据加载中...</el-tag>
                </template>
              </template>
            </el-table-column>
            
            <el-table-column label="操作" width="180" fixed="right">
              <template #default="{ row }">
                <el-button 
                  type="primary" 
                  size="small" 
                  @click="loadHistoryItem(row.id)"
                >
                  查看详情
                </el-button>
                <el-button 
                  type="danger" 
                  size="small" 
                  @click="removeHistoryItem(row.id)"
                >
                  <el-icon><Delete /></el-icon>
                </el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </div>
    </div>

    <!-- APK详情对话框 -->
    <el-dialog
      v-model="dialogVisible"
      title="APK详情"
      width="80%"
      :destroy-on-close="true"
      class="apk-detail-dialog"
      top="5vh"
    >
      <div class="dialog-content">
        <div v-if="currentApk" class="apk-detail-wrapper" v-loading="loading">
          <!-- 基本信息卡片 -->
          <el-card class="detail-card">
            <template #header>
              <div class="card-header">
                <div class="app-header-info">
                  <div class="app-icon large" v-if="currentApk.icon_base64">
                    <el-image 
                      :src="'data:image/png;base64,' + currentApk.icon_base64" 
                      alt="App icon"
                      :preview-src-list="['data:image/png;base64,' + currentApk.icon_base64]"
                      :initial-index="0"
                      fit="contain"
                      hide-on-click-modal
                      preview-teleported
                    />
                  </div>
                  <div class="app-icon large placeholder" v-else>
                    <el-icon><Box /></el-icon>
                  </div>
                  <div class="app-details">
                    <h3 class="app-name">{{ currentApk.package_name }}</h3>
                    <div class="app-meta">
                      <span class="version-label">版本: {{ currentApk.version_name }} ({{ currentApk.version_code }})</span>
                    </div>
                  </div>
                </div>
              </div>
            </template>
            
            <div class="info-section">
              <div class="info-item">
                <span class="info-label">SDK要求:</span>
                <span class="info-value">{{ currentApk.formatted_sdk_info }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">签名状态:</span>
                <span class="info-value">
                  <el-tag :type="currentApk.is_certificate_expired ? 'danger' : 'success'">
                    {{ currentApk.is_certificate_expired ? '已过期' : '有效' }}
                  </el-tag>
                </span>
              </div>
            </div>
          </el-card>

          <!-- 权限信息卡片 -->
          <el-card class="detail-card">
            <template #header>
              <div class="card-header">
                <h3>权限信息</h3>
                <div class="permission-stats">
                  <el-tag type="info">总数: {{ currentApk.permission_stats.total }}</el-tag>
                  <el-tag type="danger" v-if="currentApk.permission_stats.dangerous > 0" class="ml-2">
                    危险: {{ currentApk.permission_stats.dangerous }}
                  </el-tag>
                </div>
              </div>
            </template>
            
            <div v-if="currentApk.dangerous_permissions && currentApk.dangerous_permissions.length > 0">
              <h4 class="permission-title">危险权限</h4>
              <div class="permissions-wrapper">
                <el-tag 
                  v-for="(perm, index) in currentApk.dangerous_permissions" 
                  :key="'danger-'+index"
                  type="danger"
                  class="permission-tag"
                >
                  {{ perm.name }}
                </el-tag>
              </div>
            </div>
            
            <div v-if="currentApk.permissions && currentApk.permissions.length > 0">
              <h4 class="permission-title">所有权限</h4>
              <div class="permissions-wrapper">
                <el-tag 
                  v-for="(perm, index) in currentApk.permissions" 
                  :key="'all-'+index"
                  :type="perm.is_dangerous ? 'danger' : 'info'"
                  class="permission-tag"
                >
                  {{ perm.name }}
                </el-tag>
              </div>
            </div>
            
            <el-empty v-if="!currentApk.permissions || currentApk.permissions.length === 0" description="无权限信息"></el-empty>
          </el-card>

          <!-- 证书信息卡片 -->
          <el-card class="detail-card" v-if="currentApk.signature_info">
            <template #header>
              <div class="card-header">
                <h3>签名证书信息</h3>
              </div>
            </template>
            
            <div class="cert-info">
              <div class="info-item">
                <span class="info-label">发行者:</span>
                <span class="info-value">{{ currentApk.signature_info.issuer }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">主体:</span>
                <span class="info-value">{{ currentApk.signature_info.subject }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">有效期:</span>
                <span class="info-value cert-validity">
                  <div>
                    <span class="date-label">开始:</span> {{ formatDate(currentApk.signature_info.valid_from) }}
                  </div>
                  <div>
                    <span class="date-label">结束:</span> {{ formatDate(currentApk.signature_info.valid_to) }}
                  </div>
                  <div class="cert-status mt-2">
                    <el-tag :type="currentApk.is_certificate_expired ? 'danger' : 'success'" size="small">
                      {{ currentApk.is_certificate_expired ? '已过期' : '有效' }}
                    </el-tag>
                  </div>
                </span>
              </div>
              <div class="info-item" v-if="currentApk.signature_info.fingerprint_sha1">
                <span class="info-label">SHA1指纹:</span>
                <span class="info-value code">{{ currentApk.signature_info.fingerprint_sha1 }}</span>
              </div>
              <div class="info-item" v-if="currentApk.signature_info.fingerprint_sha256">
                <span class="info-label">SHA256指纹:</span>
                <span class="info-value code">{{ currentApk.signature_info.fingerprint_sha256 }}</span>
              </div>
            </div>
          </el-card>
        </div>
        
        <el-empty v-else description="无法加载APK信息"></el-empty>
      </div>
      <template #footer>
        <span class="dialog-footer">
          <!-- <el-button 
            v-if="currentApk" 
            type="primary" 
            @click="navigateToHome" 
            size="small"
          >
            在主页查看详细分析
          </el-button> -->
          <el-button @click="dialogVisible = false" size="small">关闭</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRouter } from 'vue-router';
import { ElMessageBox, ElMessage } from 'element-plus';
import { Delete, Box } from '@element-plus/icons-vue';
import { useApkStore, ApkInfo } from '../stores/apkStore';
import { themeManager } from '../utils/theme';

const router = useRouter();
const apkStore = useApkStore();
const pageLoaded = ref(false);
const loading = ref(false);
const dialogVisible = ref(false);
const currentApk = ref<ApkInfo | null>(null);

// 使用主题管理器
const isDarkMode = computed(() => themeManager.theme.value === 'dark');
const historyItems = computed(() => apkStore.historyItems);

onMounted(() => {
  setTimeout(() => {
    pageLoaded.value = true;
  }, 300);
});

// 加载历史记录项并在对话框中显示
const loadHistoryItem = (id: string) => {
  loading.value = true;
  try {
    // 查找历史记录项
    const item = apkStore.historyItems.find(item => item.id === id);
    if (!item) {
      ElMessage.error('找不到该历史记录项');
      loading.value = false;
      return;
    }
    
    // 设置当前APK信息并显示对话框
    currentApk.value = {...item.apk_info};
    dialogVisible.value = true;
    
    // 同时加载到apkStore中，以便后续可以在主页查看
    apkStore.loadFromHistory(id);
  } catch (error) {
    ElMessage.error('加载历史记录失败: ' + error);
  } finally {
    loading.value = false;
  }
};

// 导航到主页查看详细分析
const navigateToHome = () => {
  dialogVisible.value = false;
  router.push('/');
};

// 删除历史记录项
const removeHistoryItem = (id: string) => {
  ElMessageBox.confirm('确定要删除此历史记录吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(() => {
    apkStore.removeFromHistory(id);
    ElMessage.success('删除成功');
  }).catch(() => {});
};

// 清空所有历史记录
const clearAllHistory = () => {
  if (historyItems.value.length === 0) return;
  
  ElMessageBox.confirm('确定要清空所有历史记录吗？此操作不可撤销！', '警告', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(() => {
    apkStore.clearHistory();
    ElMessage.success('历史记录已清空');
  }).catch(() => {});
};

// 格式化日期
const formatDate = (dateString: string) => {
  if (!dateString) return '未知';
  
  try {
    // 尝试直接解析
    const date = new Date(dateString);
    
    // 检查日期是否有效（无效日期的getTime()会返回NaN）
    if (isNaN(date.getTime())) {
      // 可能是特殊格式的日期字符串，尝试不同的解析方式
      // 例如：检查是否是"Mar 23 20:57:34 2023 GMT"这样的格式
      if (dateString.includes('GMT')) {
        // 不进行额外处理，直接返回原始字符串
        return dateString;
      }
      return '无效日期';
    }
    
    return new Intl.DateTimeFormat('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    }).format(date);
  } catch (error) {
    console.error('日期格式化错误:', error, dateString);
    return '无效日期';
  }
};

// 监听历史记录变化，如果删除了当前正在显示的记录，显示提示
watch(() => apkStore.historyItems, (newItems, oldItems) => {
  if (oldItems && oldItems.length > newItems.length) {
    // 历史记录减少了，可能是删除了当前记录
    ElMessage.info('历史记录已更新');
  }
}, { deep: true });

// 监听对话框关闭
watch(dialogVisible, (visible) => {
  if (!visible) {
    currentApk.value = null;
  }
});
</script>

<style lang="scss" scoped>
.history-page {
  width: 100%;
  min-height: 100vh;
  margin: 0 auto;
  padding: clamp(16px, 3vw, 40px);
  opacity: 0;
  transform: translateY(20px);
  transition: all 0.6s ease;
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
  color: var(--el-text-color-regular);
  
  &.dark-mode {
    --gradient-start: var(--glass-bg-dark);
    --gradient-end: var(--glass-bg-dark-end);
    --glass-border-color: var(--glass-border-dark);
    --glass-shadow: var(--glass-shadow-dark);
    --decoration-color: rgba(56, 189, 248, 0.1);
    
    .background-decoration {
      opacity: var(--decoration-opacity);
      background: 
        radial-gradient(circle at 0% 0%, var(--el-color-primary) 0%, transparent 50%),
        radial-gradient(circle at 100% 100%, var(--el-color-success) 0%, transparent 50%);
    }

    .glass-effect { 
      border-color: var(--glass-border-color);
      box-shadow: var(--glass-shadow);
    }
  }
  
  .background-decoration {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 0% 0%, var(--el-color-primary-light-7) 0%, transparent 50%),
      radial-gradient(circle at 100% 100%, var(--el-color-success-light-5) 0%, transparent 50%);
    opacity: 0.1;
    z-index: -1;
    pointer-events: none;
    transition: all 0.6s ease;
  }
  
  &.page-loaded {
    opacity: 1;
    transform: translateY(0);
    
    .title-decoration {
      transform: scaleX(1);
    }
  }
  
  .page-title {
    text-align: center;
    margin-bottom: clamp(20px, 4vw, 40px);
    color: var(--el-text-color-primary);
    font-size: clamp(1.5rem, 4vw, 2.5rem);
    position: relative;
    font-weight: 600;
    
    .highlight {
      background: linear-gradient(120deg, var(--el-color-primary-light-5), var(--el-color-primary));
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }
    
    .title-decoration {
      position: absolute;
      bottom: -8px;
      left: 30%;
      right: 30%;
      height: 3px;
      background: linear-gradient(90deg, 
        transparent, 
        var(--el-color-primary),
        transparent
      );
      border-radius: 3px;
      transform: scaleX(0);
      transform-origin: center;
      transition: transform 0.8s cubic-bezier(0.17, 0.67, 0.47, 1.34) 0.2s;
    }
  }
  
  .history-section {
    background: var(--glass-bg-light, rgba(255, 255, 255, 0.95));
    backdrop-filter: blur(15px);
    border-radius: 12px;
    border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.3));
    box-shadow: var(--glass-shadow, 0 8px 32px rgba(0, 0, 0, 0.05));
    padding: 20px;
    position: relative;
    overflow: hidden;
    
    .section-decoration {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 4px;
      background: linear-gradient(90deg, 
        var(--el-color-primary),
        var(--el-color-success)
      );
      opacity: 0.7;
    }
    
    .section-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 20px;
      
      .section-title {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 500;
        color: var(--el-text-color-primary);
      }
    }
    
    .empty-state {
      padding: 40px 0;
    }
    
    .history-list {
      .app-info {
        display: flex;
        align-items: center;
        gap: 12px;
        
        .app-icon {
          width: 40px;
          height: 40px;
          border-radius: 8px;
          overflow: hidden;
          display: flex;
          align-items: center;
          justify-content: center;
          background-color: var(--el-fill-color-light);
          cursor: pointer;
          
          :deep(.el-image) {
            width: 100%;
            height: 100%;
            display: flex;
            align-items: center;
            justify-content: center;
          }
          
          :deep(img) {
            width: 100%;
            height: 100%;
            object-fit: contain;
          }
          
          &.placeholder {
            background-color: var(--el-color-primary-light-8);
            color: var(--el-color-primary);
            font-size: 24px;
            cursor: default;
          }
          
          &.large {
            width: 60px;
            height: 60px;
            border-radius: 12px;
          }
        }
        
        .app-details {
          display: flex;
          flex-direction: column;
          
          .app-name {
            font-weight: 500;
            color: var(--el-text-color-primary);
            margin-bottom: 4px;
            font-size: 0.9rem;
          }
          
          .app-version {
            color: var(--el-text-color-secondary);
            font-size: 0.8rem;
          }
        }
      }
    }
  }
  
  .glass-effect {
    background: var(--gradient-start, var(--glass-bg-light));
    backdrop-filter: blur(15px);
    -webkit-backdrop-filter: blur(15px);
    border-radius: 12px;
    border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.3));
    box-shadow: var(--glass-shadow, 0 8px 32px rgba(0, 0, 0, 0.05));
    transition: all 0.3s ease;
  }
}

.ml-2 {
  margin-left: 8px;
}

// 新增对话框相关样式
.apk-detail-dialog {
  :deep(.el-dialog) {
    display: flex;
    flex-direction: column;
    max-height: 90vh;
    margin-top: 5vh !important;
    margin-bottom: 5vh !important;
  }

  :deep(.el-dialog__header) {
    border-bottom: 1px solid var(--el-border-color-light);
    margin-right: 0;
    padding-top: 15px;
    padding-bottom: 15px;
    flex-shrink: 0;
  }
  
  :deep(.el-dialog__body) {
    padding: 20px;
    overflow-y: auto;
    flex: 1;
    max-height: calc(90vh - 130px); /* 减去头部和底部的高度 */
  }
  
  :deep(.el-dialog__footer) {
    border-top: 1px solid var(--el-border-color-light);
    padding-top: 12px;
    padding-bottom: 12px;
    flex-shrink: 0;
  }
  
  .dialog-content {
    min-height: 200px;
  }
  
  .dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
  }
  
  .apk-detail-wrapper {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 20px;
    padding-bottom: 10px;
  }
  
  .detail-card {
    height: 100%;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    
    :deep(.el-card__header) {
      padding: 15px;
      flex-shrink: 0;
    }
    
    :deep(.el-card__body) {
      flex: 1;
      overflow-y: auto;
      padding-bottom: 16px;
    }
    
    .card-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      
      h3 {
        margin: 0;
        font-size: 1rem;
        font-weight: 500;
      }
    }
    
    .app-header-info {
      display: flex;
      align-items: center;
      gap: 15px;
    }
    
    .app-icon.large {
      width: 60px;
      height: 60px;
      border-radius: 12px;
    }
    
    .app-details {
      display: flex;
      flex-direction: column;
      
      .app-name {
        margin: 0 0 5px;
        font-size: 1.1rem;
      }
      
      .app-meta {
        font-size: 0.9rem;
        color: var(--el-text-color-secondary);
      }
    }
    
    .info-section {
      display: flex;
      flex-direction: column;
      gap: 10px;
    }
    
    .info-item {
      display: flex;
      flex-direction: column;
      gap: 5px;
      
      .info-label {
        font-weight: 500;
        color: var(--el-text-color-secondary);
        font-size: 0.9rem;
      }
      
      .info-value {
        &.code {
          font-family: monospace;
          font-size: 0.85rem;
          word-break: break-all;
          background: var(--el-fill-color-light);
          padding: 5px;
          border-radius: 4px;
        }
        
        &.cert-validity {
          display: flex;
          flex-direction: column;
          gap: 4px;
          
          .date-label {
            display: inline-block;
            width: 50px;
            font-weight: 500;
            color: var(--el-text-color-secondary);
          }
          
          .cert-status {
            margin-top: 6px;
          }
        }
      }
    }
    
    .permission-title {
      margin: 10px 0;
      font-size: 0.95rem;
      color: var(--el-text-color-secondary);
    }
    
    .permission-tag {
      margin: 0 8px 8px 0;
      max-width: 100%;
      overflow: hidden;
      text-overflow: ellipsis;
      display: inline-block;
    }
    
    .cert-info {
      display: flex;
      flex-direction: column;
      gap: 12px;
    }
  }
}

.ml-2 {
  margin-left: 8px;
}

.mt-2 {
  margin-top: 8px;
}

/* 针对不同权限数量的情况优化展示 */
.permissions-wrapper {
  max-height: 300px;
  overflow-y: auto;
  padding-right: 5px;
}

/* 优化小屏幕下的卡片展示 */
@media (max-width: 767px) {
  .apk-detail-wrapper {
    grid-template-columns: 1fr;
  }
  
  .permissions-wrapper {
    max-height: 200px;
  }
}

/* Style for image preview */
:deep(.el-image-viewer__wrapper) {
  z-index: 3000;
}

:deep(.el-image-viewer__img) {
  max-width: 80vw;
  max-height: 80vh;
}
</style> 