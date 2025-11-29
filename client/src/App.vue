<script setup lang="ts">
import { computed, ref } from 'vue';
import { useColorMode } from '@vueuse/core';
import type { Theme } from 'vue-sonner/src/packages/types.js';

import { PullWindow, Toaster } from './components/ui';

import 'vue-sonner/style.css';

const { value: colorValue } = useColorMode();

const refreshKey = ref(0);

const theme = computed<Theme>(() => {
  const themeVal =
    {
      auto: 'system',
    }[colorValue as string] || colorValue;
  return themeVal as Theme;
});

const loading = ref(false);

const refreshPage = async () => {
  refreshKey.value++; // triggers RouterView remount
  await new Promise((resolve) => setTimeout(resolve, 2000));
  loading.value = false;
};
</script>

<template>
  <PullWindow v-model:loading="loading" :after-pull="refreshPage">
    <div class="text-sm">
      <RouterView :key="refreshKey" />
      <Toaster rich-colors :theme="theme" position="top-center" />
    </div>
  </PullWindow>
</template>
