import { defineConfig } from 'vite'
import UnoCSS from 'unocss/vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'

import Components from 'unplugin-vue-components/vite'
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'

import AutoImport from 'unplugin-auto-import/vite'
import Pages from 'vite-plugin-pages'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
    vueJsx(),
    UnoCSS(),

    Components({
      dirs: ['src'],
      resolvers: [
        NaiveUiResolver(),
      ],
      dts: 'src/typings/components.d.ts',
    }),

    AutoImport({
      imports: [
        'vue',
        'vue-router',
        'pinia',
        {
          '@tauri-apps/api/app': ['getName', 'getVersion', 'getTauriVersion'],
          '@tauri-apps/api/shell': ['Command'],
          '@tauri-apps/api/os': ['platform'],
          '@tauri-apps/api/notification': ['sendNotification', 'requestPermission', 'isPermissionGranted'],
        },
      ],
      dts: 'src/typings/auto-imports.d.ts',
    }),

    Pages({
      dirs: ['src/views'],
      extensions: ['vue', 'tsx'],

      extendRoute(route) {
        const path = route.path.replace(/^\//, '')
        const meta = route.meta || {}

        meta.layout = meta.layout || 'default'

        return { ...route, path, meta }
      },

    }),
  ],

  clearScreen: false,

  server: {
    port: 5173,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },

  resolve: {
    alias: {
      '@': '/src',
    },
  },
}))
