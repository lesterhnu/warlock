<template>
  <div class="w-[1440px] mx-auto px-8">
    <header class="py-6 flex items-center justify-between">
      <div class="text-2xl font-['Pacifico'] text-primary">logo</div>
      <nav class="flex items-center gap-8">
        <a href="#" class="text-gray-600 hover:text-primary">首页</a>
        <a href="#" class="text-gray-600 hover:text-primary">我的收藏</a>
        <a href="#" class="text-gray-600 hover:text-primary">标签管理</a>
      </nav>
    </header>
    <div class="mt-12 mb-16 text-center">
      <h1 class="text-4xl font-bold mb-4">发现精彩书摘</h1>
      <p class="text-gray-600 mb-8">记录阅读时刻，分享读书感悟</p>
      <div class="relative w-[720px] mx-auto">
        <div class="relative">
          <i class="fas fa-search absolute left-4 top-1/2 -translate-y-1/2 text-gray-400 text-sm"></i>
          <input v-model="searchText"
            class="w-full h-12 pl-12 pr-4 rounded-full border border-gray-300 focus:outline-none focus:border-primary"
            placeholder="搜索书摘、书名或标签">
        </div>
      </div>
      <div class="flex flex-wrap justify-center gap-2 mt-6">
        <span v-for="tag in tags" :key="tag"
          class="px-3 py-1 rounded-full text-primary border border-primary cursor-pointer hover:bg-primary hover:text-white transition-colors">
          {{ tag }}
        </span>
      </div>
    </div>
    <div class="mb-8 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <div class="relative" v-for="(dropdown, index) in dropdowns" :key="index">
          <button @click="toggleDropdown(index)"
            class="flex items-center px-4 py-2 border border-gray-300 rounded-full bg-white hover:border-primary !rounded-button whitespace-nowrap">
            {{ dropdown.label }}
            <i class="fas fa-chevron-down ml-2 text-xs"></i>
          </button>
          <div v-if="dropdown.isOpen"
            class="absolute top-full left-0 mt-1 w-40 bg-white border border-gray-200 rounded-lg shadow-lg z-10">
            <div v-for="item in dropdown.items" :key="item" class="px-4 py-2 hover:bg-gray-50 cursor-pointer">
              {{ item }}
            </div>
          </div>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <div class="border border-gray-300 rounded-lg overflow-hidden flex">
          <button class="px-3 py-2 hover:bg-gray-50 !rounded-button whitespace-nowrap"
            :class="{ 'bg-gray-50': viewMode === 'grid' }" @click="viewMode = 'grid'">
            <i class="fas fa-th-large"></i>
          </button>
          <button class="px-3 py-2 hover:bg-gray-50 !rounded-button whitespace-nowrap"
            :class="{ 'bg-gray-50': viewMode === 'list' }" @click="viewMode = 'list'">
            <i class="fas fa-bars"></i>
          </button>
        </div>
      </div>
    </div>
    <div class="card-grid">
      <div v-for="(book, index) in books" :key="index"
        class="bg-white rounded-lg border border-gray-200 shadow hover:shadow-lg transition-shadow aspect-[2.5/3.5] flex flex-col">
        <div :class="['flex items-start justify-between p-4 rounded-t-lg', book.headerClass]">
          <div>
            <h3 :class="['text-sm font-medium mb-1', book.titleColor]">《{{ book.title }}》</h3>
            <p class="text-xs text-gray-500">{{ book.author }}</p>
          </div>
          <button class="text-gray-400 hover:text-primary">
            <i class="fas fa-star"></i>
          </button>
        </div>
        <div class="px-4 flex-1 flex flex-col justify-center">
          <p class="text-gray-800 text-base leading-relaxed text-center px-2">{{ book.content }}</p>
          <!-- <div class="flex flex-wrap gap-1.5 py-4">
            <span v-for="tag in book.tags" :key="tag.text" :class="[
            'px-2 py-0.5 rounded-full text-xs',
            getTagClass(tag.type)
          ]">
              {{ tag.text }}
            </span>
          </div> -->
        </div>
      </div>
    </div>
    <div class="mt-12 mb-16 text-center">
      <button
        class="px-6 py-2 border border-gray-300 rounded-full hover:border-primary !rounded-button whitespace-nowrap"
        @click="loadMore">加载更多</button>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { ref } from 'vue';
