<template>
  <div class="h-[500px] flex">
    <ClientOnly>
      <div
        v-for="(bar) in bars"
        :key="bar.index"
        class="relative"
        :style="{
          top: `${(500-(maxHeight*2))/2}px`
        }"
      >
        <VisualsSoundBar
          :height="bar.height"
          :width="barWidth"
          :max-height="maxHeight"
          position="top"
          :class="{
            invisible: bar.position === 'bottom'
          }"
          :background-class="barBackgroundClass"
        />
      
        <VisualsSoundBar
          :height="bar.height"
          :width="barWidth"
          :max-height="maxHeight"
          position="bottom"
          :class="{
            invisible: bar.position === 'top'
          }"
          :background-class="barBackgroundClass"
        />
        <VisualsSoundDot
          v-if="!hideDots"
          :height="bar.position === 'top' ? bar.height + maxHeight + (500-(maxHeight*2)) : -bar.height + maxHeight + (500-(maxHeight*2))"
          :width="barWidth"
        />
      </div>
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
import { useWindowSize, useMediaQuery } from '@vueuse/core';
defineOptions({
  name: 'VisualsSoundBars'
});

const props = defineProps<{
  maxHeight: number;
  inverse?: boolean;
  hideDots?: boolean;
  barBackgroundClass?: string;
}>();

const { width: windowWidth } = useWindowSize();
const largeScreen = useMediaQuery('(min-width:976px)');

const barWidth = computed(() => {
  if (largeScreen.value) {
    return 20;
  }
  return 15;
});

const barCount = computed(() => {
  if (import.meta.client) {
    return Math.floor(windowWidth.value / barWidth.value);
  }
  return 0;
});

const equation = (x: number) => {
  const lambda = largeScreen.value ? -75 : -40;
  const T = 30;
  const k = (Math.PI*2)/lambda;
  const w = (Math.PI*2)/T;
  return props.maxHeight*Math.sin(k*x+w*0.5+0);
}; 
function getBarHeightAndPos(index: number) {
  const relativeNumber = index - Math.floor(barCount.value / 2);
  const num = equation(relativeNumber);
  return {
    height: num > 0 ? num : -num,
    position: num > 0 ? 'top' : 'bottom',
  };
}

const bars = computed(() => {
  if (barCount.value) {
    const barArray = Array.from({length: barCount.value}).map((_, i) => ({
      index: i,
      ...getBarHeightAndPos(i)
    }));
    const maxValue = Math.max(...barArray.map(v => v.height));
    const adjustedBarArray = barArray.map(item => {
      console.log(item.height);
      return {
        index: item.index,
        position: props.inverse ? item.position === 'top' ? 'bottom' : 'top' : item.position,
        height: ((item.height / maxValue) + (Math.random() * 0.15)) * props.maxHeight,
      };
    });

    return adjustedBarArray;
  }
  return [];
});
</script>