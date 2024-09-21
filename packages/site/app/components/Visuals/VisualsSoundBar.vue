<template>
  <div
    :style="{
      height: `${maxHeight}px`,
      width: `${width}px`
    }"
    class="flex"
    :class="{
      'items-end': position === 'top'
    }"
  >
    <div
      :style="{
        height: `${height}px`,
        width: `${width}px`
      }"
      class="flex"
      :class="{
        'items-end': position === 'top'
      }"
    >
      <div
        class="animate-grow shrink-0 w-full h-full"
        :class="backgroundClass ?? 'bg-accent'"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
defineOptions({
  name: 'VisualsSoundBar'
});

defineProps<{
  width: number;
  height: number;
  maxHeight: number;
  position: 'top' | 'bottom';
  backgroundClass?: string;
}>();
</script>