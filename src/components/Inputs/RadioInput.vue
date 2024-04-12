<script setup lang="ts">
type Props = {
  value: any;
  text?: string;
  modelValue: string;
};

defineProps<Props>();
const modelValue = defineModel();
</script>

<template>
  <div class="radio-button flex items-center select-none">
    <input
      v-model="modelValue"
      :id="`radio-${value}`"
      :value="value"
      class="radio-button__input"
      type="radio"
      :name="`radio-${value}`"
      @change="modelValue = value"
    />
    <label class="radio-button__label" :for="`radio-${value}`">
      <span class="radio-button__custom"></span>

      <slot>{{ text }}</slot>
    </label>
  </div>
</template>

<style scoped>
.radio-buttons-container {
  display: flex;
  align-items: center;
  gap: 24px;
}

.radio-button {
  position: relative;
  cursor: pointer;
}

.radio-button__input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.radio-button__label {
  display: inline-block;
  padding-left: 30px;
  margin-bottom: 10px;
  position: relative;
  font-size: 16px;
  color: #fff;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.23, 1, 0.32, 1);
}

.radio-button__custom {
  position: absolute;
  top: 50%;
  left: 0;
  transform: translateY(-50%);
  width: 20px;
  height: 20px;
  border-radius: 50%;
  border: 2px solid #555;
  transition: all 0.3s cubic-bezier(0.23, 1, 0.32, 1);
}

.radio-button__input:checked + .radio-button__label .radio-button__custom {
  transform: translateY(-50%) scale(0.9);
  @apply border-green-400 text-green-400;
  border: 5px solid;
  /* color: #4c8bf5; */
}

.radio-button__input:checked + .radio-button__label {
  @apply border-green-400;
  /* color: #4c8bf5; */
}

.radio-button__label:hover .radio-button__custom {
  @apply border-green-400;
  transform: translateY(-50%) scale(1.2);
  /* border-color: #4c8bf5; */
  box-shadow: 0 0 10px #4ac76f80;
}
</style>
