import TauriSocket from '@tauri-apps/plugin-websocket';

import { error, info } from '@/lib/log';
import { isWindowTauri } from '@/lib/utils';

export async function useWebsocket() {
  const wsUrl = `${import.meta.env.VITE_BACKEND_URL}/ws`.replace('http', 'ws');

  const isTauri = isWindowTauri();

  if (isTauri) {
    const socket = await TauriSocket.connect(wsUrl);
    return {
      send: (data: string) => socket.send(data),
      onMessage: socket.addListener,
      close: () => socket.disconnect(),
    };
  } else {
    const ws = await new Promise<WebSocket>((resolve, reject) => {
      const socket = new WebSocket(wsUrl);

      socket.onopen = (e) => {
        info(e as unknown as string);
        resolve(socket);
      };

      socket.onclose = (e) => {
        info(e as unknown as string);
        reject(e);
      };

      socket.onerror = (e) => {
        error(e as unknown as string);
        reject(e);
      };
    });
    return {
      send: (data: string) => ws.send(data),
      onMessage: (fn: (msg: string) => void) => (ws.onmessage = (e) => fn(e.data as string)),
      close: () => ws.close(),
    };
  }
}
