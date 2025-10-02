import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { createPinia } from 'pinia'
import App from './App.vue'
import './style.css'

const app = createApp(App)

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Home',
      component: () => import('./views/HomeView.vue')
    },
    {
      path: '/executions/:id',
      name: 'ExecutionDetail',
      component: () => import('./views/ExecutionDetailView.vue')
    }
  ]
})

app.use(createPinia())
app.use(router)

app.mount('#app')
