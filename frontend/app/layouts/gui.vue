<script setup lang="ts">
interface NavGroup {
  name: string
  links: {
    href: string
    label: string
  }[]
}

const groups: NavGroup[] = [
  {
    name: 'Inventory',
    links: [
      { href: '/stocks', label: 'Item stocks' },
      { href: '/products', label: 'Products' },
    ],
  },
  {
    name: 'Sales',
    links: [
      { href: '/customers', label: 'Customers' },
      { href: '/sales/quotations', label: 'Quotations' },
      { href: '/sales/orders', label: 'Orders' },
      { href: '/sales/invoices', label: 'Invoices' },
    ],
  },
  {
    name: 'Purchase',
    links: [
      { href: '/suppliers', label: 'Suppliers' },
      { href: '/purchase/orders', label: 'Purchases' },
      { href: '/purchase/invoices', label: 'Invoices' },
    ],
  },
  {
    name: 'Accounting',
    links: [{ href: '/ledger/accounts', label: 'Accounts' }],
  },
]
</script>

<template>
  <div class="h-dvh flex flex-col">
    <Navbar />
    <div class="flex-1 flex flex-row">
      <div class="min-w-96 bg-base-200">
        <template v-for="group in groups">
          <div class="bg-base-300 text-xl font-bold p-4">{{ group.name }}</div>
          <div>
            <a
              v-for="link in group.links"
              :href="link.href"
              class="block p-4 hover:link"
            >
              {{ link.label }}
            </a>
          </div>
        </template>
      </div>

      <div>
        <slot />
      </div>
    </div>
    <Footer />
  </div>
</template>
