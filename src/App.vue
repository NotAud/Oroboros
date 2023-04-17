<script setup lang="ts">
import { emit, listen } from "@tauri-apps/api/event";

type AutoclickerState = {
  is_autoclicking: boolean;
  delay: number;
};

const is_autoclicking = ref(false);
const delay: Ref<number | string> = ref(50);

const enabledText = computed(() => {
  return is_autoclicking.value ? "Stop" : "Start";
});

async function toggleAutoclicker() {
  emit("start_autoclicker");
}

watch(delay, async (currDelay, prevDelay) => {
  if (typeof currDelay !== "number" || typeof prevDelay !== "number") {
    [currDelay, prevDelay] = [Number(currDelay), Number(prevDelay)];
  }

  if (currDelay <= 0 || Number.isNaN(currDelay)) {
    delay.value = prevDelay;
    return;
  }

  await emit("set_delay", { value: currDelay });
});

onMounted(() => {
  listen("update_settings", (updatedState) => {
    const payload = updatedState.payload as AutoclickerState;
    is_autoclicking.value = payload.is_autoclicking;
    delay.value = payload.delay;
  });
});
</script>

<template>
  <div
    class="flex justify-between w-full h-full items-center px-5 bg-backgroundColor text-foregroundColor"
  >
    <input
      v-model="delay"
      type="text"
      class="bg-buttonBackgroundColor rounded-md h-10 pl-2"
      :disabled="is_autoclicking"
    />
    <button
      class="py-2 px-8 bg-buttonBackgroundColor text-white font-bold rounded-lg shadow-lg hover:bg-buttonMouseOverBackgroundColor active:bg-buttonPressedBackgroundColor"
      @click="toggleAutoclicker"
    >
      {{ enabledText }}
    </button>
  </div>
</template>
