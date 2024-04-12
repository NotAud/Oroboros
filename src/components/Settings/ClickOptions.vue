<script setup lang="ts">
import { useSettingStore } from "../../stores/useSettingsStore";

const settingsStore = useSettingStore();

const isOpen = ref(false);
const clickOptionElement = ref<HTMLElement | null>(null);

const clickTypeDisplay = computed(() => {
  return settingsStore.settings.clickType ? "Double" : "Single";
});

function setClickType(clickType: 0 | 1) {
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
  <LabelContainer title="Click Type" class="w-full">
    <div ref="clickOptionElement" class="flex flex-col relative">
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
      </div>
    </div>
  </LabelContainer>
</template>

<style scoped></style>
