<script setup lang="ts">
import { Plus } from 'lucide-vue-next'
import CreateLedgerAccountModal from '~/modals/create-ledger-account-modal.vue'

const showCreateModal = ref(false)

definePageMeta({
  layout: 'gui',
})

useHead({
  title: 'Ledger accounts',
})

const { data } = await useFetch('/api/ledger-accounts')
</script>

<template>
  <PageSection>
    <Headline level="h1" label="Ledger accounts" />

    <div>
      <button class="btn" @click="showCreateModal = true">
        <Plus />
        Create
      </button>
    </div>

    <table v-if="data" class="table table-pin-rows">
      <thead>
        <tr>
          <th>ID</th>
          <th>Account type</th>
          <th>Name</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="account in data.data">
          <td>{{ account.id }}</td>
          <td>{{ account.accountType }}</td>
          <td>
            <a :href="`/ledger/accounts/${account.id}`" class="link">
              {{ account.name }}
            </a>
          </td>
        </tr>
      </tbody>
    </table>
  </PageSection>

  <CreateLedgerAccountModal
    :open="showCreateModal"
    @close="showCreateModal = false"
  />
</template>
