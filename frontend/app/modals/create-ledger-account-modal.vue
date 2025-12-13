<script setup lang="ts">
import { toTypedSchema } from '@vee-validate/zod'
import { useForm } from 'vee-validate'
import { z } from 'zod'
import { toast } from 'vue3-toastify'
import { FetchError } from 'ofetch'

const modalRef = ref<HTMLDialogElement | null>(null)
const errorMessage = ref<string | null>(null)

const props = defineProps<{
  open: boolean
}>()

const emit = defineEmits(['close'])

const { handleSubmit } = useForm({
  initialValues: {
    name: '',
    type: '',
  },
  validationSchema: toTypedSchema(
    z.object({
      name: z.string(),
      type: z.string(),
    }),
  ),
})

const onSubmit = handleSubmit(async (data) => {
  try {
    await $fetch('/api/ledger-accounts', {
      method: 'POST',
      body: data,
    })

    emit('close')
  } catch (err: unknown) {
    errorMessage.value = getErrorMessageFromServerError(err)
  }
})

watch([modalRef], () => {
  const modal = modalRef.value
  if (modal) {
    modal.onclose = () => emit('close')
  }
})

watch(
  [props, modalRef],
  () => {
    if (props.open) {
      modalRef.value?.showModal()
    } else {
      modalRef.value?.close()
    }
  },
  { immediate: true },
)
</script>

<template>
  <dialog ref="modalRef" class="modal">
    <div class="modal-box">
      <form @submit.prevent="onSubmit">
        <h3 class="text-2xl font-bold mb-4">Create a new ledger account</h3>

        <Column>
          <p>Enter the information of the new ledger account.</p>

          <div v-if="errorMessage" class="alert alert-error">
            {{ errorMessage }}
          </div>
          <TextField type="text" name="name" label="Name" />
          <TextField type="text" name="type" label="Type" />
        </Column>

        <div class="modal-action">
          <button class="btn btn-primary">Submit</button>
          <button class="btn" @click="emit('close')">Close</button>
        </div>
      </form>
    </div>
  </dialog>
</template>
