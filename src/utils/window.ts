import { appWindow, LogicalSize } from '@tauri-apps/api/window';

export function setWindowSize() {
  const { height } = document.body.getBoundingClientRect();
  void appWindow.setSize(new LogicalSize(400, height));
}
