<script setup lang="ts">
import { useSettingStore } from "../../stores/useSettingsStore";

const settingsStore = useSettingStore();

const hotkeyButton = ref<HTMLButtonElement | null>(null);
const isCapturing = ref(false);
const recordingKeys = ref<string[]>([]);

const previousHotkey = ref<string[]>([]);
const hotkey = ref<string[]>([]);

const hotkeyDisplay = computed(() => {
  if (isCapturing.value) {
    if (recordingKeys.value.length > 0) {
      return recordingKeys.value.join(" + ").toUpperCase();
    }

    return "Recording...";
  }

  if (hotkey.value.length > 0) {
    return hotkey.value.join(" + ").toUpperCase();
  }

  return "Set Hotkey";
});

async function handleHotkeyPress(e: KeyboardEvent) {
  e.preventDefault();

  if (!isCapturing.value) {
    return;
  }

  if (e.key === "Escape") {
    resetRecording();
    return;
  }

  const allowedSingleKeys =
    "abcdefghijklmnopqrstuvwxyz1234567890/.,';\\][=-`F1F2F3F4F5F6F7F8F9F10F11F12";
  const comboKeys = ["Control", "Shift"];

  if (
    recordingKeys.value.length === 0 &&
    !allowedSingleKeys.includes(e.key) &&
    !comboKeys.includes(e.key)
  ) {
    abortRecording();
    return;
  }

  if (recordingKeys.value.includes(e.key)) {
    return;
  }

  if (comboKeys.includes(e.key)) {
    if (recordingKeys.value.length < 1) {
      recordingKeys.value.push(e.key);
    }
  } else {
    hotkey.value = [...recordingKeys.value, e.key];
    recordingKeys.value = [];
    isCapturing.value = false;
    saveHotkey();
  }
}

async function handleHotkeyRelease(e: KeyboardEvent) {
  e.preventDefault();

  if (!isCapturing.value) {
    return;
  }

  const keyIndex = recordingKeys.value.indexOf(e.key);
  if (keyIndex > -1) {
    recordingKeys.value.splice(keyIndex, 1);
  }
}

async function toggleRecording() {
  if (isCapturing.value) {
    abortRecording();
    return;
  }

  previousHotkey.value = hotkey.value;
  hotkey.value = [];
  isCapturing.value = true;
}

async function abortRecording() {
  if (!isCapturing.value) {
    return;
  }

  isCapturing.value = false;
  recordingKeys.value = [];
  hotkey.value = previousHotkey.value;
  previousHotkey.value = [];
}

async function resetRecording() {
  isCapturing.value = false;
  hotkey.value = [];
  previousHotkey.value = [];
  recordingKeys.value = [];
  saveHotkey();
}

async function saveHotkey() {
  hotkeyButton.value?.blur();
  settingsStore.settings.hotkey = hotkey.value;
}
</script>

<template>
  <button
    ref="hotkeyButton"
    class="px-4 py-1 min-w-[140px] border rounded-md hover:scale-105 transition-all"
    :class="isCapturing ? 'border-red-800 scale-105' : ''"
    @click="toggleRecording"
    @keydown="handleHotkeyPress"
    @keyup="handleHotkeyRelease"
    @focusout="abortRecording"
  >
    {{ hotkeyDisplay }}
  </button>
</template>

<style scoped></style>
