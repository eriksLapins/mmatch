<template>
  <div class="flex flex-col gap-0.5">
    <div
      ref="phoneInputRef"
      class="
        inline-flex flex-nowrap w-full
        outline-none outline-offset-0 outline-accent hover:outline-accent
        p-2 my-1 h-10 rounded-lg
      "
      :class="{
        '!outline-secondary': isFocus,
        '!outline-red-500 text-red-500': !!errors,
        'mb-[18px]': !compact && !errors && !helper
      }"
    >
      <UiSelect
        v-model="selectedCountry"
        :items="countries"
        :default-value="countries[0].value"
        class="
          w-16 pr-2
          border-r-2 border-r-accent -my-2
          flex items-center justify-center
        "
        :class="{
          '!border-r-secondary': isFocus
        }"
        @dropdown="(value) => value ? isFocus = value : ''"
        @update:model-value="(v) => v ? prefix = v.value.toString() : prefix = v"
        @selected="isFocus = false"
      />
      <input
        v-bind="$attrs"
        class="w-full h-full placeholder:text-disabled focus:outline-[0] pl-2"
        :data-error="!!errors"
        :value="phone"
        @input="(e) => {phone = (e.target as EventTargetCustom)?.value; emit('input', phone)}"
        @change="(e) => {phone = (e.target as EventTargetCustom)?.value; emit('change', phone)}"
        @blur="(e) => {phone = (e.target as EventTargetCustom)?.value; emit('blur', phone); isFocus = false}"
        @focus="(e) => {phone = (e.target as EventTargetCustom)?.value; emit('focus', phone); isFocus = true}"
      >
    </div>
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
import UiSelect from '../UiSelect.vue';
import { onClickOutside } from '@vueuse/core';

defineOptions({
  name: 'UiInputPhone'
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

const phoneInputRef = ref<HTMLDivElement>();
onClickOutside(phoneInputRef, () => {
  isFocus.value = false;
});
  
const phone = defineModel<string | undefined>('phone');
const prefix = defineModel<string | undefined>('prefix');
const selectedCountry = ref<{ value: string, title: string }>();
const countries = [
  {
    value: '00371',
    title: '+371',
  },
  {
    value: '00370',
    title: '+370',
  }
];

const isFocus = ref(false);
</script>