<script setup lang="ts">
type Props = {
  title: string;
};

defineProps<Props>();
</script>

<template>
  <div class="flex flex-col select-none w-fit">
    <label class="text-[12px] pl-1">{{ title }}</label>
    <slot></slot>
  </div>
</template>

<style scoped></style>
