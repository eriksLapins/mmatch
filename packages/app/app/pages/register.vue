<template>
  <div class="container">
    <form
      class="flex flex-col gap-2 max-w-[500px] mx-auto my-10 md:my-20"
      @submit.prevent="onSubmit"
    >
      <UiInput
        id="email"
        v-model="form.email"
        type="email"
        placeholder="E-mail"
        autocomplete="email"
        :errors="errors.email"
        @input="errors.email = undefined"
      />
      <UiInputPhone
        id="phone"
        v-model:phone="form.phone"
        v-model:prefix="form.phone_prefix"
        placeholder="Phone"
        autocomplete="phone"
        :errors="errors.phone"
        @input="errors.phone = undefined"
      />
      <UiInput
        id="password"
        v-model="form.password"
        type="password"
        placeholder="Password"
        autocomplete="off"
        :errors="failedRepeat ?? errors.password"
        @input="errors.password = undefined"
        @blur="checkPasswordsMatch"
      />
      <UiInput
        id="password-repeat"
        v-model="repeatPassword"
        type="password"
        placeholder="Repeat Password"
        autocomplete="off"
        :errors="failedRepeat"
        @blur="checkPasswordsMatch"
      />
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <UiInput
          id="name"
          v-model="form.name"
          placeholder="Name"
          autocomplete="name"
          :errors="errors.name"
          @input="errors.name = undefined"
        />
        <UiInput
          id="lastname"
          v-model="form.lastname"
          placeholder="Lastname"
          autocomplete="lastname"
          :errors="errors.lastname"
          @input="errors.lastname = undefined"
        />
      </div>
      <UiInput
        id="description"
        v-model="form.description"
        placeholder="Description"
        autocomplete="description"
        :errors="errors.description"
        @input="errors.description = undefined"
      />
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <UiInput
          id="country"
          v-model="form.country"
          placeholder="Country"
          autocomplete="country"
          :errors="errors.country"
          @input="errors.country = undefined"
        />
        <UiInput
          id="city"
          v-model="form.city"
          placeholder="City"
          autocomplete="city"
          :errors="errors.city"
          @input="errors.city = undefined"
        />
      </div>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <UiInput
          id="street"
          v-model="form.street"
          placeholder="Street"
          autocomplete="street"
          :errors="errors.street"
          @input="errors.street = undefined"
        />
        <UiInput
          id="house-number"
          v-model="form.house_number"
          placeholder="House Number"
          autocomplete="house-number"
          :errors="errors.house_number"
          @input="errors.house_number = undefined"
        />
      </div>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <UiInput
          id="apartment"
          v-model="form.apartment"
          placeholder="Apartment"
          autocomplete="apartment"
          :errors="errors.apartment"
          @input="errors.apartment = undefined"
        />
      </div>
      <UiButton
        type="submit"
        class="w-max"
      >
        Register
      </UiButton>
    </form>
  </div>
</template>

<script setup lang="ts">
import type { User } from '~/types';

defineOptions({
  name: 'RegisterPage'
});

definePageMeta({
  layout: 'clean'
});

const form = ref<User>({
  id: '',
  email: '',
  password: '',
  name: '',
  lastname: '',
  description: '',
  country: '',
  city: '',
  street: '',
  house_number: '',
  apartment: null,
  phone: '',
  phone_prefix: '',
  types: []
});

const errors = ref<Record<keyof User, string | undefined>>({
  id: undefined,
  email: undefined,
  password: undefined,
  name: undefined,
  lastname: undefined,
  description: undefined,
  city: undefined,
  country: undefined,
  street: undefined,
  house_number: undefined,
  apartment: undefined,
  phone: undefined,
  phone_prefix: undefined,
  types: undefined,
});

const repeatPassword = ref('');
const failedRepeat = ref<string>();

function checkPasswordsMatch() {
  if (repeatPassword.value === '' || form.value.password === '') {
    failedRepeat.value = undefined;
    return;
  }
  if (repeatPassword.value === form.value.password) {
    failedRepeat.value = undefined;
    return;
  }
  failedRepeat.value = 'Paswords do not match';
}

function onSubmit() {
  if (failedRepeat.value || repeatPassword.value !== form.value.password) {
    failedRepeat.value = 'Password do not match';
    return;
  }
  try {
    console.log(form.value);
    // return navigateTo('/', { external: true });
  } catch (e) {
    console.log(e);
  }
}
</script>