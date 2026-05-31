/// <reference types="vite/client" />

declare module '*.css' {
  const content: string
  export default content
}

declare module '@xterm/xterm/css/xterm.css' {
  const content: string
  export default content
}

// Tauri plugins without bundled types
declare module '@tauri-apps/process' {
  export function relaunch(): Promise<void>
}
