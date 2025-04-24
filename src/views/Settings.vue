<template>
  <div class="settings-container">
    <el-card class="settings-card" shadow="never">
      <el-tabs v-model="activeTab">
        <el-tab-pane label="程序信息" name="app">
          <div class="settings-section">
            <h3>基础信息</h3>
            <el-descriptions :column="1" border>
              <el-descriptions-item label="版本">
                {{ appVersion }}
              </el-descriptions-item>
              <el-descriptions-item label="构建时间">
                {{ buildTime }}
              </el-descriptions-item>
            </el-descriptions>

            <h3 class="mt-4">系统信息</h3>
            <el-descriptions :column="1" border>
              <el-descriptions-item label="操作系统">
                {{ osName }} {{ osVersion }}
              </el-descriptions-item>
              <el-descriptions-item label="内核版本">
                {{ kernelVersion }}
              </el-descriptions-item>
              <el-descriptions-item label="主机名">
                {{ hostName }}
              </el-descriptions-item>
            </el-descriptions>

            <h3 class="mt-4">硬件信息</h3>
            <el-descriptions :column="1" border>
              <el-descriptions-item label="处理器">
                {{ cpuInfo.brand }} ({{ cpuInfo.cores_count }}核)
              </el-descriptions-item>
              <el-descriptions-item label="CPU使用率">
                <el-progress 
                  :percentage="Math.round(cpuInfo.usage_percent)" 
                  :format="(val: number) => val + '%'"
                />
              </el-descriptions-item>
              <el-descriptions-item label="内存使用情况">
                <div>
                  总内存: {{ formatBytes(memoryInfo.total_memory) }}
                  <el-progress 
                    :percentage="Math.round((memoryInfo.used_memory / memoryInfo.total_memory) * 100)"
                    :format="() => formatBytes(memoryInfo.used_memory) + ' / ' + formatBytes(memoryInfo.total_memory)"
                  />
                </div>
              </el-descriptions-item>
              <el-descriptions-item label="交换内存">
                <div v-if="memoryInfo.total_swap > 0">
                  总交换内存: {{ formatBytes(memoryInfo.total_swap) }}
                  <el-progress 
                    :percentage="Math.round((memoryInfo.used_swap / memoryInfo.total_swap) * 100)"
                    :format="() => formatBytes(memoryInfo.used_swap) + ' / ' + formatBytes(memoryInfo.total_swap)"
                  />
                </div>
                <span v-else>未使用</span>
              </el-descriptions-item>
            </el-descriptions>
          </div>
        </el-tab-pane>

        <!-- <el-tab-pane label="日志信息" name="logs">
          <div class="settings-section">
            <h3>日志信息</h3>
            <el-space vertical>
              <el-select
                v-model="logLevel"
                :options="logLevelOptions"
                placeholder="选择日志级别"
              />
              <el-input-number
                v-model="maxLogDays"
                :min="1"
                :max="90"
                placeholder="日志保留天数"
              >
                <template #prefix>保留</template>
                <template #suffix>天</template>
              </el-input-number>
              <el-button @click="clearLogs" type="warning">
                清除日志
              </el-button>
            </el-space>
          </div>
        </el-tab-pane> -->

        <el-tab-pane label="开发者信息" name="developer">
          <div class="settings-section">
            <h3>开发者信息</h3>
            <el-descriptions :column="1" border>
              <el-descriptions-item label="开发者">
                若愚6792
              </el-descriptions-item>
              <el-descriptions-item label="GitHub">
                <el-link href="https://github.com/webmrz" target="_blank" type="primary">
                  https://github.com/webmrz
                </el-link>
              </el-descriptions-item>
              <el-descriptions-item label="联系方式">
                webmrz@gmail.com
              </el-descriptions-item>
            </el-descriptions>
          </div>
        </el-tab-pane>

        <el-tab-pane label="关注与支持" name="social">
          <div class="settings-section">
            <el-row :gutter="20">
              <el-col :span="12">
                <el-card class="social-card" shadow="hover">
                  <template #header>
                    <div class="card-header">
                      <h3>微信公众号</h3>
                      <el-tag type="success">扫码关注</el-tag>
                    </div>
                  </template>
                  <div class="qr-container">
                    <el-image
                      :src="wechatImage"
                      :preview-src-list="[wechatImage]"
                      fit="contain"
                      class="qr-code"
                      :initial-index="0"
                      hide-on-click-modal
                      preview-teleported
                    >
                      <template #error>
                        <div class="image-error">
                          <el-icon><Picture /></el-icon>
                          <span>加载失败</span>
                        </div>
                      </template>
                    </el-image>
                    <div class="qr-description">
                      <p>扫描二维码关注公众号</p>
                      <p class="sub-text">获取最新更新和更多功能</p>
                    </div>
                  </div>
                </el-card>
              </el-col>
              <el-col :span="12">
                <el-card class="social-card" shadow="hover">
                  <template #header>
                    <div class="card-header">
                      <h3>赞赏支持</h3>
                      <el-tag type="warning">感谢支持</el-tag>
                    </div>
                  </template>
                  <div class="qr-container">
                    <el-image
                      :src="donationImage"
                      :preview-src-list="[donationImage]"
                      fit="contain"
                      class="qr-code"
                      :initial-index="0"
                      hide-on-click-modal
                      preview-teleported
                    >
                      <template #error>
                        <div class="image-error">
                          <el-icon><Picture /></el-icon>
                          <span>加载失败</span>
                        </div>
                      </template>
                    </el-image>
                    <div class="qr-description">
                      <p>扫描二维码赞赏支持</p>
                      <p class="sub-text">您的支持是我持续开发的动力</p>
                    </div>
                  </div>
                </el-card>
              </el-col>
            </el-row>
          </div>
        </el-tab-pane> 
      </el-tabs>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import wechatQrCode from '../assets/wechat.png'
