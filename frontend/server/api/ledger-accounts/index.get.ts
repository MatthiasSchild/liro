import { Page, proxyGetRequest } from "~~/server/internal/proxy"

interface Account {
  id: number
  accountType: 'asset'
  name: string
}

export default defineEventHandler(async (event) => {
  return proxyGetRequest<Page<Account>>('/api/ledger-accounts', event);
})