const searchText = ref('');
const viewMode = ref('grid');
const tags = ['心理学', '哲学', '文学', '历史', '科技', '经济'];
const dropdowns = ref([
  {
    label: '最新发布',
    isOpen: false,
    items: ['最新发布', '最多收藏', '最多评论']
  },
  {
    label: '全部分类',
    isOpen: false,
    items: tags
  }
]);
const toggleDropdown = (index: number) => {
  dropdowns.value.forEach((dropdown, i) => {
    if (i === index) {
      dropdown.isOpen = !dropdown.isOpen;
    } else {
      dropdown.isOpen = false;
    }
  });
};
// const getTagClass = (type: string) => {
//   const classes = {
//     primary: 'bg-blue-50 text-blue-600',
//     success: 'bg-green-50 text-green-600',
//     warning: 'bg-yellow-50 text-yellow-600',
//     danger: 'bg-red-50 text-red-600',
//     info: 'bg-gray-50 text-gray-600'
//   };
//   return classes[type] || classes.primary;
// };
const books = [
  {
    title: '人类简史',
    author: '尤瓦尔·赫拉利',
    content: '人类之所以能够成为地球的统治者，是因为我们是唯一能够大规模合作的物种。这种合作建立在我们对共同故事的信仰之上，比如宗教、国家、金钱等。这些故事既不是客观事实，也不是主观幻想，而是主体间的现实。',
    headerClass: 'bg-blue-50',
    titleColor: 'text-primary',
    tags: [
      { text: '历史', type: 'primary' },
      { text: '人类学', type: 'warning' }
    ]
  },
  {
    title: '认知觉醒',
    author: '周岭',
    content: '大脑就像一块田地，如果不经过精心的耕耘，杂草就会丛生。我们需要通过持续的学习和思考来培育认知能力，就像农夫精心照料庄稼一样。只有这样，我们才能收获丰硕的果实，实现认知的跃迁。',
    headerClass: 'bg-green-50',
    titleColor: 'text-green-600',
    tags: [
      { text: '心理学', type: 'danger' },
      { text: '认知科学', type: 'warning' }
    ]
  },
  {
    title: '置身事内',
    author: '兰小欢',
    content: '中国经济的发展离不开政府的推动作用，但政府的角色并非简单的计划者和管理者。在市场经济中，政府更像是一个引导者和协调者，需要在维护市场秩序和促进经济发展之间找到平衡点。',
    headerClass: 'bg-yellow-50',
    titleColor: 'text-yellow-600',
    tags: [
      { text: '经济', type: 'success' },
      { text: '政治', type: 'info' }
    ]
  },
  {
    title: '沉默的大多数',
    author: '王小波',
    content: '知识分子的责任不仅在于传播知识，更在于保持独立思考的能力。在面对各种社会现象时，我们需要有自己的判断，而不是随波逐流。这种独立思考的能力，是知识分子最宝贵的品质。',
    headerClass: 'bg-purple-50',
    titleColor: 'text-purple-600',
    tags: [
      { text: '文学', type: 'danger' },
      { text: '社会学', type: 'info' }
    ]
  },
  {
    title: '未来简史',
    author: '尤瓦尔·赫拉利',
    content: '在生物技术和人工智能快速发展的今天，人类正面临着前所未有的挑战。我们可能会创造出超越人类的新物种，也可能会彻底改变人类的生存方式。在这个过程中，我们需要慎重思考每一步的选择。',
    headerClass: 'bg-pink-50',
    titleColor: 'text-pink-600',
    tags: [
      { text: '科技', type: 'primary' },
      { text: '未来学', type: 'success' }
    ]
  },
  {
    title: '思考，快与慢',
    author: '丹尼尔·卡尼曼',
    content: '我们的思维系统分为快速系统和慢速系统。快速系统负责直觉反应，而慢速系统则负责深度思考。在日常生活中，我们需要学会平衡这两个系统，既要相信直觉，也要保持理性思考。',
    headerClass: 'bg-orange-50',
    titleColor: 'text-orange-600',
    tags: [
      { text: '心理学', type: 'warning' },
      { text: '决策论', type: 'info' }
    ]
  }
];
const loadMore = () => {
  // 加载更多逻辑
};
</script>
<style scoped>
body {
  min-height: 1024px;
  background-color: #F7F9FC;
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 24px;
}

.text-primary {
  color: #409EFF;
}

/* Remove number input arrows */
input[type=number]::-webkit-inner-spin-button,
input[type=number]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

input[type=number] {
  -moz-appearance: textfield;
}
</style>