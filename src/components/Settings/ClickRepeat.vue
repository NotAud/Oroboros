<script setup lang="ts">
import { useSettingStore } from "../../stores/useSettingsStore";

const settingsStore = useSettingStore();
</script>

<template>
  <div class="flex flex-col">
    <div class="flex items-center gap-x-1">
      <RadioInput v-model="settingsStore.settings.clickRepeater" value="finite">
        <div class="flex items-center gap-x-4">
          <span>Repeat </span>
          <NumberInput v-model="settingsStore.settings.repeatAmount" />
          <span>Times</span>
        </div>
      </RadioInput>
    </div>
    <div class="flex items-center">
      <RadioInput
        v-model="settingsStore.settings.clickRepeater"
        value="infinite"
        text="Repeat until stopped"
      />
    </div>
  </div>
</template>

<style scoped></style>
