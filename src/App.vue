<script setup lang="tsx">
import type { NDateLocale, NLocale } from 'naive-ui'
import { NText, dateZhCN, zhCN } from 'naive-ui'
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
        <div>
          <NText delete>La-Calculate 是一个用于计算角色刻印搭配的工具，帮助你省钱搭配刻印。</NText>
          <br />
          <NText type="warning">已经被T4背刺了</NText>
        </div>
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
            <p>欢迎使用 La-Calculate</p>
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

<style lang="scss"></style>
