<script setup lang="ts">
import { NText } from 'naive-ui';
import { Calculate } from './components/calculate';
import AppProvider from './components/common/app-provide.vue';

onMounted(() => {
  window.$dialog?.info({
    title: "欢迎使用 La-Calculate",
    content: () => h('div', [
      h(NText, {delete: true}, { default: () => 'La-Calculate 是一个用于计算角色刻印搭配的工具，帮助你省钱搭配刻印。' }),
      h("br"),
      h(NText, {type: "warning"}, { default: () => '已经被T4背刺了' }),
    ]),
  })
})
</script>

<template>
  <NConfigProvider class="h-full">
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
