<script setup lang="ts">
type Props = {
  rangeGuard?: (num: number) => number;
};

const props = defineProps<Props>();

const model = defineModel({ required: true, type: Number });

async function updateValue(e: KeyboardEvent) {
  e.preventDefault();

  const updatedValue = await getValue(e);
  if (!updatedValue) {
    return;
  }

  if (props.rangeGuard) {
    model.value = props.rangeGuard(updatedValue);
  } else {
    model.value = updatedValue;
  }
}

async function getValue(e: KeyboardEvent) {
  e.preventDefault();

  if (e.key === "Enter") {
    (e.target as HTMLInputElement).blur();
    return null;
  }

  const value = (e.target as HTMLInputElement).value;

  if (e.key === "Backspace") {
    const newValue = parseInt(value.slice(0, -1));
    if (isNaN(newValue) || newValue < 0) {
      return 0;
    }

    return newValue;
  }

  if (isNaN(parseInt(e.key))) return;
  if (model.value === 0) {
    return parseInt(e.key);
  }

  const newValue = value + e.key;
  if (newValue.length > 6) {
    return null;
  }

  return parseInt(newValue);
}
</script>

<template>
  <input :value="model" type="text" @keydown="updateValue" />
</template>

<style scoped></style>
