<script setup lang="ts">
import { useForm } from 'vee-validate'
import { z } from 'zod'

const { handleSubmit, isSubmitting } = useForm({
	initialValues: {
		email: '',
		password: '',
	},
	validationSchema: z.object({
		email: z.email(),
		password: z.string(),
	}),
})

const onSubmit = handleSubmit(async (data) => {
	console.log(data)
})
</script>

<template>
	<form @submit.prevent="onSubmit">
		<TextField
			type="email"
			name="email"
			label="E-Mail"
			:disabled="isSubmitting"
		/>
		<TextField
			type="password"
			name="password"
			label="Password"
			:disabled="isSubmitting"
		/>
		<SubmitButton label="Sign in" :disabled="isSubmitting" />
	</form>
</template>
