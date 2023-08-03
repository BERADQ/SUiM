// https://vitejs.dev/guide/env-and-mode.html#intellisense-for-typescript
/// <reference types="svelte" />
/// <reference types="vite/client" />
/// <reference types="unocss/vite" />
interface ImportMetaEnv {
  TAURI_PLATFORM: import('./common').OS
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}