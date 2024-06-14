import antfu from '@antfu/eslint-config'

export default antfu(
  {
    ignores: ['src-tauri/**/*', 'src/vite-env.d.ts'],
  },
)
