<script setup lang="tsx">
import type { NDateLocale, NLocale } from 'naive-ui'
import { NCheckbox, NDivider, NH2, NText, dateZhCN, zhCN } from 'naive-ui'
import hljs from 'highlight.js/lib/core'
import json from 'highlight.js/lib/languages/json'
import { Calculate } from './components/calculate'
import AppProvider from './components/common/app-provide.vue'

const locale = ref<NLocale>(zhCN)
const dateLocale = ref<NDateLocale>(dateZhCN)

onMounted(() => {
  hljs.registerLanguage('json', json)

  window.$dialog?.info({
    title: '欢迎使用 La-Calculate',
    content: () =>
      (
        <>
          <NText delete>La-Calculate 是一个用于计算角色刻印搭配的工具，帮助你省钱搭配刻印。</NText>
          <br />
          <NText type="warning">已经被T4背刺了</NText>
          <NDivider />
          <div class="flex flex-col">
            <NH2>未来规划</NH2>
            <NCheckbox disabled>支持负面刻印</NCheckbox>
            <NCheckbox disabled>多语言支持</NCheckbox>
            <NCheckbox disabled>本地数据库持久化</NCheckbox>
          </div>
        </>
      ),
  })
})
</script>

<template>
  <NConfigProvider :locale="locale" :date-locale="dateLocale" class="h-full" :hljs="hljs">
    <AppProvider>
      <n-space vertical size="large">
        <n-layout class="h-full">
          <n-layout-header class="flex-c">
            <svg viewBox="400 0 400 200">
              <text x="50%" y="70%" width="100%" height="100%"> La-Calculate </text>
            </svg>
          </n-layout-header>
          <n-layout-content class="h-full" content-style="padding: 24px;">
            <Calculate />
          </n-layout-content>
          <n-layout-footer class="flex-c">
            <p>CopyRight © 2024 Kabuda-czh All Rights Reserved</p>
          </n-layout-footer>
        </n-layout>
      </n-space>
    </AppProvider>
  </NConfigProvider>
</template>

<style lang="scss">
svg {
  width: 100%;
  height: 100px;
  margin: auto;
}
svg text {
  text-transform: uppercase;
  animation: stroke 5s infinite alternate;
  letter-spacing: 10px;
  font-size: 90px;
}
@keyframes stroke {
  0% {
    fill: rgba(72, 138, 20, 0);
    stroke: rgba(54, 95, 160, 1);
    stroke-dashoffset: 25%;
    stroke-dasharray: 0 50%;
    stroke-width: 0.8;
  }
  50% {
    fill: rgba(72, 138, 20, 0);
    stroke: rgba(54, 95, 160, 1);
    stroke-width: 1.2;
  }
  70% {
    fill: rgba(72, 138, 20, 0);
    stroke: rgba(54, 95, 160, 1);
    stroke-width: 1.5;
  }
  90%,
  100% {
    fill: rgba(72, 138, 204, 1);
    stroke: rgba(54, 95, 160, 0);
    stroke-dashoffset: -25%;
    stroke-dasharray: 50% 0;
    stroke-width: 0;
  }
}
</style>
