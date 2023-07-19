declare interface Figure {
  name: string
  avatar: string
  describes: string
}

declare module "$env/static/public" {
  export const TAURI_PLATFORM: string
}
