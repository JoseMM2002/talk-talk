<script setup lang="ts">
import { DefaultFooter } from './footer';
import { DefaultHeader } from './header';
import { DefaultSidebar } from './sidebar';
</script>
<template>
  <div class="flex flex-col w-screen h-screen bg-emphasis @container/main">
    <DefaultHeader />
    <main class="flex flex-1">
      <DefaultSidebar />
      <RouterView />
    </main>
    <DefaultFooter />
  </div>
</template>
