<script setup lang="ts"></script>

<template>
  <div class="flex flex-col w-full h-full p-4 gap-y-4">
    <div class="flex justify-between">
      <Status />
      <Hotkey />
    </div>
    <div class="flex flex-col gap-y-3">
      <SectionContainer title="Interval Options">
        <Interval />
      </SectionContainer>
      <div class="flex gap-x-3 h-full">
        <SectionContainer title="Window Detection">
          <LockableWindows />
        </SectionContainer>
        <SectionContainer title="Press Type" class="w-full">
          <ClickOptions />
        </SectionContainer>
      </div>
      <SectionContainer title="Repeat Amount" class="w-full">
        <ClickRepeat />
      </SectionContainer>
    </div>
  </div>
</template>

<style scoped></style>
