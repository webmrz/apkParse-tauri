import { createApp } from "vue";
import { createPinia } from "pinia";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
// import { Icon } from "@iconify/vue";
import router from "./router";
import App from "./App.vue";

const app = createApp(App);

try {
  // 注册所有Element Plus图标
  for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component);
  }

  // 注册Iconify图标组件
  // app.component("IconifyIcon", Icon);

  app.use(createPinia());
  app.use(ElementPlus);
  app.use(router);

  app.mount("#app");
} catch (error) {
  console.error('Error during app initialization:', error);
}
