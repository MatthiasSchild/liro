<script setup lang="ts">
const props = defineProps<{
	level: 'h1' | 'h2'
	label: string
}>()

const styles = {
	'h1': 'text-3xl font-bold',
	'h2': 'text-2xl font-bold',
}[props.level]
</script>

<template>
	<component :is="props.level" :class="styles">
		{{ props.label }}
	</component>
</template>
