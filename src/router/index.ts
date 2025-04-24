import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/HomePage.vue'
import Help from '../views/HelpPage.vue'
import Settings from '../views/Settings.vue'
import History from '../views/HistoryPage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/help',
      name: 'help',
      component: Help
    },
    {
      path: '/settings',
      name: 'settings',
      component: Settings
    },
    {
      path: '/history',
      name: 'history',
      component: History
    }
  ]
})

export default router 