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
  --secondary-container: #3a4b39;
  --primary: #84da89;
  font-size: 17px;
  position: relative;
  display: inline-block;
  width: 60px;
  height: 20px;
}

.switch input {
  display: none;
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #313033;
  transition: 0.2s;
  border-radius: 30px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 15px;
  width: 15px;
  border-radius: 20px;
  left: 3px;
  bottom: 3px;
  background-color: #aeaaae;
  transition: 0.4s;
}

input:checked + .slider::before {
  background-color: var(--primary);
}

input:checked + .slider {
  background-color: var(--secondary-container);
}

input:focus + .slider {
  box-shadow: 0 0 1px var(--secondary-container);
}

input:checked + .slider:before {
  transform: translateX(38px);
}
</style>