import donationQrCode from '../assets/donation.jpg'
// import { platform } from '@tauri-apps/api/os'

interface AppInfo {
  version: string
  build_time: string
  os_name: string
  os_version: string
  kernel_version: string
  host_name: string
  cpu_info: {
    brand: string
    cores_count: number
    usage_percent: number
  }
  memory_info: {
    total_memory: number
    used_memory: number
    total_swap: number
    used_swap: number
  }
}

interface AppSettings {
  logLevel: string
  maxLogDays: number
  autoStart: boolean
  autoUpdate: boolean
}

const appVersion = ref('')
const buildTime = ref('')
const osName = ref('')
const osVersion = ref('')
const kernelVersion = ref('')
const hostName = ref('')
const cpuInfo = ref<AppInfo['cpu_info']>({
  brand: '',
  cores_count: 0,
  usage_percent: 0
})
const memoryInfo = ref<AppInfo['memory_info']>({
  total_memory: 0,
  used_memory: 0,
  total_swap: 0,
  used_swap: 0
})

const wechatImage = ref(wechatQrCode)
const donationImage = ref(donationQrCode)

const logLevelOptions = [
  { label: '调试', value: 'debug' },
  { label: '信息', value: 'info' },
  { label: '警告', value: 'warn' },
  { label: '错误', value: 'error' }
]

const activeTab = ref('app')  // Set default tab to 'app'

// Format bytes to human readable format
const formatBytes = (bytes: number): string => {
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  let size = bytes
  let unitIndex = 0
  
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }
  
  return `${size.toFixed(2)} ${units[unitIndex]}`
}

