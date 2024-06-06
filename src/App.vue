<script setup lang="ts">
import { Calculate } from './components/calculate';
import AppProvider from './components/common/app-provide.vue';
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
