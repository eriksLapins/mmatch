<template>
  <div class="flex flex-col gap-0.5">
    <input
      v-bind="$attrs"
      class="
          w-full
          h-10 rounded-lg p-2 my-1
          placeholder:text-disabled
          outline-none outline-offset-0 outline-accent focus:outline-secondary focus-within:outline-secondary hover:outline-accent
          data-[error=true]:outline-red-500
          data-[error=true]:text-red-500
      "
      :class="{
        'mb-[18px]': !compact && !errors && !helper
      }"
      :data-error="!!errors"
      :value="modelValue"
      @input="(e) => {modelValue = (e.target as EventTargetCustom)?.value; emit('input', modelValue)}"
      @change="(e) => {modelValue = (e.target as EventTargetCustom)?.value; emit('change', modelValue)}"
      @blur="(e) => {modelValue = (e.target as EventTargetCustom)?.value; emit('blur', modelValue)}"
      @focus="(e) => {modelValue = (e.target as EventTargetCustom)?.value; emit('focus', modelValue)}"
    >
    <div v-if="errors || helper">
      <p
        v-if="errors"
        class="text-xs leading-none text-red-500"
      >
        {{ errors }}
      </p>
      <p
        v-if="helper"
        class="text-xs leading-none text-black"
      >
        {{ helper }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
defineOptions({
  name: 'UiInput'
});

defineProps<{
  errors?: string;
  helper?: string;
  compact?: boolean;
}>();

const emit = defineEmits<{
  'input': [string | undefined]
  'change': [string | undefined]
  'blur': [string | undefined]
  'focus': [string | undefined]
}>();

type EventTargetCustom = EventTarget & {
  value: string | undefined
} | null;

const modelValue = defineModel<string | undefined | null>('modelValue');
</script>