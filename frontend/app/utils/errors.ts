import { FetchError } from 'ofetch'
import { toast } from 'vue3-toastify'

export function getErrorMessageFromServerError(err: unknown) {
  if (err instanceof FetchError) {
    const errorData = err.data.data as {
      error?: string
      errorCode?: string
    }

    const message = errorData.error
    if (message) {
      return message
    }
  }

  return 'An unknown error occurred'
}

export function handleErrorFromServer(err: unknown) {
  const message = getErrorMessageFromServerError(err)
  toast.error(message)
}
