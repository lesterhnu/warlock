<template>
  <div class="w-[1440px] mx-auto px-8">
    <header class="py-6 flex items-center  justify-between  ">
      <div>
        FunRead
      </div>
      
      <div class="flex items-center gap-4">
        <label class="input w-[350px]">
          <svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor">
              <circle cx="11" cy="11" r="8"></circle>
              <path d="m21 21-4.3-4.3"></path>
            </g>
          </svg>
          <input type="search" v-model="searchText" required placeholder="Search" />
        </label>
        <button class="btn btn-primary" @click="search">Search</button>
      </div>
    </header>
    <div class="divider top-0 left-0 rigth-0"></div>
    <div class="card-grid">
      <div v-for="(book, index) in books" :key="index"
        class="bg-white rounded-lg border border-gray-200 shadow hover:shadow-lg transition-shadow aspect-[2.5/3.5] flex flex-col">
        <div :class="[
          'flex items-start justify-between p-4 rounded-t-lg',
          headerClass[get_random(0, 5)],
        ]">
          <div>
            <h3 :class="['text-sm font-medium mb-1', titleColor[get_random(0, 5)]]">
              《{{ book.title }}》
            </h3>
            <p class="text-xs text-gray-500">--{{ book.author }}</p>
          </div>
          <!-- <button class="text-gray-400 hover:text-primary">
            <i class="fas fa-star"></i>
          </button> -->
        </div>
        <div class="px-4 flex-1 flex flex-col justify-center">
          <p
            class="text-gray-800 flex-grow  text-base font-serif leading-10 tracking-widest text-lg indent-8  flex flex-col justify-center">
            {{ book.content }}
          </p>
          <!-- <div class="flex flex-wrap gap-1.5 py-4">
            <span
              v-for="tag in book.tags"
              :key="tag.text"
              :class="['px-2 py-0.5 rounded-full text-xs', getTagClass(tag.type)]"
            >
              {{ tag.text }}
            </span>
          </div> -->
          <!-- <div class="flex flex-wrap gap-1.5 py-4" >
            <span :class="['px-2 py-0.5 rounded-full text-xs', tagClasses[get_random(0, 4)]]">
              毛选
            </span>
          </div> -->
        </div>
      </div>
    </div>
    <!-- <div class="mt-12 mb-16 text-center">
      <button
        class="px-6 py-2 border border-gray-300 rounded-full hover:border-primary !rounded-button whitespace-nowrap"
        @click="loadMore"
      >
        加载更多
      </button>
    </div> -->
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { get, post } from '@/utils/request'
import { get_random } from '@/utils/random'
interface Posts {
  id: number
  title: string
  author: string
  content: string
  chapter: string
  bg: string
  create_time: string
}
onMounted(() => {
  fetchPosts()
})
const books = ref<Posts[]>([])
const searchText = ref('')
const viewMode = ref('grid')
const tags = ['心理学', '哲学', '文学', '历史', '科技', '经济']
const headerClass = [
  'bg-blue-50',
  'bg-green-50',
  'bg-yellow-50',
  'bg-purple-50',
  'bg-pink-50',
  'bg-orange-50',
]
const titleColor = [
  'text-blue-600',
  'text-green-600',
  'text-yellow-600',
  'text-purple-600',
  'text-pink-600',
  'text-orange-600',
]

const tagClasses = [
  'bg-blue-50 text-blue-600',
  'bg-green-50 text-green-600',
  'bg-yellow-50 text-yellow-600',
  'bg-red-50 text-red-600',
  'bg-gray-50 text-gray-600',
]
const fetchPosts = async () => {
  try {
    const response = await get('/book/get_books')
    books.value = response
  } catch (error) {
    console.error('Error fetching posts:', error)
  }
}
const searchBooks = async (searchContent: string) => {
  try {
    const response = await post('/book/search', { content: searchContent })
    books.value = response
  } catch (error) {
    console.error('Error searching books:', error)
  }
}
const search = () => {
  if (searchText.value.trim() === '') {
    alert('请输入搜索内容')
    return
  }
  // 执行搜索逻辑
  console.log('Searching for:', searchText.value)
  searchBooks(searchText.value)
  // 可以在这里调用 API 或进行其他操作
}
const getTagClass = (type: string) => {
  const classes: { [key: string]: string } = {
    primary: 'bg-blue-50 text-blue-600',
    success: 'bg-green-50 text-green-600',
    warning: 'bg-yellow-50 text-yellow-600',
    danger: 'bg-red-50 text-red-600',
    info: 'bg-gray-50 text-gray-600',
  }
  return classes[type] || classes.primary
}

const loadMore = () => {
  // 加载更多逻辑
}
</script>

<style scoped>
body {
  min-height: 1024px;
  background-color: #f7f9fc;
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 24px;
}

.text-primary {
  color: #409eff;
}

/* Remove number input arrows */
input[type='number']::-webkit-inner-spin-button,
input[type='number']::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

input[type='number'] {
  -moz-appearance: textfield;
}
</style>
