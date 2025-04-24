<template>
  <div class="app-nav" :class="{ 
    collapsed: isCollapsed,
    initialized: isInitialized 
  }">
    <div class="nav-header">
      <div class="logo-container">
        <img src="../assets/logo.png" alt="Logo" class="logo" />
        <span class="app-title">APK 解析工具</span>
      </div>
    </div>

    <el-menu
      class="nav-menu"
      :collapse="isCollapsed"
      :default-active="activeRoute"
      router
      @select="handleSelect"
    >
      <el-menu-item index="/">
        <el-icon><House /></el-icon>
        <template #title>首页</template>
      </el-menu-item>
      <el-menu-item index="/history">
        <el-icon><Document /></el-icon>
        <template #title>历史记录</template>
      </el-menu-item>
      <el-menu-item index="/help">
        <el-icon><InfoFilled /></el-icon>
        <template #title>使用帮助</template>
      </el-menu-item>
      <el-menu-item index="/settings">
        <el-icon><Setting /></el-icon>
        <template #title>设置</template>
      </el-menu-item>
    </el-menu>

    <div class="nav-footer">
      <div class="status-wrapper" :class="{ hidden: isCollapsed }">
        <div class="version-info">
          <el-icon><Connection /></el-icon>
          <span class="version-text">v{{ appVersion }}</span>
        </div>
      </div>

      <el-menu
        class="control-menu"
        :collapse="isCollapsed"
      >
        <el-menu-item @click="toggleTheme">
          <el-icon>
            <Moon v-if="isDarkTheme" />
            <Sunny v-else />
          </el-icon>
          <template #title>{{ isDarkTheme ? '深色模式' : '浅色模式' }}</template>
        </el-menu-item>

        <el-menu-item @click="toggleCollapse">
          <el-icon>
            <Fold v-if="!isCollapsed" />
            <Expand v-else />
          </el-icon>
          <template #title>{{ isCollapsed ? '展开' : '收起' }}</template>
        </el-menu-item>
      </el-menu>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { 
  House,
  InfoFilled,
  Loading,
  CircleCheckFilled,
  Connection,
  Moon,
  Sunny,
  Fold,
  Expand,
  Setting,
  Document
} from '@element-plus/icons-vue';
import { themeManager } from '../utils/theme';

const props = defineProps<{ 
  appVersion: string;
  isCollapsed: boolean;
}>();

const route = useRoute();
const router = useRouter();
const activeRoute = computed(() => route.path);
const isDarkTheme = computed(() => themeManager.theme.value === 'dark');
const isInitialized = ref(false);

const emit = defineEmits<{
  'update:isCollapsed': [value: boolean];
}>();

const handleSelect = (key: string) => {
  try {
    console.log('Selected menu item:', key);
    // 确保路由切换前 DOM 元素存在
    if (!document.querySelector('.app-nav')) {
      console.warn('Navigation container not found');
      return;
    }
    // 执行路由导航
    router.push(key);
  } catch (error) {
    console.error('Error in handleSelect:', error);
  }
};

const toggleCollapse = () => {
  emit('update:isCollapsed', !props.isCollapsed);
  localStorage.setItem('nav-collapsed', (!props.isCollapsed).toString());
};

const toggleTheme = () => {
  themeManager.toggleTheme();
};

onMounted(() => {
  try {
    // 恢复导航栏状态
    const savedState = localStorage.getItem('nav-collapsed');
    if (savedState !== null) {
      emit('update:isCollapsed', savedState === 'true');
    }
    
    // 延迟添加初始化标记，避免首次加载动画
    requestAnimationFrame(() => {
      try {
        isInitialized.value = true;
      } catch (error) {
        console.error('Error in requestAnimationFrame:', error);
      }
    });
  } catch (error) {
    console.error('Error in AppNav onMounted:', error);
  }
});

onUnmounted(() => {
  try {
    // 清理可能的事件监听器或其他资源
    isInitialized.value = false;
  } catch (error) {
    console.error('Error in AppNav onUnmounted:', error);
  }
});
</script>

<style lang="scss" scoped>
.app-nav {
  width: 250px;
  height: 100%;
  background: var(--el-bg-color);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  position: relative;
  transform: translateZ(0);

  &:not(.initialized) * {
    transition: none !important;
  }

  &.initialized {
    // transition: width 15ms ease-out;
  }

  &.collapsed {
    width: 64px;

    .app-title {
      opacity: 0;
      width: 0;
      margin-left: 0;
    }

    .logo-container {
      justify-content: center;
    }
  }
}

.nav-header {
  padding: 1rem;
  display: flex;
  align-items: center;
  border-bottom: 1px solid var(--el-border-color-light);
  min-height: 64px;
}

.logo-container {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex: 1;
  min-width: 0;
  transition: justify-content 150ms ease-out;
}

.logo {
  width: 32px;
  height: 32px;
  flex-shrink: 0;
}

.app-title {
  font-weight: 600;
  font-size: 1.1rem;
  white-space: nowrap;
  overflow: hidden;
  margin-left: 0.75rem;
  transition: all 150ms ease-out;
}

.nav-menu {
  flex: 1;
  border-right: none;

  :deep(.el-menu) {
    border-right: none;
  }

  :deep(.el-menu-item) {
    // transition: padding 15ms ease-out;
  }
}

.nav-footer {
  border-top: 1px solid var(--el-border-color-light);
  padding: 0.5rem 0;
}

.status-wrapper {
  // transition: opacity 15ms ease-out;
  
  &.hidden {
    height: 0;
    opacity: 0;
    overflow: hidden;
  }
}
 
.version-info {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem 1rem;
  color: var(--el-text-color-regular);

  .el-icon {
    font-size: 1.2rem;
    width: 24px;
    text-align: center;
  }
}

.control-menu {
  border-right: none;
  margin-top: 0.5rem;

  :deep(.el-menu-item) {
    // transition: padding 15ms ease-out;
    border-top: 1px solid var(--el-border-color-light);
  }
}

@media (max-width: 768px) {
  .app-nav {
    width: 100%;
    height: auto;

    &.initialized {
      // transition: height 15ms ease-out;
    }

    &.collapsed {
      height: 64px;
      overflow: hidden;
    }
  }

  .nav-menu,
  .control-menu {
    flex-direction: row !important;
  }

  .nav-footer {
    border: none;
    display: flex;
    align-items: center;
    padding: 0;

    .status-wrapper {
      display: flex;
      align-items: center;
    }

    .status-item,
    .version-info {
      border: none;
      padding: 0.5rem;
    }
  }
}

:root {
  --el-menu-bg-color: var(--el-bg-color);
  --el-menu-hover-bg-color: var(--el-fill-color-light);
  --el-menu-active-color: var(--el-color-primary);
  --el-color-primary: #1a73e8;
}

.dark {
  --el-bg-color: #1a1a1a;
  --el-text-color-regular: #e0e0e0;
  --el-border-color-light: #333;
  --el-fill-color-light: #2c2c2c;
  --el-color-primary-light-9: rgba(26, 115, 232, 0.1);
  --el-menu-bg-color: var(--el-bg-color);
  --el-menu-text-color: var(--el-text-color-regular);
  --el-menu-hover-bg-color: var(--el-fill-color-light);
  --el-menu-border-color: var(--el-border-color-light);
}
</style> 