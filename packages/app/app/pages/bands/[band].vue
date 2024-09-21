<template>
  <div>
    Hello {{ $route.params.band }}
  </div>
</template>

<script setup lang="ts">
defineOptions({
  name: 'BandPage'
});
</script>