<script setup lang="ts">
import { useSettingStore } from "../../stores/useSettingsStore";

const settingsStore = useSettingStore();

function maxGuard(num: number) {
  let value = num;
  if (num > settingsStore.settings.randomizedSpeed.to) {
    value = settingsStore.settings.randomizedSpeed.to;
  }

  return value;
}

function minGuard(num: number) {
  let value = num;
  if (num < settingsStore.settings.randomizedSpeed.from) {
    value = settingsStore.settings.randomizedSpeed.from;
  }

  return value;
}
</script>

<template>
  <div class="flex gap-x-6 items-center">
    <LabelContainer title="Randomize">
      <SwitchInput v-model="settingsStore.settings.isRandomized" />
    </LabelContainer>
    <LabelContainer
      v-if="!settingsStore.settings.isRandomized"
      title="Click Interval (ms)"
    >
      <NumberInput
        v-model="settingsStore.settings.interval"
        aria-disabled="true"
      />
    </LabelContainer>
    <template v-else>
      <LabelContainer title="From (ms)">
        <NumberInput
          v-model="settingsStore.settings.randomizedSpeed.from"
          :customGuard="maxGuard"
          aria-disabled="true"
        />
      </LabelContainer>
      <LabelContainer title="To (ms)">
        <NumberInput
          v-model="settingsStore.settings.randomizedSpeed.to"
          :customGuard="minGuard"
          aria-disabled="true"
        />
      </LabelContainer>
    </template>
  </div>
</template>

<style scoped></style>
