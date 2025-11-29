<script setup lang="ts">
import { computed, onMounted, ref, type StyleValue } from 'vue';
import { refDebounced } from '@vueuse/core';
import { Loader2Icon } from 'lucide-vue-next';

import { cn } from '@/lib/utils';

type Props = {
  maxPull?: number;
  minPull?: number;
  afterPull?: () => Promise<void>;
};

const loading = defineModel<boolean>('loading', { type: Boolean, required: true });
let startY: number;
const props = withDefaults(defineProps<Props>(), {
  maxPull: 60,
  minPull: 40,
});

const pulling = ref(false);
const pullRef = ref<HTMLDivElement>();
const textRef = ref<HTMLDivElement>();
const pullDistance = ref(0);

const afterLoading = refDebounced(loading, 300);
const finishedLoading = computed(() => !loading.value || !afterLoading.value);

const rotation = computed(() => {
  if (loading.value) return undefined;
  return pullDistance.value * 18; // 1px pull = 10deg rotation
});

const containerStyle = computed<StyleValue>(() => ({
  height: `${pullDistance.value}px`,
  transition: pulling.value ? 'none' : 'height 0.3s cubic-bezier(0,0,0.2,1)',
  overflow: 'hidden',
}));

const iconStyle = computed<StyleValue>(() => ({
  opacity: Math.min(pullDistance.value / (props.minPull * 0.7), 1), // Fade in faster than pull
  transform: `rotate(${rotation.value}deg)`,
  // Smooth rotation on release/load, instant on pull
  transition: pulling.value ? 'none' : 'transform 0.3s linear',
}));

onMounted(() => {
  if (!pullRef.value) return;

  pullRef.value.addEventListener('touchstart', (e) => {
    if (loading.value || loading.value === undefined) return;
    pulling.value = true;
    const t = e.touches[0];
    if (!t) return;
    startY = t.clientY;
  });

  pullRef.value.addEventListener('touchmove', (e) => {
    if (loading.value) return;
    const t = e.touches[0];
    if (!t) return;
    pullDistance.value = Math.max(Math.min(t.clientY - startY, props.maxPull), 0);
    if (pullDistance.value > 0) e.preventDefault();
  });

  pullRef.value.addEventListener('touchend', async () => {
    loading.value = true;
    pulling.value = false;
    await props.afterPull?.();
    pullDistance.value = 0;
  });
});
</script>

<template>
  <div class="overflow-hidden relative" ref="pullRef">
    <div
      :style="containerStyle"
      class="flex absolute right-0 left-0 justify-center items-center p-2"
      ref="textRef"
    >
      <Loader2Icon
        v-if="pulling || loading || afterLoading"
        role="status"
        :style="iconStyle"
        :class="
          cn('size-4', {
            ['animate-spin']: !finishedLoading,
          })
        "
      />
    </div>
    <slot />
  </div>
</template>
