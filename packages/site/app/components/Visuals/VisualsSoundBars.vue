<template>
  <div class="h-[500px] flex w-full">
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
          :delay="delayFn(bar.index)"
          position="top"
          class="[&_.growing-bar]:border-b-0"
          :class="{
            invisible: bar.position === 'bottom' && !mirrored
          }"
          :background-class="barBackgroundClass"
        />
      
        <VisualsSoundBar
          :height="bar.height"
          :width="barWidth"
          :max-height="maxHeight"
          :delay="delayFn(bar.index)"
          position="bottom"
          class="[&_.growing-bar]:border-t-0"
          :class="{
            invisible: bar.position === 'top' && !mirrored
          }"
          :background-class="barBackgroundClass"
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
  functionName: 'wave' | 'equalizer'
  mirrored?: boolean;
}>();

const { width: windowWidth } = useWindowSize();
const largeScreen = useMediaQuery('(min-width:976px)');


const barWidth = computed(() => {
  if (largeScreen.value) {
    return 10;
  }
  return 5;
});

const barCount = computed(() => {
  if (import.meta.client) {
    return Math.floor(windowWidth.value / barWidth.value);
  }
  return 0;
});

const waveEquation = (x: number) => {
  const lambda = largeScreen.value ? -40 : -40;
  const T = 30;
  const k = (Math.PI*2)/lambda;
  const w = (Math.PI*2)/T;
  return props.maxHeight*Math.sin(k*x+w*0.9+5);
};

const equalizerEquation = (x: number) => {
  const A = 1.8;
  const frequency = 0.175;
  const phi = 0.125;

  return A*Math.sin(2*Math.PI*(frequency*x*(Math.pow(x, (0.75))) + phi));
};
function getBarHeightAndPos(index: number) {
  const num = props.functionName === 'wave' ? waveEquation(index) : equalizerEquation(index);
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
    const adjustHeight = (item:typeof barArray[0]) => {
      if (props.functionName === 'wave') {
        return ((item.height / maxValue) + (Math.random() * 0.15)) * props.maxHeight;
      }
      if (props.functionName === 'equalizer') {
        let divider = 0.35;
        if (item.index >= 10 && item.index < 30) {
          divider = 5;
        } else if (item.index >= 30 && item.index < 50) {
          divider = 10;
        } else if (item.index >= 50 && item.index < 60) {
          divider = 15;
        } else if (item.index >= 60 && item.index < 75) {
          divider = 20;
        } else if (item.index >= 75 && item.index < 90) {
          divider = 40;
        } else if (item.index >= 90 && item.index < 105) {
          divider = 30;
        } else if (item.index >= 105 && item.index < 125) {
          divider = 20;
        } else if (item.index >= 125 && item.index < 145) {
          divider = 40;
        } else if (item.index > 145) {
          divider = 30;
        }
        return (item.height / maxValue)*(1/((item.index || 1)/divider)) * props.maxHeight*1.5;
      }
      return item.height;

    };
    const maxValue = Math.max(...barArray.map(v => v.height));
    const adjustedBarArray = barArray.map(item => {
      console.log(item.height);
      return {
        index: item.index,
        position: props.inverse ? item.position === 'top' ? 'bottom' : 'top' : item.position,
        height: adjustHeight(item),
      };
    });

    return adjustedBarArray;
  }
  return [];
});

const delayFn = (index: number) => {
  return (index/barCount.value)*10;
};
</script>