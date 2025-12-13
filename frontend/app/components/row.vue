<template>
  <div class="row flex flex-col md:flex-row gap-4">
    <slot />
  </div>
</template>

<style scoped>
.row>* {
  flex: 1;
}
</style>
