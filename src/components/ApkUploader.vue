<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { ElMessage, ElProgress  } from 'element-plus';
import { Upload, Loading, Check, Close, Warning } from '@element-plus/icons-vue';
import { useApkStore } from '../stores/apkStore';
import { invoke } from '@tauri-apps/api/core'; 

/**
 * 组件属性定义
 */
const props = defineProps<{
  maxSize?: number; // 最大文件大小（MB）
  acceptedExtensions?: string[]; // 接受的文件扩展名
}>();

/**
 * 事件定义
 */
const emits = defineEmits<{
  (e: 'fileSelected', path: string): void;
  (e: 'uploadSuccess', result: any): void;
  (e: 'uploadError', error: string): void;
  (e: 'reset'): void;
}>();

// 默认值设置
const maxSize = props.maxSize || 100; // 默认最大100MB
const maxSizeBytes = computed(() => maxSize * 1024 * 1024);
const acceptedExtensions = props.acceptedExtensions || ['apk'];

/**
 * 格式化文件大小为人类可读格式
 * @param bytes 文件大小（字节）
 * @returns 格式化后的文件大小字符串
 */
const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  
  return parseFloat((bytes / Math.pow(1024, i)).toFixed(2)) + ' ' + units[i];
};

// 状态管理
const isDragging = ref(false);
const isUploading = ref(false);
const uploadProgress = ref(0);
const uploadStatus = ref<'idle' | 'uploading' | 'success' | 'error'>('idle');
const statusMessage = ref('拖拽APK文件到此处或点击选择');
const selectedFile = ref<File | null>(null);
const fileName = ref('');
const fileSize = ref(0);

// 文件大小格式化
const fileSizeFormatted = computed(() => {
  if (!fileSize.value) return '';
  return formatFileSize(fileSize.value);
});

// 创建取消控制器 - 用于取消正在进行的请求
let abortController: AbortController | null = null;

// 使用apk store
const apkStore = useApkStore();

/**
 * 检查文件是否超过大小限制
 * @param size 文件大小（字节）
 * @returns 是否超过限制
 */
const isFileTooLarge = (size: number): boolean => {
  return size > maxSizeBytes.value;
};

/**
 * 检查文件扩展名是否被接受
 * @param filename 文件名
 * @returns 是否接受该文件类型
 */
const isAcceptedFileType = (filename: string): boolean => {
  const extension = filename.split('.').pop()?.toLowerCase() || '';
  return acceptedExtensions.includes(extension);
};

/**
 * 处理拖拽事件
 */
const handleDragEnter = (e: DragEvent) => {
  e.preventDefault();
  e.stopPropagation();
  if (isUploading.value) return;
  isDragging.value = true;
};

const handleDragOver = (e: DragEvent) => {
  e.preventDefault();
  e.stopPropagation();
  if (isUploading.value) return;
  isDragging.value = true;
};

const handleDragLeave = (e: DragEvent) => {
  e.preventDefault();
  e.stopPropagation();
  isDragging.value = false;
};

/**
 * 处理拖放的文件
 */
const handleDrop = async (e: DragEvent) => {
  e.preventDefault();
  e.stopPropagation();
  isDragging.value = false;
  
  if (isUploading.value) return;
  
  // 获取拖放的文件路径（仅支持本地文件）
  const items = e.dataTransfer?.items;
  if (!items) return;
  
  for (let i = 0; i < items.length; i++) {
    const item = items[i];
    if (item.kind === 'file') {
      const file = item.getAsFile();
      if (file && isAcceptedFileType(file.name)) {
        // 如果是本地文件，我们可以获取其路径
        if ('path' in file) {
          await processApkFile((file as any).path);
          return;
        }
      }
    }
  }
  
  handleError("不支持的操作", "请使用文件选择器选择APK文件");
};

/**
 * 打开文件选择器选择APK文件
 */
const openFileSelector = async () => {
  if (isUploading.value) return;
  
  try {
    console.log('Opening file dialog...');
    // 调用后端命令选择文件
    const filePath = await invoke<string>('select_apk_file');
    
    console.log('Selected file:', filePath);
    
    // 获取文件名
    const pathParts = filePath.split(/[/\\]/);
    fileName.value = pathParts[pathParts.length - 1];
    
    // 直接处理文件
    await processApkFile(filePath);
  } catch (error) {
    console.error("Failed to select file:", error);
    if (error === "No file selected") {
      console.log('No file selected');
      return;
    }
    handleError("选择文件失败", `无法选择文件: ${error}`);
  }
};

 
 

/**
 * 处理APK文件
 * @param filePath APK文件路径
 */
const processApkFile = async (filePath: string) => {
  try {
    console.log('Processing APK file:', filePath);
    uploadStatus.value = 'uploading';
    isUploading.value = true;
    statusMessage.value = '正在解析APK文件...';
    
    // 直接使用文件路径解析APK
    const result = await apkStore.parseApk(filePath);
    
    uploadStatus.value = 'success';
    statusMessage.value = '解析成功';
    emits('uploadSuccess', result);
    emits('fileSelected', filePath);
  } catch (error) {
    console.error('Failed to process APK:', error);
    handleError("处理文件失败", error as string);
  } finally {
    isUploading.value = false;
  }
};

/**
 * 处理错误
 * @param title 错误标题
 * @param error 错误详情
 */
const handleError = (title: string, error: any) => {
  resetProgress();
  console.error(`${title}:`, error);
  
  uploadStatus.value = 'error';
  statusMessage.value = `${title}: ${error}`;
  
  ElMessage.error({
    message: String(error),
    duration: 5000,
  });
};

