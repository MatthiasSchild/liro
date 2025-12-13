<script setup lang="ts">
useHead({
  title: 'ERP Setup',
})
</script>

<template>
  <div class="p-4">
    <div class="container mx-auto flex flex-col gap-4">
      <Headline level="h1" label="Set up ERP system" />
      <SetupForm />
    </div>
  </div>
</template>
