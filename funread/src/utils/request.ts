import axios from 'axios'
import {
  type AxiosInstance,
  type AxiosResponse,
  type AxiosError,
  type AxiosRequestConfig,
  AxiosHeaders,
} from 'axios'
import { ref } from 'vue'

// 定义基础响应类型
interface BaseResponse<T = any> {
  code: number
  message: string
  data?: T
}

// 定义请求配置类型
interface RequestConfig extends AxiosRequestConfig {
  showLoading?: boolean
  showError?: boolean
  headers: AxiosHeaders
}

// 创建请求实例
const createRequest = (baseConfig?: RequestConfig) => {
  const instance: AxiosInstance = axios.create({
    baseURL: import.meta.env.VITE_API_BASE_URL || '/api',
    timeout: 10000,
    ...baseConfig,
  })

  // 请求拦截器
  instance.interceptors.request.use(
    (config: RequestConfig) => {
      // 可以在这里添加全局请求逻辑，如添加 token
      // const token = localStorage.getItem('token')
      // if (token) {
      //   config.headers = config.headers || {}
      //   config.headers.Authorization = `Bearer ${token}`
      // }

      // 显示加载状态
      if (config.showLoading !== false) {
        // 这里可以使用 Vue 的全局状态管理或自定义逻辑
        console.log('Loading...')
      }

      return config
    },
    (error: AxiosError) => {
      return Promise.reject(error)
    },
  )

  // 响应拦截器
  instance.interceptors.response.use(
    (response: AxiosResponse) => {
      // 隐藏加载状态
      console.log('Loading finished')

      // 处理业务逻辑错误
      if (response.data.code !== 0) {
        console.error(response.data.message || '请求失败')
        return Promise.reject(response.data)
      }

      return response.data
    },
    (error: AxiosError) => {
      // 隐藏加载状态
      console.log('Loading finished')

      // 处理 HTTP 错误
      if (error.response) {
        switch (error.response.status) {
          case 401:
            console.error('未授权，请登录')
            // 可以跳转到登录页
            break
          case 403:
            console.error('拒绝访问')
            break
          case 404:
            console.error('请求地址不存在')
            break
          case 500:
            console.error('服务器内部错误')
            break
          default:
            console.error('未知错误')
        }
      } else if (error.request) {
        console.error('请求未响应')
      } else {
        console.error('请求设置错误')
      }

      return Promise.reject(error)
    },
  )

  // 封装请求方法
  const request = <T = any>(config: RequestConfig): Promise<T> => {
    return instance.request(config)
  }

  // 封装 GET 方法
  const get = <T = any>(url: string, params?: any, config?: RequestConfig): Promise<T> => {
    return instance
      .get<T>(url, { ...config, params: params || undefined })
      .then((response) => response.data)
  }

  // 封装 POST 方法
  const post = <T = any>(url: string, data?: any, config?: RequestConfig): Promise<T> => {
    return instance
      .post<T>(url, data)
      .then((response) => response.data)
  }

  // 封装 PUT 方法
  const put = <T = any>(url: string, data?: any, config?: RequestConfig): Promise<T> => {
    return instance
      .put<T>(url, { ...config, params: data || undefined })
      .then((response) => response.data)
  }

  // 封装 DELETE 方法
  const del = <T = any>(url: string, config?: RequestConfig): Promise<T> => {
    return instance.delete<T>(url, { ...config }).then((response) => response.data)
  }

  return {
    instance,
    request,
    get,
    post,
    put,
    delete: del,
  }
}

// 创建默认请求实例
const { instance, request, get, post, put, delete: del } = createRequest()

// 导出请求方法
export { instance, request, get, post, put, del, createRequest }
export type { BaseResponse, RequestConfig }
