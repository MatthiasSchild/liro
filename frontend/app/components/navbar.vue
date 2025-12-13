<script setup lang="ts">
import { User } from 'lucide-vue-next'

const links = [
  { href: '/profile', labels: 'Profile' },
  { href: '/settings', label: 'Settings' },
  { href: '/logout', label: 'Logout' },
]
</script>

<template>
  <div class="navbar bg-primary shadow-sm">
    <div class="flex-1">
      <a class="btn btn-ghost text-xl">ERP</a>
    </div>
    <div class="flex-none">
      <div class="dropdown dropdown-end">
        <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
          <User />
        </div>
        <ul
          tabindex="-1"
          class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow"
        >
          <li v-for="link in links" :key="link.href">
            <a :href="link.href">{{ link.label }}</a>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
