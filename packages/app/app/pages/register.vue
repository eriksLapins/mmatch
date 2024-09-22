<template>
  <div class="container">
    <div class="flex flex-col gap-4 max-w-[500px] mx-auto my-10 md:my-20">
      <UiInput
        id="email"
        v-model="form.email"
        type="email"
        placeholder="E-mail"
        autocomplete="email"
        :errors="!!errors.email"
        @input="errors.email = undefined"
      />
      <UiInput
        id="password"
        v-model="form.password"
        type="password"
        placeholder="Password"
        autocomplete="password"
        :errors="!!errors.password"
        @input="errors.password = undefined"
      />
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <UiInput
          id="name"
          v-model="form.name"
          placeholder="Name"
          autocomplete="name"
          :errors="!!errors.name"
          @input="errors.name = undefined"
        />
        <UiInput
          id="lastname"
          v-model="form.lastname"
          placeholder="Lastname"
          autocomplete="lastname"
          :errors="!!errors.lastname"
          @input="errors.lastname = undefined"
        />
      </div>
      <UiInput
        id="stagename"
        v-model="form.stageName"
        placeholder="Stagename"
        autocomplete="stagename"
        :errors="!!errors.stageName"
        @input="errors.stageName = undefined"
      />
      <UiInput
        id="description"
        v-model="form.description"
        placeholder="Description"
        autocomplete="description"
        :errors="!!errors.description"
        @input="errors.description = undefined"
      />
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <UiInput
          id="country"
          v-model="form.country"
          placeholder="Country"
          autocomplete="country"
          :errors="!!errors.country"
          @input="errors.country = undefined"
        />
        <UiInput
          id="city"
          v-model="form.city"
          placeholder="City"
          autocomplete="city"
          :errors="!!errors.city"
          @input="errors.city = undefined"
        />
      </div>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <UiInput
          id="street"
          v-model="form.street"
          placeholder="Street"
          autocomplete="street"
          :errors="!!errors.street"
          @input="errors.street = undefined"
        />
        <UiInput
          id="house-number"
          v-model="form.houseNumber"
          placeholder="House Number"
          autocomplete="house-number"
          :errors="!!errors.houseNumber"
          @input="errors.houseNumber = undefined"
        />
      </div>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <UiInput
          id="apartment"
          v-model="form.apartment"
          placeholder="Apartment"
          autocomplete="apartment"
          :errors="!!errors.apartment"
          @input="errors.apartment = undefined"
        />
      </div>
      <UiButton
        type="submit"
        class="w-max"
      >
        Login
      </UiButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Manager, Musician } from '~/types';

defineOptions({
  name: 'RegisterPage'
});

const form = ref<Musician & Manager>({
  email: '',
  password: '',
  name: '',
  lastname: '',
  stageName: '',
  description: '',
  country: '',
  city: '',
  street: '',
  houseNumber: '',
  apartment: undefined,
  bands: [],
  skills: [],
  commission: [] as unknown as [number, number],
  links: [],
  managers: [],
  phone: '',
  phoneCountry: '',
  openToCollabWith: [],
  categoriesInterestedIn: [],
});

const errors = ref<Record<keyof (Musician & Manager), string | undefined>>({
  email: undefined,
  password: undefined,
  name: undefined,
  lastname: undefined,
  stageName: undefined,
  description: undefined,
  city: undefined,
  country: undefined,
  street: undefined,
  houseNumber: undefined,
  apartment: undefined,
  bands: undefined,
  skills: undefined,
  commission: undefined,
  links: undefined,
  managers: undefined,
  phone: undefined,
  phoneCountry: undefined,
  openToCollabWith: undefined,
  categoriesInterestedIn: undefined,
});
</script>