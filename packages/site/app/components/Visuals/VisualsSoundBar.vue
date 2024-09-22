<template>
  <div
    :style="{
      height: `${maxHeight}px`,
      width: `${width}px`,
    }"
    class="flex base-bar"
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
        class="growing-bar transition-[height] delay-200 duration-500 ease-in shrink-0 w-full border border-solid border-accent"
        :class="[backgroundClass ?? 'bg-accent', {
          'h-full': isLoaded,
          'h-0': !isLoaded,
        }]"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
defineOptions({
  name: 'VisualsSoundBar'
});

const props = defineProps<{
  width: number;
  height: number;
  maxHeight: number;
  position: 'top' | 'bottom';
  backgroundClass?: string;
  delay?: number
}>();

const isLoaded = ref(false);
onMounted(() => {
  setTimeout(() => {
    isLoaded.value = true;
  }, (props.delay ?? 0)*150);
});
</script>