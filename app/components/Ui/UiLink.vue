<template>
  <NuxtLink
    class="
        h-9 py-2 px-3
        rounded-lg
        outline-2 outline outline-secondary hover:outline-accent
        inline-flex items-center
        font-semibold hover:cursor-pointer
        focus:outline-accent hover:text-accent
        "
    :class="{
      'bg-secondary text-primary hover:text-primary': !outlined
    }"
  >
    <slot />
  </NuxtLink>
</template>

<script setup lang="ts">
defineOptions({
  name: 'UiLink'
});

defineProps<{
  outlined?: boolean;
}>();
</script>