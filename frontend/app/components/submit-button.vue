<script setup lang="ts">
defineProps<{
  label: string
  loading?: boolean
  disabled?: boolean
}>()
</script>

<template>
  <button
    type="submit"
    class="btn btn-primary flex flex-row gap-2"
    :disabled="loading || disabled"
  >
    <span v-if="loading" class="loading loading-spinner loading-md"></span>
    <span>{{ label }}</span>
  </button>
</template>