/**
 * 重置上传进度
 */
const resetProgress = () => {
  uploadProgress.value = 0;
  uploadStatus.value = 'idle';
  
  // 如果有一个活动的请求，取消它
  if (abortController) {
    abortController.abort();
    abortController = null;
  }
};

/**
 * 清除当前上传
 */
const clearUpload = () => {
  resetProgress();
  fileName.value = '';
  fileSize.value = 0;
  selectedFile.value = null;
  statusMessage.value = '拖拽APK文件到此处或点击选择';
  
  emits('reset');
};

/**
 * 取消上传
 */
const cancelUpload = () => {
  if (isUploading.value) {
    resetProgress();
    isUploading.value = false;
    statusMessage.value = '上传已取消';
    emits('reset');
  }
};

// 组件卸载前清理
onBeforeUnmount(() => {
  if (abortController) {
    abortController.abort();
  }
});

// 组件初始化
onMounted(() => {
  // 任何初始化代码
});
</script>

<template>
  <div 
    class="apk-uploader"
    :class="{
      'is-dragging': isDragging,
      'is-uploading': isUploading,
      'is-success': uploadStatus === 'success',
      'is-error': uploadStatus === 'error'
    }"
    @dragenter="handleDragEnter"
    @dragover="handleDragOver"
    @dragleave="handleDragLeave"
    @drop="handleDrop"
  >
    <!-- 上传区域 -->
    <div class="uploader-content">
      <!-- 文件未选择状态 -->
      <div v-if="!fileName && uploadStatus === 'idle'" class="uploader-empty">
        <el-icon class="uploader-icon"><Upload /></el-icon>
        <div class="uploader-text">{{ statusMessage }}</div>
        <el-button type="primary" @click="openFileSelector" :disabled="isUploading">
          选择APK文件
        </el-button>
      </div>
      
      <!-- 上传/处理中状态 -->
      <div v-else-if="isUploading" class="uploader-loading">
        <el-icon class="uploader-icon spinning"><Loading /></el-icon>
        <div class="uploader-text">{{ statusMessage }}</div>
        
        <el-button type="danger" @click="cancelUpload" size="small">
          取消
        </el-button>
      </div>
      
      <!-- 成功状态 -->
      <div v-else-if="uploadStatus === 'success'" class="uploader-success">
        <el-icon class="uploader-icon success"><Check /></el-icon>
        <div class="uploader-text">{{ statusMessage }}</div>
        <div v-if="fileName" class="file-info">
          <div class="file-name">{{ fileName }}</div>
          <div v-if="fileSize" class="file-size">{{ fileSizeFormatted }}</div>
        </div>
        <div class="action-buttons">
          <el-button type="primary" @click="openFileSelector" size="small">
            选择其他文件
          </el-button>
          <el-button type="default" @click="clearUpload" size="small">
            清除
          </el-button>
        </div>
      </div>
      
      <!-- 错误状态 -->
      <div v-else-if="uploadStatus === 'error'" class="uploader-error">
        <el-icon class="uploader-icon error"><Close /></el-icon>
        <div class="uploader-text">{{ statusMessage }}</div>
        <el-button type="primary" @click="openFileSelector">
          重试
        </el-button>
      </div>
      
      <!-- 文件已选但未上传状态 -->
      <div v-else class="uploader-ready">
        <el-icon class="uploader-icon ready"><Warning /></el-icon>
        <div class="uploader-text">已选择文件</div>
        <div class="file-info">
          <div class="file-name">{{ fileName }}</div>
          <div v-if="fileSize" class="file-size">{{ fileSizeFormatted }}</div>
        </div>
        <div class="action-buttons">
          <el-button type="primary" @click="processApkFile" :disabled="isUploading">
            解析APK
          </el-button>
          <el-button type="default" @click="clearUpload" :disabled="isUploading">
            清除
          </el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.apk-uploader {
  width: 100%;
  border: 2px dashed var(--el-border-color);
  border-radius: 8px;
  padding: 30px;
  text-align: center;
  transition: all 0.3s;
  background-color: var(--el-fill-color-lighter);
  
  &.is-dragging {
    border-color: var(--el-color-primary);
    background-color: var(--el-color-primary-light-9);
  }
  
  &.is-success {
    border-color: var(--el-color-success);
    background-color: var(--el-color-success-light-9);
  }
  
  &.is-error {
    border-color: var(--el-color-danger);
    background-color: var(--el-color-danger-light-9);
  }
  
  .uploader-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 180px;
  }
  
  .uploader-icon {
    font-size: 40px;
    color: var(--el-text-color-secondary);
    margin-bottom: 16px;
    
    &.spinning {
      animation: rotate 1.5s linear infinite;
    }
    
    &.success {
      color: var(--el-color-success);
    }
    
    &.error {
      color: var(--el-color-danger);
    }
    
    &.ready {
      color: var(--el-color-warning);
    }
  }
  
  .uploader-text {
    font-size: 16px;
    color: var(--el-text-color-regular);
    margin-bottom: 16px;
  }
  
  .uploader-progress {
    width: 80%;
    max-width: 300px;
    margin: 16px 0;
  }
  
  .file-info {
    margin: 16px 0;
    
    .file-name {
      font-weight: 500;
      word-break: break-all;
    }
    
    .file-size {
      color: var(--el-text-color-secondary);
      font-size: 14px;
      margin-top: 4px;
    }
  }
  
  .action-buttons {
    display: flex;
    gap: 8px;
    margin-top: 16px;
  }
}

@keyframes rotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style> 