import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { sendNotification } from '@tauri-apps/plugin-notification';
import { defineStore } from 'pinia';
import { toast } from 'vue-sonner';

import { isWindowTauri } from '@/lib/utils';

export type NotificationSeverity = 'success' | 'info' | 'warning' | 'error' | 'message';

export const useWindowStore = defineStore('window-store', () => {
  const isTauri = isWindowTauri();
  const isActive = ref(true);
  const isLoading = ref(true);

  const window = isTauri ? getCurrentWindow() : null;

  window?.onFocusChanged(({ payload }) => {
    isActive.value = payload;
  });

  const notify = ({
    title,
    body,
    severity = 'message',
  }: {
    title: string;
    body?: string;
    severity?: NotificationSeverity;
  }) => {
    if (isTauri && !isActive.value) {
      sendNotification({
        title,
        body,
      });
    } else {
      toast[severity](title);
    }
  };

  return {
    window,
    isActive,
    isLoading,
    isTauri,
    notify,
  };
});
