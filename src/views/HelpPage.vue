<script setup lang="ts">
import { computed, ref } from 'vue'; 
import { themeManager } from '../utils/theme';
import SupportInfo from '../components/SupportInfo.vue';

const isDarkMode = computed(() => themeManager.theme.value === 'dark');
const activeSection = ref('basic'); // 用于追踪当前激活的部分

const sections = [
  {
    id: 'basic',
    title: '基本使用',
    icon: '📱'
  },
  {
    id: 'analysis',
    title: '分析结果',
    icon: '📊'
  },
  {
    id: 'advanced',
    title: '高级分析（TODO）',
    icon: '🔍'
  }
];
</script>

<template>
  <div class="help-page" :class="{ 'dark-mode': isDarkMode }">
    <div class="help-content glass-effect">
      <h1 class="page-title">
        <span class="title-icon">📖</span>
        使用帮助
      </h1>
      
      <div class="section-tabs">
        <button
          v-for="section in sections"
          :key="section.id"
          class="tab-button"
          :class="{ active: activeSection === section.id }"
          @click="activeSection = section.id"
        >
          <span class="tab-icon">{{ section.icon }}</span>
          {{ section.title }}
        </button>
      </div>
      
      <div class="help-sections">
        <transition name="fade" mode="out-in">
          <div v-if="activeSection === 'basic'" key="basic" class="help-section">
            <h2>APK分析工具使用指南</h2>
            <div class="feature-card">
              <p class="feature-intro">本工具可帮助您分析Android APK文件，提取包信息、权限、证书等关键数据。</p>
            </div>
            
            <h3>基本使用步骤</h3>
            <div class="steps-container">
              <div class="step-item">
                <div class="step-number">1</div>
                <div class="step-content">在首页拖放或选择一个APK文件</div>
              </div>
              <div class="step-item">
                <div class="step-number">2</div>
                <div class="step-content">点击"解析APK"按钮开始分析</div>
              </div>
              <div class="step-item">
                <div class="step-number">3</div>
                <div class="step-content">分析完成后，结果会直接显示在页面上</div>
              </div>
            </div>
          </div>

          <div v-else-if="activeSection === 'analysis'" key="analysis" class="help-section">
            <h2>分析结果说明</h2>
            <div class="info-cards">
              <div class="info-card">
                <div class="card-header">
                  <span class="card-icon">ℹ️</span>
                  <h4>基本信息</h4>
                </div>
                <p>包名、版本号、SDK版本等</p>
              </div>
              
              <div class="info-card">
                <div class="card-header">
                  <span class="card-icon">🔒</span>
                  <h4>权限</h4>
                </div>
                <p>应用申请的所有权限，危险权限会被特别标记</p>
              </div>
              
              <div class="info-card">
                <div class="card-header">
                  <span class="card-icon">📜</span>
                  <h4>证书信息</h4>
                </div>
                <p>应用签名相关信息，包括有效期和指纹</p>
              </div>
              
              <div class="info-card">
                <div class="card-header">
                  <span class="card-icon">📁</span>
                  <h4>文件信息</h4>
                </div>
                <p>APK文件哈希值、文件大小等</p>
              </div>
            </div>
          </div>

          <div v-else-if="activeSection === 'advanced'" key="advanced" class="help-section">
            <h2>高级分析功能</h2>
            <div class="feature-card">
              <p class="feature-intro">本工具提供多种高级分析功能，帮助您深入了解APK文件的安全性和内部结构。</p>
            </div>

            <div class="advanced-features">
              <div class="feature-item">
                <div class="feature-header">
                  <span class="feature-icon">🦠</span>
                  <h4>病毒分析</h4>
                </div>
                <ul class="feature-list">
                  <li>离线病毒特征码扫描</li>
                  <li>恶意行为模式识别</li>
                  <li>风险代码检测</li>
                  <li>隐私合规检查</li>
                </ul>
              </div>

              <div class="feature-item">
                <div class="feature-header">
                  <span class="feature-icon">🔧</span>
                  <h4>反编译分析</h4>
                </div>
                <ul class="feature-list">
                  <li>Java源码反编译</li>
                  <li>资源文件提取</li>
                  <li>代码结构分析</li>
                  <li>敏感API调用检测</li>
                </ul>
              </div>

              <div class="feature-item">
                <div class="feature-header">
                  <span class="feature-icon">📈</span>
                  <h4>即将推出</h4>
                </div>
                <ul class="feature-list">
                  <li>动态行为分析</li>
                  <li>网络通信监控</li>
                  <li>第三方SDK识别</li>
                  <li>安全漏洞扫描</li>
                </ul>
              </div>
            </div>
          </div> 
        </transition>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.help-page {
  min-height: 100vh;
  padding: clamp(16px, 3vw, 40px);
  background: var(--app-bg);
  color: var(--el-text-color-regular);
  
  &.dark-mode {
    --gradient-start: var(--glass-bg-dark);
    --gradient-end: var(--glass-bg-dark-end);
    --glass-border-color: var(--glass-border-dark);
    --glass-shadow: var(--glass-shadow-dark);
    
    .glass-effect {
      background: linear-gradient(135deg, var(--gradient-start), var(--gradient-end));
      border-color: var(--glass-border-color);
      box-shadow: var(--glass-shadow);
    }
    
    .page-title::after {
      background: linear-gradient(90deg, 
        var(--el-color-primary-light-3), 
        var(--el-color-primary)
      );
    }
    
    .info-card {
      background: rgba(255, 255, 255, 0.05);
      border-color: rgba(255, 255, 255, 0.1);
    }
    
    .step-number {
      background: var(--el-color-primary-dark-2);
    }
  }
  
  .help-content {
    max-width: 900px;
    margin: 0 auto;
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
    border-radius: 16px;
    padding: clamp(24px, 4vw, 48px);
  }
  
  .page-title {
    font-size: clamp(1.75rem, 4vw, 2.25rem);
    margin-bottom: 2.5rem;
    color: var(--el-text-color-primary);
    font-weight: 700;
    display: flex;
    align-items: center;
    gap: 12px;
    
    .title-icon {
      font-size: 1.2em;
    }
  }

  .section-tabs {
    display: flex;
    gap: 12px;
    margin-bottom: 2rem;
    
    .tab-button {
      padding: 10px 20px;
      border: none;
      background: transparent;
      color: var(--el-text-color-regular);
      border-radius: 8px;
      cursor: pointer;
      transition: all 0.3s ease;
      display: flex;
      align-items: center;
      gap: 8px;
      font-size: 1rem;
      
      &:hover {
        background: var(--el-fill-color-light);
      }
      
      &.active {
        background: var(--el-color-primary);
        color: white;
      }
      
      .tab-icon {
        font-size: 1.2em;
      }
    }
  }

  .feature-card {
    background: var(--el-bg-color-overlay);
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 2rem;
    border: 1px solid var(--el-border-color-light);
    
    .feature-intro {
      font-size: 1.1rem;
      line-height: 1.6;
      margin: 0;
    }
  }

  .steps-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    
    .step-item {
      display: flex;
      align-items: center;
      gap: 1rem;
      padding: 1rem;
      background: var(--el-bg-color-overlay);
      border-radius: 8px;
      transition: transform 0.3s ease;
      
      &:hover {
        transform: translateX(8px);
      }
      
      .step-number {
        width: 32px;
        height: 32px;
        background: var(--el-color-primary);
        color: white;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: bold;
      }
      
      .step-content {
        font-size: 1.1rem;
      }
    }
  }

  .info-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
    margin-top: 1.5rem;
    
    .info-card {
      background: var(--el-bg-color-overlay);
      border: 1px solid var(--el-border-color-light);
      border-radius: 12px;
      padding: 1.25rem;
      transition: all 0.3s ease;
      
      &:hover {
        transform: translateY(-4px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
      }
      
      .card-header {
        display: flex;
        align-items: center;
        gap: 8px;
        margin-bottom: 0.75rem;
        
        .card-icon {
          font-size: 1.5rem;
        }
        
        h4 {
          margin: 0;
          font-size: 1.1rem;
          font-weight: 600;
          color: var(--el-text-color-primary);
        }
      }
      
      p {
        margin: 0;
        color: var(--el-text-color-regular);
        line-height: 1.5;
      }
    }
  }

  .advanced-features {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 1.5rem;
    margin-top: 2rem;

    .feature-item {
      background: var(--el-bg-color-overlay);
      border: 1px solid var(--el-border-color-light);
      border-radius: 12px;
      padding: 1.5rem;
      transition: all 0.3s ease;

      &:hover {
        transform: translateY(-4px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
      }

      .feature-header {
        display: flex;
        align-items: center;
        gap: 12px;
        margin-bottom: 1rem;

        .feature-icon {
          font-size: 1.75rem;
        }

        h4 {
          margin: 0;
          font-size: 1.2rem;
          font-weight: 600;
          color: var(--el-text-color-primary);
        }
      }

      .feature-list {
        list-style: none;
        padding: 0;
        margin: 0;

        li {
          position: relative;
          padding-left: 1.5rem;
          margin-bottom: 0.75rem;
          color: var(--el-text-color-regular);

          &:before {
            content: "→";
            position: absolute;
            left: 0;
            color: var(--el-color-primary);
          }

          &:last-child {
            margin-bottom: 0;
          }
        }
      }
    }
  }

  .note-card {
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    padding: 15px;
    margin-top: 20px;
    text-align: center;
    font-size: 0.9rem;
    color: var(--el-text-color-secondary);
    border-left: 4px solid var(--el-color-primary);
  }
}

// 过渡动画
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style> 