<script setup lang="ts">
type Props = {
  title: string;
};

defineProps<Props>();
</script>

<template>
  <div class="flex flex-col w-full h-full">
    <span class="pl-1 select-none">{{ title }}</span>
    <div class="bg-white/5 shd rounded-md p-2">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped></style>
