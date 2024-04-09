<script setup lang="ts">
import { useSettingStore } from "../../stores/useSettingsStore";

const settingsStore = useSettingStore();
</script>

<template>
  <div
    class="w-fit h-fit rounded-[4px] px-4 py-1 select-none"
    :class="
      settingsStore.settings.isActive ? 'bg-green-900/50' : 'bg-red-900/50'
    "
  >
    <span
      >Status:
      {{ settingsStore.settings.isActive ? "Running" : "Stopped" }}</span
    >
  </div>
</template>

<style scoped></style>
