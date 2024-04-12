<script setup lang="ts">
type Props = {
  title: string;
};

defineProps<Props>();
</script>

<template>
  <div class="flex flex-col select-none">
    <slot></slot>
    <label class="text-[10px] pl-1">{{ title }}</label>
  </div>
</template>

<style scoped></style>
