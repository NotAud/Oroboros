<script setup lang="ts"></script>

<template>
  <div class="flex flex-col w-full h-full p-4 gap-y-6">
    <div class="flex justify-between">
      <Status />
      <Hotkey />
    </div>
    <div class="flex flex-col gap-y-3">
      <SectionContainer title="Interval Options">
        <Interval />
      </SectionContainer>
      <SectionContainer title="Window Detection" class="w-fit">
        <LockableWindows />
      </SectionContainer>
    </div>
  </div>
</template>

<style scoped></style>
