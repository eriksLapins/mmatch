<template>
  <div class="w-full h-dvh flex justify-center items-center">
    <form
      class="h-[320px] w-[500px] space-y-6 p-6 align-top"
      @submit.prevent="submitForm"
    >
      <UiInput
        id="email"
        v-model="login.email"
        type="email"
        placeholder="E-mail"
        autocomplete="email"
        :errors="!!errorMessage"
        @input="errorMessage = undefined"
      />
      <UiInput
        id="password"
        v-model="login.password"
        type="password"
        placeholder="Password"
        autocomplete="password"
        :errors="!!errorMessage"
        @input="errorMessage = undefined"
      />
      <UiButton
        type="submit"     
      >
        Login
      </UiButton>
      <div
        v-if="errorMessage"
        class="text-red-500"
      >
        {{ errorMessage }}
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
defineOptions({
  name: 'LoginPage',
});

const login = ref<{
  email: string;
  password: string;
}>({
  email: '',
  password: ''
});

const errorMessage = ref();

async function submitForm() {
  try {
    console.log(login.value);
    return navigateTo('/', { external: true });
  } catch (e) {
    console.log(e);
  }
}

useSeoMeta({
  title: 'Login',
  robots: 'noindex, nofollow'
});
</script>