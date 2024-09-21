<template>
  <button
    class="
        h-9 py-2 px-3
        rounded-lg
        outline-2 outline outline-secondary hover:outline-accent
        inline-flex items-center
        font-semibold
        focus:outline-accent
        "
    :class="{
      'bg-secondary text-primary': !outlined
    }"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
defineOptions({
  name: 'UiButton'
});

defineProps<{
  outlined?: boolean;
}>();
</script>