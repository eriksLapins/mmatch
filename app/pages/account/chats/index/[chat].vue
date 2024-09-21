<template>
  <div>
    some {{ $route.params.chat }}
  </div>
</template>

<script setup lang="ts">
defineOptions({
  name: 'ChatIdIndex'
});
</script>