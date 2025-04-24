<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { ElMessage } from 'element-plus';
import { Download, Tools, InfoFilled, Warning, Setting } from '@element-plus/icons-vue';

const aapt2Guide = ref<string>('');
const isAapt2Available = ref<boolean>(false);
const isCheckingAapt2 = ref<boolean>(true);
const isShowingGuide = ref<boolean>(false);
const diagnosticResult = ref<string>('');
const isDiagnosing = ref<boolean>(false);

// 检查aapt2是否可用
const checkAapt2Availability = async () => {
  isCheckingAapt2.value = true;
  try {
    const available = await invoke('is_aapt2_available');
    isAapt2Available.value = !!available;
  } catch (error) {
    console.error('检查aapt2可用性时出错:', error);
    isAapt2Available.value = false;
  } finally {
    isCheckingAapt2.value = false;
  }
};

// 获取aapt2下载指南
const getAapt2Guide = async () => {
  try {
    const guide = await invoke('get_aapt2_download_guide');
    if (typeof guide === 'string') {
      aapt2Guide.value = guide;
      isShowingGuide.value = true;
    }
  } catch (error) {
    console.error('获取aapt2指南时出错:', error);
    ElMessage.error('无法获取aapt2指南');
  }
};

// 打开下载指南
const showDownloadGuide = () => {
  if (aapt2Guide.value) {
    isShowingGuide.value = true;
  } else {
    getAapt2Guide();
  }
};

// 隐藏下载指南
const hideDownloadGuide = () => {
  isShowingGuide.value = false;
};

// 执行aapt2诊断测试
const runAapt2Diagnostic = async () => {
  isDiagnosing.value = true;
  diagnosticResult.value = '正在执行aapt2诊断测试...';
  
  try {
    const result = await invoke('test_aapt2');
    if (typeof result === 'string') {
      diagnosticResult.value = result;
    } else {
      diagnosticResult.value = '无法获取诊断结果';
    }
  } catch (error) {
    console.error('执行aapt2诊断测试出错:', error);
    diagnosticResult.value = `诊断测试失败: ${error}`;
  } finally {
    isDiagnosing.value = false;
  }
};

onMounted(() => {
  checkAapt2Availability();
  getAapt2Guide();
});
</script>

<template>
  <div class="aapt2-helper">
    <el-card v-if="!isAapt2Available && !isCheckingAapt2" shadow="hover" class="warning-card">
      <template #header>
        <div class="card-header">
          <el-icon><Warning /></el-icon>
          <span>aapt2.exe 未找到或不可用</span>
        </div>
      </template>
      
      <el-alert
        type="warning"
        :closable="false"
        show-icon
      >
        <p>aapt2.exe是解析APK文件的核心工具，但当前系统中未找到有效的aapt2.exe。</p>
        <p>没有此工具，APK分析功能将受到限制。</p>
      </el-alert>
      
      <div class="action-buttons">
        <el-button type="primary" @click="showDownloadGuide">
          <el-icon><Download /></el-icon>查看下载指南
        </el-button>
        <el-button type="info" @click="runAapt2Diagnostic" :loading="isDiagnosing">
          <el-icon><Setting /></el-icon>执行诊断测试
        </el-button>
      </div>
      
      <!-- 诊断结果显示区 -->
      <div v-if="diagnosticResult" class="diagnostic-result">
        <el-divider>诊断结果</el-divider>
        <pre class="result-text">{{ diagnosticResult }}</pre>
      </div>
    </el-card>
    
    <el-card v-if="isShowingGuide" shadow="hover" class="guide-card">
      <template #header>
        <div class="card-header">
          <el-icon><InfoFilled /></el-icon>
          <span>aapt2.exe 下载与安装指南</span>
          <el-button class="close-button" text @click="hideDownloadGuide">关闭</el-button>
        </div>
      </template>
      
      <div class="markdown-content" v-html="aapt2Guide.replace(/\n/g, '<br>').replace(/^## (.*)/gm, '<h3>$1</h3>').replace(/^# (.*)/gm, '<h2>$1</h2>')" />
      
      <el-divider />
      
      <h3>快速安装步骤</h3>
      <el-steps :active="1" simple>
        <el-step title="第1步" description="下载aapt2.exe" />
        <el-step title="第2步" description="将文件复制到tools目录" />
        <el-step title="第3步" description="重启应用程序" />
      </el-steps>
      
      <div class="notes">
        <p><strong>注意:</strong> 在tools目录中还有一个帮助脚本 download_aapt2.ps1，可以帮助您下载aapt2.exe。</p>
        <p>打开PowerShell，切换到tools目录，然后运行 ./download_aapt2.ps1</p>
      </div>
      
      <!-- 添加诊断测试按钮 -->
      <div class="diagnostic-section">
        <el-divider>问题诊断</el-divider>
        <p>如果您已经放置了aapt2.exe但系统仍然无法识别，请运行诊断测试以获取更多信息：</p>
        <el-button type="primary" @click="runAapt2Diagnostic" :loading="isDiagnosing">
          <el-icon><Setting /></el-icon>执行aapt2诊断测试
        </el-button>
        
        <!-- 诊断结果显示区 -->
        <div v-if="diagnosticResult" class="diagnostic-result">
          <el-divider>诊断结果</el-divider>
          <pre class="result-text">{{ diagnosticResult }}</pre>
        </div>
      </div>
    </el-card>
  </div>
</template>

<style lang="scss" scoped>
.aapt2-helper {
  margin-bottom: 20px;
  
  .warning-card, .guide-card {
    margin-bottom: 16px;
    background-color: var(--el-bg-color);
    
    :deep(.el-card__header) {
      border-bottom: 1px solid var(--el-border-color-light);
    }
  }
  
  .card-header {
    display: flex;
    align-items: center;
    color: var(--el-text-color-primary);
    
    .el-icon {
      margin-right: 8px;
      color: var(--el-color-warning);
    }
    
    .close-button {
      margin-left: auto;
      color: var(--el-text-color-regular);
      
      &:hover {
        color: var(--el-text-color-primary);
      }
    }
  }
  
  .action-buttons {
    margin-top: 16px;
    display: flex;
    justify-content: center;
    gap: 10px;
  }
  
  .markdown-content {
    line-height: 1.6;
    color: var(--el-text-color-primary);
    
    h2 {
      font-size: 1.5em;
      margin: 16px 0 8px 0;
      border-bottom: 1px solid var(--el-border-color-light);
      padding-bottom: 8px;
    }
    
    h3 {
      font-size: 1.2em;
      margin: 16px 0 8px 0;
    }
    
    p {
      margin: 8px 0;
    }
    
    ul, ol {
      margin: 8px 0;
      padding-left: 24px;
    }
    
    pre {
      background-color: var(--el-fill-color-light);
      border-radius: 3px;
      padding: 10px;
      overflow-x: auto;
    }
  }
  
  .diagnostic-section {
    margin-top: 20px;
  }
  
  .diagnostic-result {
    margin-top: 16px;
    
    .result-text {
      background-color: var(--el-fill-color-light);
      border-radius: 3px;
      padding: 10px;
      white-space: pre-wrap;
      max-height: 300px;
      overflow-y: auto;
      font-family: monospace;
      font-size: 0.9em;
      color: var(--el-text-color-primary);
    }
  }
  
  .notes {
    background-color: var(--el-fill-color-light);
    border-left: 4px solid var(--el-color-success);
    padding: 12px;
    margin: 16px 0;
    border-radius: 0 3px 3px 0;
    color: var(--el-text-color-primary);
    
    p {
      margin: 8px 0;
    }
  }
}
</style> 