<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod'
import { useForm } from 'vee-validate'
import { z } from 'zod'

const errorMessage = ref<string | null>(null)

const { handleSubmit, isSubmitting } = useForm({
  initialValues: {
    adminEmail: '',
    adminPassword: '',
  },
  validationSchema: toTypedSchema(
    z.object({
      adminEmail: z.email(),
      adminPassword: z.string(),
    })
  ),
})

const onSubmit = handleSubmit(async (data) => {
  try {
    const response = await $fetch('/api/setup', {
      method: 'POST',
      body: JSON.stringify(data),
    })

    console.log(response)
  } catch {
    errorMessage.value = 'An unknown error occurred'
  }
})
</script>

<template>
  <form @submit.prevent="onSubmit">
    <Column>
      <Headline level="h2" label="Create the admin account" />
      <Row>
        <TextField
          type="email"
          name="adminEmail"
          label="Admin E-Mail"
          :disabled="isSubmitting"
        />
        <TextField
          type="password"
          name="adminPassword"
          label="Admin Password"
          :disabled="isSubmitting"
        />
      </Row>
      <div>
        <SubmitButton label="Set up ERP system" :loading="isSubmitting" />
      </div>
    </Column>
  </form>
</template>
