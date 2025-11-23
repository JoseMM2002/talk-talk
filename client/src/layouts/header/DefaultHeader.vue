<script setup lang="ts">
import { Inbox } from 'lucide-vue-next';

import { Button } from '@/components/ui';
</script>
<template>
  <header class="flex px-4 py-1 justify-between align-center @max-2xl/main:hidden">
    <div />
    <div class="flex flex-col justify-center">
      <RouterView name="header-title" />
    </div>
    <Button size="icon-sm" variant="ghost">
      <Inbox />
    </Button>
  </header>
</template>
