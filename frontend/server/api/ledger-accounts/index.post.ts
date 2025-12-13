import { proxyPostRequest } from "~~/server/internal/proxy"

interface Data {
  id: number
  accountType: 'asset'
  name: string
}

export default defineEventHandler(async (event) => {
  return proxyPostRequest<Data>('/api/ledger-accounts', event);
})
