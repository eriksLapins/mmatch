<template>
  <input
    class="
        w-full
        h-10 rounded-lg p-2 my-1
        placeholder:text-disabled
        outline-none outline-offset-0 outline-accent focus:outline-secondary focus-within:outline-secondary hover:outline-accent
        data-[error=true]:outline-red-500
        data-[error=true]:text-red-500
    "
    :data-error="errors"
    :value="modelValue"
    @input="(e) => {modelValue = (e.target as EventTargetCustom)?.value; emit('input', modelValue)}"
    @change="(e) => {modelValue = (e.target as EventTargetCustom)?.value; emit('change', modelValue)}"
  >
</template>

<script setup lang="ts">
defineOptions({
  name: 'UiInput'
});

defineProps<{
  errors?: boolean;
}>();

const emit = defineEmits<{
  'input': [string | undefined]
  'change': [string | undefined]
}>();

type EventTargetCustom = EventTarget & {
  value: string | undefined
} | null;

const modelValue = defineModel<string | undefined>('modelValue');
</script>