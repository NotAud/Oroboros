<script setup lang="ts">
const model = defineModel({ required: true, type: Boolean });
</script>

<template>
  <label class="switch">
    <input v-model="model" type="checkbox" />
    <span class="slider"></span>
  </label>
</template>

<style scoped>
.switch {
  @apply h-7 relative inline-block;
  width: 4.5em;
}

.switch input {
  @apply w-0 h-0 opacity-0;
}

.slider {
  @apply bg-zinc-900 absolute cursor-pointer top-0 left-0 right-0 bottom-0;
  box-shadow: inset -1px 1px 4px 2px rgba(0, 0, 0, 0.8);
  transition: 0.4s;
  border-radius: 30px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 1.1em;
  width: 1.1em;
  border-radius: 20px;
  left: 0.3em;
  bottom: 0.3em;
  background-color: rgb(255, 255, 255);
  transition: 0.4s;
}

.switch input:checked + .slider {
  @apply bg-green-500;
  box-shadow: inset 2px 2px 4px 2px rgba(0, 0, 0, 0.8);
}

/* .switch input:focus + .slider {
  box-shadow: inset -2px 2px 5px 1px rgba(0, 0, 0, 0.8);
} */

.switch input:checked + .slider:before {
  transform: translateX(2.5em);
}
</style>
