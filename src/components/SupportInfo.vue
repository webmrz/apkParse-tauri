<template>
  <div class="support-container">
    <div class="support-cards">
      <!-- 公众号二维码 -->
      <el-card class="qr-card">
        <template #header>
          <div class="card-header">
            <span>关注微信公众号</span>
          </div>
        </template>
        <div class="qr-wrapper">
          <el-image 
            src="/images/wechat-official.png" 
            alt="微信公众号"
            :preview-src-list="['/images/wechat-official.png']"
            fit="contain"
            class="qr-image"
          />
          <div class="qr-desc">扫码关注公众号获取更多内容</div>
        </div>
      </el-card>

      <!-- 赞赏码 -->
      <el-card class="qr-card">
        <template #header>
          <div class="card-header">
            <span>赞赏支持</span>
          </div>
        </template>
        <div class="qr-wrapper">
          <el-image 
            src="/images/donation.png" 
            alt="赞赏码"
            :preview-src-list="['/images/donation.png']"
            fit="contain"
            class="qr-image"
          />
          <div class="qr-desc">如果觉得有帮助可以请作者喝杯咖啡</div>
        </div>
      </el-card>
    </div>
  </div>
</template>

<script setup lang="ts">
// No additional script needed
</script>

<style scoped lang="scss">
.support-container {
  margin: 2rem 0;
}

.support-cards {
  display: flex;
  gap: 20px;
  flex-wrap: wrap;
  justify-content: center;
}

.qr-card {
  width: 250px;
  
  .card-header {
    display: flex;
    justify-content: center;
    font-weight: 500;
  }
  
  .qr-wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    
    .qr-image {
      width: 200px;
      height: 200px;
      cursor: pointer;
    }
    
    .qr-desc {
      font-size: 0.9rem;
      color: var(--el-text-color-secondary);
      text-align: center;
    }
  }
}

/* 优化预览样式 */
:deep(.el-image-viewer__wrapper) {
  z-index: 3000;
}

:deep(.el-image-viewer__img) {
  max-width: 80vw;
  max-height: 80vh;
}
</style> 