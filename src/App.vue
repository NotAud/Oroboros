<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
async function test() {
  let list = await invoke<string[]>("window_handles");
  console.log(list);
}
</script>

<template>
  <div class="flex flex-col w-full h-full p-4 gap-y-4">
    <div class="flex justify-between">
      <Status />
      <Hotkey />
    </div>
    <SectionContainer title="Settings">
      <button @click="test">Test</button>
      <Interval />
      <LockableWindows />
    </SectionContainer>
  </div>
</template>

<style scoped></style>
