<script setup lang="ts">
import { ErrorMessage, Field } from 'vee-validate'

defineProps<{
	type: 'text' | 'email' | 'password'
	name: string
	label: string
	disabled?: boolean
}>()
</script>

<template>
	<fieldset class="fieldset">
		<legend class="fieldset-legend">{{ label }}</legend>
		<Field
			:type="type"
			:name="name"
			:disabled="disabled"
			class="input w-full"
		/>
		<ErrorMessage :name="name" as="p" class="label text-error" />
	</fieldset>
</template>
