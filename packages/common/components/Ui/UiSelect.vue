<template>
  <div
    ref="selectRef"
    class="relative"
  >
    <button
      class="w-full flex justify-start items-center"
      @click.prevent="dropdownVisible = !dropdownVisible; emit('dropdown', dropdownVisible)"
    >
      <div
        v-if="modelValue"
        class="min-w-[20px] inline-flex gap-1 items-center overflow-clip"
      >
        <slot
          name="selectPrepend"
        />
        {{ modelValue[title as keyof T] }}
        <slot
          name="selectAppend"
        />
      </div>
      <IArrowDown
        class="size-4 shrink-0 min-w-4 transition-transform duration-200"
        :class="{
          'rotate-180': dropdownVisible
        }"
      />
    </button>
    <div
      v-if="dropdownVisible"
      class="absolute top-12 min-h-10 bg-white shadow-md w-full rounded-lg p-2"
    >
      <button
        v-for="item in items"
        :key="item[value as keyof T]"
        class="w-full flex justify-start items-start gap-1 max-h-[100px] scrollbar-vertical"
        @click.prevent="modelValue = item; dropdownVisible = false; emit('dropdown', dropdownVisible); emit('selected', item)"
      >
        <slot
          name="itemPrepend"
          :item="item"
        />
        {{ item[title as keyof T] }}
        <slot
          name="itemAppend"
          :item="item"
        />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts" generic="T extends Record<string, any>">
import { onClickOutside} from '@vueuse/core';
defineOptions({
  name: 'UiSelect'
});

const props = withDefaults(
  defineProps<{
    items: T[];
    value?: keyof T;
    title?: keyof T;
    defaultValue?: string;
  }>(),
  {
    value: 'value',
    title: 'title',
    defaultValue: undefined,
  }
);

const emit = defineEmits<{
  'dropdown': [boolean];
  'selected': [T];
}>();

const selectRef = ref<HTMLDivElement>();
onClickOutside(selectRef, () => {
  dropdownVisible.value = false;
  emit('dropdown', false);
});

const modelValue = defineModel<T| undefined>('modelValue');
onMounted(() => {
  if (props.defaultValue) {
    modelValue.value = props.items.find(item => item[props.value] === props.defaultValue);
  }
});

const dropdownVisible = ref(false);
</script>