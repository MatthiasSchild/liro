<script setup lang="ts">
definePageMeta({
  layout: 'gui',
})

const { data } = await useFetch('/api/ledger-accounts')
</script>

<template>
  <PageSection>
    <Headline level="h1" label="Dashboard" />
  </PageSection>
</template>