onMounted(async () => {
  try {
    // 获取应用信息
    const appInfo = await invoke<AppInfo>('get_app_info')
    
    appVersion.value = appInfo.version
    buildTime.value = appInfo.build_time
    osName.value = appInfo.os_name
    osVersion.value = appInfo.os_version
    kernelVersion.value = appInfo.kernel_version
    hostName.value = appInfo.host_name
    cpuInfo.value = appInfo.cpu_info
    memoryInfo.value = appInfo.memory_info
     
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
})

const clearLogs = async () => {
  try {
    await invoke('clear_logs')
    // 可以添加成功提示
  } catch (error) {
    console.error('Failed to clear logs:', error)
  }
}

// 监听设置变化并保存
// watch([logLevel, maxLogDays, autoStart, autoUpdate], async () => {
//   try {
//     await invoke('save_settings', {
//       settings: {
//         logLevel: logLevel.value,
//         maxLogDays: maxLogDays.value,
//         autoStart: autoStart.value,
//         autoUpdate: autoUpdate.value
//       }
//     })
//   } catch (error) {
//     console.error('Failed to save settings:', error)
//   }
// })
</script>

<style lang="scss" scoped>
.settings-container {
  padding: clamp(16px, 3vw, 40px); 
  margin: 0 auto;
  height: 100%;
  min-height: 100vh;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  background: var(--app-bg);
  color: var(--el-text-color-regular);
}

.settings-card {
  flex: 1;
  border: none;
  background-color: var(--el-bg-color);
  color: var(--el-text-color-primary);
  border-radius: 8px;
  box-shadow: var(--el-box-shadow-light) !important;

  :deep(.el-card__body) {
    height: 100%;
    padding: 0;
  }

  :deep(.el-tabs) {
    height: 100%;
  }

  :deep(.el-tabs__content) {
    padding: 0;
    height: calc(100% - 55px);
    box-sizing: border-box;
  }

  :deep(.el-tab-pane) {
    height: 100%;
  }
}

.settings-section {
  padding: 24px;
  height: 100%;
  background-color: var(--el-bg-color);
  border-radius: var(--el-border-radius-base);
  box-sizing: border-box;
  overflow-y: auto;

  h3 {
    margin-bottom: 24px;
    font-size: 18px;
    font-weight: 500;
    color: var(--el-text-color-primary);
    display: flex;
    align-items: center;
    gap: 8px;
  }
}

.social-card {
  height: 100%;
  transition: transform 0.3s ease;
  
  &:hover {
    transform: translateY(-5px);
  }
  
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    
    h3 {
      margin: 0;
      font-size: 1.1rem;
      font-weight: 500;
    }
  }
  
  .qr-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 20px;
    
    .qr-code {
      
      border-radius: 8px;
      overflow: hidden;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
      transition: transform 0.3s ease;
      
      &:hover {
        transform: scale(1.05);
      }
    }
    
    .image-error {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      height: 100%;
      color: var(--el-text-color-secondary);
      
      .el-icon {
        font-size: 2rem;
        margin-bottom: 8px;
      }
    }
    
    .qr-description {
      margin-top: 16px;
      text-align: center;
      
      p {
        margin: 4px 0;
        color: var(--el-text-color-regular);
        
        &.sub-text {
          font-size: 0.9rem;
          color: var(--el-text-color-secondary);
        }
      }
    }
  }
}

.mt-4 {
  margin-top: 24px;
}

:deep(.el-tabs__nav-wrap) {
  padding: 0 16px;

  &::after {
    height: 1px;
    opacity: 0.5;
    background-color: var(--el-border-color);
  }
}

.el-descriptions {
  --el-descriptions-item-bordered-label-background: var(--el-fill-color-light);
  background-color: var(--el-bg-color);
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid var(--el-border-color-light);
}

.el-descriptions__cell {
  padding: 16px 24px;
  color: var(--el-text-color-regular);
}

.el-descriptions__label {
  color: var(--el-text-color-primary);
  font-weight: 500;
}

.el-progress {
  margin-top: 8px;
  width: 100%;

  &-bar__outer {
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
  }

  &-bar__inner {
    border-radius: 8px;
    transition: all 0.3s ease;
  }

  &__text {
    color: var(--el-text-color-regular);
    font-size: 13px;
    font-weight: normal;
  }
}

.el-tabs__item {
  color: var(--el-text-color-regular);
  font-size: 14px;
  padding: 0 16px;
  height: 40px;
  line-height: 40px;

  &.is-active {
    color: var(--el-color-primary);
    font-weight: 500;
  }

  &:hover {
    color: var(--el-color-primary);
  }
}

.el-link {
  font-size: 14px;
  transition: all 0.3s ease;

  &:hover {
    text-decoration: none;
  }
}

// 响应式调整
@media (max-width: 768px) {
  .el-col {
    width: 100%;
    margin-bottom: 20px;
  }
  
  .social-card {
    .qr-container {
      .qr-code {
        width: 160px;
        height: 160px;
      }
    }
  }
}
</style> 