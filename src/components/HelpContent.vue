<script setup lang="ts">
import { ref  } from 'vue';
import { QuestionFilled, Document, Warning, InfoFilled } from '@element-plus/icons-vue';

const activeTab = ref('guide');

// 指南内容
const guideContent = [
  {
    title: '开始使用',
    icon: Document,
    content: `
      <h3>欢迎使用APK分析工具</h3>
      <p>这是一个用于快速分析Android APK文件的桌面工具，可以帮助您提取APK的基本信息、权限列表、签名信息等内容。</p>
      
      <h4>基本使用步骤：</h4>
      <ol>
        <li>拖放APK文件到上传区域，或点击选择文件</li>
        <li>系统会自动解析APK并显示结果</li>
        <li>您可以查看不同标签页获取详细信息</li>
        <li>可以导出结果为JSON或图片格式</li>
      </ol>
    `
  },
  {
    title: '功能介绍',
    icon: InfoFilled,
    content: `
      <h3>主要功能</h3>
      <ul>
        <li><strong>基本信息查看</strong> - 包括应用名称、包名、版本、SDK信息等</li>
        <li><strong>权限分析</strong> - 展示应用申请的所有权限，并标记危险权限</li>
        <li><strong>签名验证</strong> - 显示APK签名信息，包含多种哈希值</li>
        <li><strong>文件信息</strong> - 显示APK文件大小、MD5、SHA1等校验信息</li>
        <li><strong>导出功能</strong> - 支持将分析结果导出为JSON或截图</li>
      </ul>
      
      <h3>高级功能</h3>
      <ul>
        <li><strong>批量分析</strong> - 支持同时分析多个APK文件（计划中）</li>
        <li><strong>对比分析</strong> - 比较两个APK版本的差异（计划中）</li>
      </ul>
    `
  },
  {
    title: '工具配置',
    icon: InfoFilled,
    content: `
      <h3>配置说明</h3>
      <p>本工具依赖aapt2.exe来解析APK文件，如果您收到提示需要安装aapt2，请按照以下步骤操作：</p>
      
      <ol>
        <li>从Android SDK下载aapt2.exe</li>
        <li>将文件放置在应用程序的tools目录中</li>
        <li>重启应用程序</li>
      </ol>
      
      <p>您也可以使用内置的下载脚本自动完成此操作：</p>
      <pre>在tools目录中运行 ./download_aapt2.ps1 脚本</pre>
    `
  }
];

// 常见问题
const faqItems = [
  {
    question: '为什么我无法上传APK文件？',
    answer: '请确保您选择的是有效的APK文件。文件必须具有.apk扩展名，并且大小不应超过100MB。如果问题仍然存在，请尝试重启应用程序。'
  },
  {
    question: 'APK分析结果不完整怎么办？',
    answer: '这通常是因为aapt2工具未正确安装。请检查"帮助"页面中的aapt2安装指南，确保工具已正确安装在应用程序的tools目录中。'
  },
  {
    question: '如何导出分析结果？',
    answer: '在分析结果页面，点击右上角的"导出"按钮，选择您想要的导出格式（JSON或图片）。导出的文件将保存到您选择的位置。'
  },
  {
    question: '为什么有些APK无法解析？',
    answer: '可能的原因包括：<br>1. APK文件已损坏<br>2. APK使用了特殊的保护措施或加固<br>3. APK格式不符合标准<br><br>对于这些情况，您可以尝试使用其他专业工具进行分析。'
  },
  {
    question: '应用程序支持哪些系统？',
    answer: '当前版本支持Windows 10/11。未来计划支持macOS和Linux系统。'
  },
  {
    question: '是否支持分析AAB文件？',
    answer: '目前不支持直接分析Android App Bundle (.aab)文件。我们计划在未来版本中添加此功能。'
  }
];

