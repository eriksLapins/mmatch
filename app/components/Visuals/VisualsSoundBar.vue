<template>
  <div
    :style="{
      height: actualHeight,
      width: width
    }"
    class="bg-secondary"
  />
</template>

<script setup lang="ts">
defineOptions({
  name: 'VisualsSoundBar'
});

const props = defineProps<{
  width: number;
  height: number;
  position: 'top' | 'bottom';
}>();
const actualHeight = ref<number>(0);

onMounted(() => {
  const numbers = new Array(props.height).map((_, i) => i);
  for (const number of numbers) {
    setTimeout(() => {
      actualHeight.value = number;
    }, 10);
  }
});
</script>