<template>
  <div class="relative">
    <div class="min-h-[500px] flex justify-center items-center flex-col gap-4 container mx-auto px-4 z-20 mb-[100px] md:mb-[200px]">
      <h1 class="text-7xl sm:text-9xl lg:text-[250px] text-black leading-none">
        MMatch
      </h1>
      <p class="text-3xl md:text-4xl font-semibold text-black">
        A place where musicians and managers meet
      </p>
      <div class="w-full flex flex-col sm:flex-row gap-4 justify-center max-md:items-center">
        <UiLink
          outlined
          class="w-40 justify-center"
          rel="external"
          :href="`${$config.public.appUrl}/login`"
        >
          Login
        </UiLink>
        <UiLink
          class="w-40 justify-center pointer-events-auto"
          rel="external"
          :href="`${$config.public.appUrl}/register`"
        >
          Register
        </UiLink>
      </div>
    </div>
    <div
      class="absolute top-[100px] md:top-[200px] -z-20"
    >
      <VisualsSoundBars
        class="absolute"
        :max-height="isLargeScreen ? 350: 250"
        bar-background-class="bg-slate-300"
        hide-dots
        mirrored
        function-name="equalizer"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMediaQuery } from '@vueuse/core';
defineOptions({
  name: 'IndexPage'
});

useSeoMeta({
  title: 'Home',
  ogTitle: 'Home',
  description: 'Music Match is a place where musicians meet'
});

const isLargeScreen = useMediaQuery('(min-width:976px)');
</script>