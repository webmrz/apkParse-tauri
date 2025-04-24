<template>
  <div class="app-container">
    <AppNav  
      :app-version="appVersion"
      v-model:is-collapsed="navCollapsed"
    />
    <main class="content-area" >
      <router-view v-slot="{ Component }">
        <component 
          :is="Component" 
          v-if="!routeError"
          @error="handleRouteError"
        />
        <div v-else class="error-container">
          <h2>页面加载出错</h2>
          <p>{{ routeError }}</p>
          <el-button @click="retryNavigation">重试</el-button>
        </div>
      </router-view>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue"; 
import AppNav from './components/AppNav.vue';
import { themeManager } from './utils/theme'; 
// 导航栏折叠状态
const navCollapsed = ref(false); 
// 应用版本
const appVersion = ref('1.0.0');
// 路由错误状态
const routeError = ref<string | null>(null);

const handleRouteError = (error: Error) => {
  console.error('Route error:', error);
  routeError.value = error.message;
};

const retryNavigation = () => {
  routeError.value = null;
  window.location.reload();
};

// 确保主题管理器在应用启动时初始化
onMounted(() => {
  try {
    // themeManager 会在导入时自动初始化
    console.log('Current theme:', themeManager.theme.value);
  } catch (error) {
    console.error('Error initializing theme:', error);
  }
});
</script>

<style>
@import './assets/reset.css';

.app-container {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background-color: #f8f9fa;
}

.content-area {
  flex: 1;
  overflow-y: auto;
  transition: margin-left 0.3s cubic-bezier(0.4, 0, 0.2, 1); 
}

@media (max-width: 768px) {
  .app-container {
    flex-direction: column;
  }
  
  .content-area {
    margin-left: 0 !important;
    padding: 1rem;
  }
}

/* 全局主题变量 */
:root {
  /* 浅色主题变量 */
  --el-color-primary: #1a73e8;
  --el-bg-color: #ffffff;
  --el-text-color-primary: #303133;
  --el-text-color-regular: #606266;
  --el-border-color: #dcdfe6;
  --el-border-color-light: #e4e7ed;
  --el-fill-color: #f0f2f5;
  --el-fill-color-light: #f5f7fa;
  --el-bg-color-overlay: #ffffff;
  --app-bg: #f5f7fa;
  --glass-bg-light: rgba(255, 255, 255, 0.95);
  --glass-bg-light-end: rgba(255, 255, 255, 0.85);
  --glass-border: rgba(255, 255, 255, 0.2);
  --glass-shadow: 0 8px 24px rgba(0, 0, 0, 0.05), 0 1px 2px rgba(0, 0, 0, 0.1);
  --decoration-opacity: 0.1;
}

/* 深色主题变量 */
html.dark {
  --el-color-primary: #1a73e8;
  --el-bg-color: #1a1a1a;
  --el-text-color-primary: #ffffff;
  --el-text-color-regular: #e0e0e0;
  --el-border-color: #333333;
  --el-border-color-light: #333;
  --el-fill-color: #2c2c2c;
  --el-fill-color-light: #2c2c2c;
  --el-bg-color-overlay: #1a1a1a;

  /* Element Plus 深色主题变量 */
  --el-menu-bg-color: var(--el-bg-color);
  --el-menu-text-color: var(--el-text-color-regular);
  --el-menu-hover-bg-color: var(--el-fill-color-light);
  --el-menu-border-color: var(--el-border-color-light);
  --el-menu-active-color: var(--el-color-primary);
  
  /* 其他深色主题特定样式 */
  color-scheme: dark;
  --app-bg: #121212;
  --glass-bg-dark: rgba(30, 41, 59, 0.8);
  --glass-bg-dark-end: rgba(15, 23, 42, 0.8);
  --glass-border-dark: rgba(255, 255, 255, 0.1);
  --glass-shadow-dark: 0 8px 24px rgba(0, 0, 0, 0.2), 0 1px 2px rgba(0, 0, 0, 0.3);
  --decoration-opacity: 0.15;
  --el-bg-color-overlay: #242424;
  --el-text-color-primary: #e5eaf3;
  --el-text-color-secondary: #a3a6ad;
  --el-border-color: #363637;
  --el-fill-color-blank: var(--el-bg-color);
  --el-mask-color: rgba(0, 0, 0, 0.8);
  --el-mask-color-extra-light: rgba(0, 0, 0, 0.3);
  --el-card-bg-color: #242424;
}

/* 主题切换过渡效果 */
* {
  transition: background-color 0.15s ease-out, 
              border-color 0.15s ease-out, 
              color 0.15s ease-out;
  transition-property: background-color, border-color, color, fill, stroke;
  transition-timing-function: ease;
  transition-duration: 0.3s;
}

/* Global styles */
body {
  background-color: var(--app-bg);
  color: var(--el-text-color-regular);
  transition: background-color 0.3s ease, color 0.3s ease;
}

/* Disable transitions for performance */
.disable-transitions * {
  transition: none !important;
}

.error-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  padding: 2rem;
  text-align: center;
  color: var(--el-text-color-regular);
}

.error-container h2 {
  margin-bottom: 1rem;
  color: var(--el-color-danger);
}

.error-container p {
  margin-bottom: 2rem;
  color: var(--el-text-color-secondary);
}
</style>