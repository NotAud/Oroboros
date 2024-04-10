<script setup lang="ts">
import { useSettingStore } from "../../stores/useSettingsStore";

const settingsStore = useSettingStore();
</script>

<template>
  <div
    class="flex items-center space-x-2 bg-green-400/20 px-5 rounded-[4px] shd select-none"
    :class="
      settingsStore.settings.isActive ? 'bg-green-400/20' : 'bg-red-400/20'
    "
  >
    <svg
      v-if="settingsStore.settings.isActive"
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
      class="h-4 w-4 text-green-400"
    >
      <polygon points="5 3 19 12 5 21 5 3"></polygon>
    </svg>
    <svg
      v-else
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 384 512"
      class="fill-red-400"
    >
      <path
        d="M0 128C0 92.7 28.7 64 64 64H320c35.3 0 64 28.7 64 64V384c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V128z"
      />
    </svg>
    <span class="text-white">{{
      settingsStore.settings.isActive ? "Running" : "Stopped"
    }}</span>
  </div>
</template>

<style scoped></style>
