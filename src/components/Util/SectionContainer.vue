<script setup lang="ts">
type Props = {
  title: string;
};

defineProps<Props>();
</script>

<template>
  <div class="flex flex-col relative">
    <div class="absolute left-2 top-2">
      <span class="select-none">{{ title }}</span>
    </div>
    <div class="bg-white/5 shd rounded-md px-3 pb-2 pt-10">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped></style>
