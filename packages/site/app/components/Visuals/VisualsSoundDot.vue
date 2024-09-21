<template>
  <div
    :style="{
      bottom: `${height-4}px`,
      left: `${Math.floor(width/2)-4}px`
    }"
    class="absolute size-2 bg-red-500 rounded-full"
    :class="{
      'animate-fadeIn': !isLoaded,
      'animate-pulse': isLoaded,
    }"
  />
</template>

<script setup lang="ts">
defineOptions({
  name: 'VisualsSoundDot'
});

const isLoaded = ref(false);

defineProps<{
  width: number;
  height: number;
}>();

onMounted(() => {
  setTimeout(() => {
    isLoaded.value = true;
  }, 1000);
});
</script>