<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import ApkUploader from '../components/ApkUploader.vue';
import AnalysisResult from '../components/AnalysisResult.vue'; 
import { ElButton, ElSkeleton } from 'element-plus';
import { themeManager } from '../utils/theme';
 
const showResult = ref(false);
const isLoading = ref(false);
const pageLoaded = ref(false);

// 使用主题管理器
const isDarkMode = computed(() => themeManager.theme.value === 'dark');

onMounted(() => {
  setTimeout(() => {
    pageLoaded.value = true;
  }, 300);
});

// 处理上传成功
const handleUploadSuccess = (_result: any) => {
  isLoading.value = true;
  setTimeout(() => {
    isLoading.value = false;
    showResult.value = true;
  }, 800); // 添加短暂延迟以显示加载动画
};

// 处理上传错误
const handleUploadError = (_error: string) => {
  isLoading.value = false;
  showResult.value = false;
};

// 重置
const handleReset = () => {
  showResult.value = false;
};

// 返回上传页面
const backToUpload = () => {
  showResult.value = false;
};
</script>

<template>
  <div class="home-page" :class="{ 'page-loaded': pageLoaded, 'dark-mode': isDarkMode }">
    <div class="background-decoration"></div>
    
    <h1 class="page-title">
      <span class="highlight">APK</span> 分析工具
      <div class="title-decoration"></div>
    </h1>
    
    <div class="page-content">
      <Transition name="fade-slide" mode="out-in">
        <!-- 当没有解析结果时显示上传器 -->
        <div class="upload-section glass-effect" v-if="!showResult && !isLoading" key="upload">
          <div class="section-decoration"></div>
          <ApkUploader 
            :max-size="100" 
            @upload-success="handleUploadSuccess"
            @upload-error="handleUploadError"
            @reset="handleReset"
          />
        </div>
        
        <!-- 加载状态 -->
        <div class="loading-section glass-effect" v-else-if="isLoading" key="loading">
          <div class="section-decoration"></div>
          <div class="loading-animation">
            <div class="loading-icon">
              <div class="loading-inner"></div>
            </div>
            <p class="loading-text">正在分析APK文件...</p>
          </div>
          <ElSkeleton :rows="10" animated />
        </div>
        
        <!-- 解析成功后显示结果 -->
        <div class="result-wrapper" v-else key="result">
          <div class="result-header glass-effect">
            <h2 class="section-title">
              <span class="highlight">APK</span> 解析结果
            </h2>
            <ElButton type="primary" size="small" @click="backToUpload" class="back-button">
              <i class="el-icon-arrow-left"></i>
              返回上传页面
            </ElButton>
          </div>
          
          <div class="result-section glass-effect">
            <div class="section-decoration"></div>
            <AnalysisResult />
          </div>
        </div>
      </Transition>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.home-page {
  // max-width: 1200px;
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

    .loading-animation {
      .loading-icon {
        border-color: rgba(255, 255, 255, 0.2);
        border-top-color: var(--el-color-primary);
        
        &::before {
          background: linear-gradient(135deg, 
            var(--el-color-primary-light-3), 
            var(--el-color-primary)
          );
        }
        
        .loading-inner {
          border-color: var(--el-color-primary-light-8);
          border-top-color: transparent;
        }
      }
    }
    
    .result-header {
      .back-button {
        &::before {
          background: rgba(255, 255, 255, 0.05);
        }
      }
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
      padding: 0 0.2em;
    }
    
    .title-decoration {
      position: absolute;
      bottom: -10px;
      left: 50%;
      width: clamp(40px, 8vw, 80px);
      height: 3px;
      background: linear-gradient(90deg, var(--el-color-primary-light-5), var(--el-color-primary));
      transform: translateX(-50%) scaleX(0);
      transition: transform 0.6s ease 0.3s;
      border-radius: 3px;
    }
  }
  
  .glass-effect {
    background: linear-gradient(135deg, 
      var(--glass-bg-light),
      var(--glass-bg-light-end)
    );
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid var(--glass-border);
    box-shadow: var(--glass-shadow);
  }
  
  .section-decoration {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: var(--decoration-color, rgba(59, 130, 246, 0.05));
    mask: 
      linear-gradient(90deg, transparent 0%, #fff 50%, transparent 100%);
    -webkit-mask:
      linear-gradient(90deg, transparent 0%, #fff 50%, transparent 100%);
    animation: shine 3s infinite;
    pointer-events: none;
  }
  
  .page-content {
    display: flex;
    flex-direction: column;
    gap: clamp(20px, 4vw, 40px);
    flex: 1;
    width: 100%;
    margin: 0 auto;
    position: relative;
    padding-bottom: 10%;
  }
  
  .section-title {
    margin-bottom: 16px;
    font-size: clamp(1.25rem, 3vw, 1.5rem);
    color: var(--el-text-color-primary);
    font-weight: 600;
    
    .highlight {
      background: linear-gradient(120deg, var(--el-color-primary-light-5), var(--el-color-primary));
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }
  }
  
  .upload-section, .loading-section {
    border-radius: clamp(8px, 1.5vw, 12px);
    padding: clamp(20px, 4vw, 40px);
    box-shadow: 
      0 8px 24px rgba(0, 0, 0, 0.05),
      0 1px 2px rgba(0, 0, 0, 0.1);
    width: min(100%, 800px);
    margin: auto;
    transition: all 0.3s ease;
    position: relative;
    overflow: hidden;
    
    @media (max-width: 640px) {
      margin: 0;
      width: 100%;
    }
    
    &:hover {
      transform: translateY(-5px);
      box-shadow: 
        0 12px 28px rgba(0, 0, 0, 0.08),
        0 2px 4px rgba(0, 0, 0, 0.1);
    }
  }
  
  .loading-section {
    .loading-animation {
      display: flex;
      flex-direction: column;
      align-items: center;
      margin-bottom: clamp(20px, 4vw, 30px);
      
      .loading-icon {
        width: clamp(40px, 8vw, 60px);
        height: clamp(40px, 8vw, 60px);
        border-radius: 50%;
        position: relative;
        margin-bottom: 15px;
        
        &::before {
          content: '';
          position: absolute;
          inset: 0;
          border-radius: 50%;
          padding: 3px;
          background: linear-gradient(135deg, var(--el-color-primary-light-5), var(--el-color-primary));
          mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
          -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
          -webkit-mask-composite: xor;
          mask-composite: exclude;
          animation: spin 1s linear infinite;
        }
        
        .loading-inner {
          position: absolute;
          inset: 6px;
          border-radius: 50%;
          border: 3px solid var(--el-color-primary-light-8);
          border-top: 3px solid transparent;
          animation: spin 1s linear infinite reverse;
        }
      }
      
      .loading-text {
        color: var(--el-text-color-secondary);
        font-size: clamp(14px, 2vw, 16px);
        text-align: center;
      }
    }
  }
  
  .result-wrapper {
    width: 100%;
    
    .result-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: clamp(16px, 3vw, 24px);
      flex-wrap: wrap;
      gap: 12px;
      border-radius: clamp(8px, 1.5vw, 12px);
      padding: clamp(12px, 2vw, 20px);
      
      @media (max-width: 480px) {
        flex-direction: column;
        align-items: flex-start;
      }
      
      .back-button {
        transition: all 0.3s ease;
        white-space: nowrap;
        position: relative;
        overflow: hidden;
        
        &::before {
          content: '';
          position: absolute;
          top: 50%;
          left: 0;
          width: 100%;
          height: 100%;
          background: rgba(255, 255, 255, 0.1);
          transform: translateY(-50%) scaleX(0);
          transform-origin: right;
          transition: transform 0.3s ease;
        }
        
        &:hover {
          transform: translateX(-3px);
          
          &::before {
            transform: translateY(-50%) scaleX(1);
            transform-origin: left;
          }
        }
        
        i {
          margin-right: 4px;
          position: relative;
        }
      }
    }
    
    .result-section {
      border-radius: clamp(8px, 1.5vw, 12px);
      padding: clamp(16px, 3vw, 24px);
      box-shadow: 
        0 8px 24px rgba(0, 0, 0, 0.05),
        0 1px 2px rgba(0, 0, 0, 0.1);
      transition: all 0.3s ease;
      position: relative;
      overflow: hidden;
      
      &:hover {
        box-shadow: 
          0 12px 28px rgba(0, 0, 0, 0.08),
          0 2px 4px rgba(0, 0, 0, 0.1);
      }
    }
  }
}

// 转场动画
.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}

.fade-slide-enter-from {
  opacity: 0;
  transform: translateY(30px) scale(0.95);
}

.fade-slide-leave-to {
  opacity: 0;
  transform: translateY(-30px) scale(0.95);
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

@keyframes shine {
  0% { transform: translateX(-100%); }
  50%, 100% { transform: translateX(100%); }
}

// 添加加载动画样式
.loading-animation {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  margin-bottom: 2rem;

  .loading-icon {
    width: 40px;
    height: 40px;
    border: 3px solid var(--el-border-color-light);
    border-top: 3px solid var(--el-color-primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .loading-text {
    color: var(--el-text-color-regular);
    font-size: 0.9rem;
  }
}
</style> 