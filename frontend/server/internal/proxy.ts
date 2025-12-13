import { H3Event, getHeaders, getQuery, readBody } from 'h3'
import { FetchError } from 'ofetch'

const target = 'http://localhost:5000'

export interface Page<T> {
  limit: number
  offset: number
  total: number
  data: T[]
}

export async function proxyGetRequest<T>(
  path: string,
  event: H3Event,
): Promise<T> {
  const query = getQuery(event)
  const headers = getHeaders(event)

  const url = new URL(target + path)
  for (const key in query) {
    const value = query[key] as string
    url.searchParams.append(key, value)
  }

  try {
    const response = await $fetch(url.toString(), {
      method: 'GET',
      headers: headers as Record<string, string>,
    })

    return response as T
  } catch (err) {
    console.error(err)

    throw createError({
      statusCode: 500,
      statusMessage: 'Proxy request failed',
    })
  }
}

export async function proxyPostRequest<T>(
  path: string,
  event: H3Event,
): Promise<T> {
  const body = await readBody(event)
  const headers = getHeaders(event)

  try {
    const response = await $fetch(target + path, {
      method: 'POST',
      headers: headers as Record<string, string>,
      body,
    })

    return response as T
  } catch (err: unknown) {
    if (err instanceof FetchError) {
      throw err
    }

    throw createError({
      statusCode: 500,
      statusMessage: 'Proxy request failed',
    })
  }
}
