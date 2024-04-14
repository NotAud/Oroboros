<script setup lang="ts">
import { useSettingStore } from "../../stores/useSettingsStore";

const settingsStore = useSettingStore();

const isOpen = ref(false);
const clickOptionElement = ref<HTMLElement | null>(null);

const clickTypeDisplay = computed(() => {
  switch (settingsStore.settings.clickType) {
    case 0:
      return "Single";
    case 1:
      return "Double";
    case 2:
      return "Hold";
  }
});

function setClickType(clickType: 0 | 1 | 2) {
  isOpen.value = false;
  settingsStore.settings.clickType = clickType;
}

function clickOutside(e: MouseEvent) {
  if ((e.target as HTMLElement).contains(clickOptionElement.value as Node)) {
    isOpen.value = false;
  }
}

onMounted(() => {
  document.addEventListener("click", clickOutside);
});
</script>

<template>
  <div ref="clickOptionElement" class="flex flex-col w-full relative">
    <button
      class="rounded-md bg-white/5 py-0.5 hover:bg-white/10 transition-all w-full shd whitespace-nowrap text-ellipsis overflow-x-hidden"
      :class="isOpen ? 'bg-white/10' : ''"
      @click="isOpen = !isOpen"
    >
      {{ clickTypeDisplay }}
    </button>
    <div
      v-if="isOpen"
      class="flex flex-col bg-surface overflow-x-hidden max-h-[160px] w-full rounded-[4px] overflow-y-auto absolute top-[calc(100%+2px)] left-0 z-10 shadow-md transition-all"
    >
      <button
        class="px-2 py-1.5 flex-none text-start bg-white/5 hover:bg-white/10 transition-all whitespace-nowrap text-ellipsis overflow-x-hidden"
        @click="setClickType(0)"
      >
        Single
      </button>
      <button
        class="px-2 py-1.5 flex-none text-start bg-white/5 hover:bg-white/10 transition-all whitespace-nowrap text-ellipsis overflow-x-hidden"
        @click="setClickType(1)"
      >
        Double
      </button>
      <button
        class="px-2 py-1.5 flex-none text-start bg-white/5 hover:bg-white/10 transition-all whitespace-nowrap text-ellipsis overflow-x-hidden"
        @click="setClickType(2)"
      >
        Hold
      </button>
    </div>
  </div>
</template>

<style scoped></style>
