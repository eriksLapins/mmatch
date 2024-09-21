<template>
  <div>
    <div class="min-h-[500px] flex justify-center items-center flex-col gap-4 container mx-auto px-4">
      <h1 class="text-7xl sm:text-9xl lg:text-[250px] text-black leading-none">
        MMatch
      </h1>
      <div class="w-full flex flex-col sm:flex-row gap-4 justify-center max-md:items-center">
        <UiLink
          outlined
          class="w-40 justify-center"
          href="/login"
        >
          Login
        </UiLink>
        <UiLink
          class="w-40 justify-center"
          href="/register"
        >
          Register
        </UiLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineOptions({
  name: 'IndexPage'
});

useSeoMeta({
  title: 'Home',
  ogTitle: 'Home',
  description: 'Music Match is a place where musicians meet'
});
</script>