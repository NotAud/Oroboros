<script setup lang="ts">
type Props = {
  customGuard?: (num: number) => number;
};

const props = defineProps<Props>();

const internalInput = defineModel({ required: true, type: Number });

function inputGuard(e: KeyboardEvent) {
  e.preventDefault();
  const inputElement = e.target as HTMLInputElement;
  if (!inputElement || (isNaN(Number(e.key)) && e.key !== "Backspace")) {
    return;
  }

  let newValue;
  const start = inputElement.selectionStart;
  const end = inputElement.selectionEnd;

  if (start !== end) {
    if (e.key === "Backspace") {
      newValue =
        inputElement.value.substring(0, start as number) +
        inputElement.value.substring(end as number);
    } else {
      newValue =
        inputElement.value.substring(0, start as number) +
        e.key +
        inputElement.value.substring(end as number);
    }
  } else {
    if (e.key === "Backspace") {
      newValue = inputElement.value.substring(0, inputElement.value.length - 1);
    } else {
      newValue = inputElement.value + e.key;
    }
  }

  const newValueAsNumber = isNaN(Number(newValue)) ? 1 : Number(newValue);
  const rangedValue = Math.min(Math.max(newValueAsNumber, 1), 9999);
  internalInput.value = props.customGuard
    ? props.customGuard(rangedValue)
    : rangedValue;
}
</script>

<template>
  <input
    v-model="internalInput"
    type="text"
    class="shd w-24"
    @keydown="inputGuard"
  />
</template>

<style scoped>
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>
