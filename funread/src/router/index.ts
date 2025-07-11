import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/desktop/Home/Home.vue'
import { isMobile } from '@/utils/device'

const mobileRoutes = [
  {
    path: '/',
    name: 'home',
    component: () => import('../views/mobile/Home/Home.vue'),
  },
]
const desktopRoutes = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      children: [...desktopRoutes.map((route) => ({ ...route, meta: { device: 'desktop' } }))],
    },
    // {
    //   path: '/m',
    //   children: [...mobileRoutes.map((route) => ({ ...route, meta: { device: 'mobile' } }))],
    // },
  ],
})
router.beforeEach((to, from, next) => {
  const currentDevice = isMobile() ? 'mobile' : 'desktop'
  // 检查路由是否匹配当前设备
  if (to.matched.some((record) => record.meta.device)) {
    const targetDevice = to.meta.device
    // 设备类型匹配则放行
    if (targetDevice === currentDevice) {
      return next()
    }

    // 设备不匹配时跳转到切换页
    return next({
      name: 'notfound',
      query: {
        redirect: to.fullPath,
        // fromDevice: targetDevice,
        // toDevice: currentDevice,
      },
    })
  }

  // 处理根目录重定向
  if (to.path === '/') {
    if (currentDevice === 'mobile') {
      return next('/m/')
    } else {
      return next()
    }
  }

  next()
})
export default router