// 故障排除
const troubleshootItems = [
  {
    title: 'aapt2工具问题',
    icon: Warning,
    content: `
      <h3>aapt2工具问题排查</h3>
      <p>如果您看到"aapt2.exe未找到或不可用"的错误：</p>
      
      <ol>
        <li>检查tools目录中是否存在aapt2.exe文件</li>
        <li>确认文件有执行权限</li>
        <li>使用诊断功能获取更详细的错误信息</li>
        <li>尝试从Android SDK中获取最新版本的aapt2.exe</li>
      </ol>
    `
  },
  {
    title: '权限问题',
    icon: Warning,
    content: `
      <h3>应用程序权限问题</h3>
      <p>如果应用程序无法读取或写入文件：</p>
      
      <ol>
        <li>确保您有足够的权限访问相关文件夹</li>
        <li>尝试以管理员身份运行应用程序</li>
        <li>检查您的防病毒软件是否阻止了应用程序的操作</li>
      </ol>
    `
  },
  {
    title: '性能问题',
    icon: Warning,
    content: `
      <h3>应用程序性能问题</h3>
      <p>如果应用程序运行缓慢或崩溃：</p>
      
      <ol>
        <li>检查您的系统资源使用情况</li>
        <li>对于大型APK文件，请确保有足够的内存</li>
        <li>关闭不需要的应用程序以释放系统资源</li>
        <li>尝试重启应用程序</li>
      </ol>
    `
  }
];
</script>

<template>
  <div class="help-content">
    <el-tabs v-model="activeTab" class="help-tabs">
      <el-tab-pane label="使用指南" name="guide">
        <div class="guide-container">
          <el-collapse>
            <el-collapse-item v-for="(guide, index) in guideContent" :key="index" :title="guide.title">
              <template #title>
                <div class="collapse-title">
                  <el-icon><component :is="guide.icon" /></el-icon>
                  <span>{{ guide.title }}</span>
                </div>
              </template>
              <div class="guide-content" v-html="guide.content"></div>
            </el-collapse-item>
          </el-collapse>
        </div>
      </el-tab-pane>
      
      <el-tab-pane label="常见问题" name="faq">
        <div class="faq-container">
          <el-collapse>
            <el-collapse-item v-for="(faq, index) in faqItems" :key="index" :title="faq.question">
              <template #title>
                <div class="collapse-title">
                  <el-icon><QuestionFilled /></el-icon>
                  <span>{{ faq.question }}</span>
                </div>
              </template>
              <div class="faq-answer" v-html="faq.answer"></div>
            </el-collapse-item>
          </el-collapse>
        </div>
      </el-tab-pane>
      
      <el-tab-pane label="故障排除" name="troubleshoot">
        <div class="troubleshoot-container">
          <el-collapse>
            <el-collapse-item v-for="(item, index) in troubleshootItems" :key="index" :title="item.title">
              <template #title>
                <div class="collapse-title">
                  <el-icon><component :is="item.icon" /></el-icon>
                  <span>{{ item.title }}</span>
                </div>
              </template>
              <div class="troubleshoot-content" v-html="item.content"></div>
            </el-collapse-item>
          </el-collapse>
        </div>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style lang="scss" scoped>
.help-content {
  padding: 0;
  
  .help-tabs {
    min-height: 500px;
  }
  
  .collapse-title {
    display: flex;
    align-items: center;
    
    .el-icon {
      margin-right: 8px;
    }
  }
  
  .guide-content, .faq-answer, .troubleshoot-content {
    padding: 10px;
    line-height: 1.6;
    
    h3 {
      margin-top: 0;
      margin-bottom: 16px;
      font-size: 18px;
      color: #333;
    }
    
    h4 {
      margin-top: 16px;
      margin-bottom: 8px;
      font-size: 16px;
      color: #555;
    }
    
    ul, ol {
      padding-left: 20px;
      margin: 10px 0;
    }
    
    li {
      margin-bottom: 8px;
    }
    
    pre {
      background-color: #f5f7fa;
      padding: 10px;
      border-radius: 4px;
      overflow-x: auto;
      font-family: monospace;
    }
  }
  
  .faq-answer {
    background-color: #f5f7fa;
    border-radius: 4px;
  }
  
  @media (max-width: 768px) {
    .help-tabs {
      min-height: 300px;
    }
  }
}
</style> 