<script setup lang="ts">
import { useColorMode } from '@vueuse/core';
useColorMode();
</script>

<template>
  <div class="text-sm">
    <RouterView />
  </div>
</template>
