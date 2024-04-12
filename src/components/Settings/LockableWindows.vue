<script setup lang="ts">
import { useSettingStore } from "../../stores/useSettingsStore";

const settingsStore = useSettingStore();
const isOpen = ref(false);
const windowDetectionElement = ref<HTMLElement | null>(null);

const displayName = computed(() => {
  if (!settingsStore.settings.windowInfo[0]) {
    return "Select Window";
  }

  return parseName(settingsStore.settings.windowInfo[1]);
});

function parseName(name: string) {
  const combined = name.split(" - ");
  const last = combined[combined.length - 1];
  return last;
}

async function set_window(windowInfo: [number, string]) {
  isOpen.value = false;
  settingsStore.settings.windowInfo = windowInfo;
}

function clickOutside(e: MouseEvent) {
  if (e.target.contains(windowDetectionElement.value as Node)) {
    console.log("click outside");
  }
}

onMounted(() => {
  document.addEventListener("click", clickOutside);
});
</script>

<template>
  <div class="flex items-center gap-x-4 relative w-[250px] h-[30px]">
    <SwitchInput
      v-model="settingsStore.settings.windowDetection"
      @click="settingsStore.populateLockableWindows"
    />
    <div
      v-if="settingsStore.settings.windowDetection"
      ref="windowDetectionElement"
      class="flex items-center"
    >
      <div class="flex h-full relative">
        <button
          class="rounded-md bg-white/5 py-0.5 hover:bg-white/10 transition-all shd w-[180px] whitespace-nowrap text-ellipsis overflow-x-hidden"
          :class="isOpen ? 'bg-white/10' : ''"
          @click="isOpen = !isOpen"
        >
          {{ displayName }}
        </button>
        <div
          v-if="isOpen"
          class="flex flex-col bg-surface overflow-x-hidden max-h-[160px] w-full rounded-[4px] overflow-y-auto absolute top-[calc(100%+5px)] left-0 z-10 shadow-md transition-all"
        >
          <button
            class="px-2 py-1.5 flex-none text-start bg-white/5 hover:bg-white/10 transition-all whitespace-nowrap text-ellipsis overflow-x-hidden"
            v-for="(window, $i) in settingsStore.settings.lockableWindows"
            :key="$i"
            @click="set_window(window)"
          >
            {{ parseName(window[1]) }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
