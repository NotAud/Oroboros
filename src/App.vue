<script setup lang="ts"></script>

<template>
  <div class="flex flex-col w-full h-full p-4 gap-y-4">
    <div class="flex justify-between">
      <Status />
      <Hotkey />
    </div>
    <SectionContainer title="Settings">
      <div class="flex flex-col gap-y-6">
        <Interval />
        <LockableWindows />
      </div>
    </SectionContainer>
  </div>
</template>

<style scoped></style>
