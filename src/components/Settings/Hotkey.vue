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
  <div class="flex gap-x-2 items-center">
    <button
      v-if="settingsStore.settings.hotkey.length > 0"
      class="h-full px-2 hover:scale-105 transition-all"
      @click="resetRecording"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="18"
        w="18"
        viewBox="0 0 512 512"
        class="fill-white"
      >
        <path
          d="M386.3 160H336c-17.7 0-32 14.3-32 32s14.3 32 32 32H464c17.7 0 32-14.3 32-32V64c0-17.7-14.3-32-32-32s-32 14.3-32 32v51.2L414.4 97.6c-87.5-87.5-229.3-87.5-316.8 0s-87.5 229.3 0 316.8s229.3 87.5 316.8 0c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0c-62.5 62.5-163.8 62.5-226.3 0s-62.5-163.8 0-226.3s163.8-62.5 226.3 0L386.3 160z"
        />
      </svg>
    </button>
    <button
      ref="hotkeyButton"
      class="px-4 py-1 min-w-[140px] border rounded-md select-none hover:scale-105 transition-all"
      :class="isCapturing ? 'border-red-800 scale-105' : ''"
      @click="toggleRecording"
      @keydown="handleHotkeyPress"
      @keyup="handleHotkeyRelease"
      @focusout="abortRecording"
    >
      {{ hotkeyDisplay }}
    </button>
  </div>
</template>

<style scoped></style>
