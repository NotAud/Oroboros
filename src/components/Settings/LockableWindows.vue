<script setup lang="ts">
import { useSettingStore } from "../../stores/useSettingsStore";

const settingsStore = useSettingStore();
const isOpen = ref(false);

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
</script>

<template>
  <div class="flex gap-x-4 relative w-fit">
    <LabelContainer title="Window Detection">
      <SwitchInput
        v-model="settingsStore.settings.windowDetection"
        @click="settingsStore.populateLockableWindows"
      />
    </LabelContainer>
    <div
      v-if="settingsStore.settings.windowDetection"
      class="flex items-center gap-x-4"
    >
      <div class="flex items-end h-full relative">
        <button
          class="border border-white rounded-md bg-zinc-800/5 py-0.5 hover:bg-zinc-900/80 transition-all w-[200px] whitespace-nowrap text-ellipsis overflow-x-hidden"
          @click="isOpen = !isOpen"
        >
          {{ displayName }}
        </button>
        <div
          v-if="isOpen"
          class="flex flex-col bg-zinc-900 overflow-x-hidden max-h-[300px] max-w-full rounded-[4px] overflow-y-auto absolute top-[calc(100%+5px)] left-0 z-10 shadow-md transition-all"
        >
          <button
            class="px-2 py-1.5 flex-none text-start hover:bg-zinc-950/40 transition-all whitespace-nowrap text-ellipsis overflow-x-hidden"
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

<style scoped>
/* width */
::-webkit-scrollbar {
  width: 5px;
}

/* Track */
::-webkit-scrollbar-track {
  background: transparent;
}

/* Handle */
::-webkit-scrollbar-thumb {
  @apply rounded-full;
  background: #888;
}

/* Handle on hover */
::-webkit-scrollbar-thumb:hover {
  background: #555;
}
</style>
