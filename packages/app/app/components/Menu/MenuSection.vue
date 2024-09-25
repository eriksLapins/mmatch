<template>
  <div
    class="h-dvh relative transition-[width] delay-100 duration-200 bg-gray-100 max-md:absolute"
    :class="{
      'w-14': !isExpanded,
      'w-full md:w-1/4': isExpanded
    }"
  >
    <div
      class="flex flex-col justify-between transition-[visibility]"
      :class="{
        'invisible': !isExpanded,
        'visible delay-200': isExpanded,
      }"
    >
      <div
        class="h-14 p-3 flex justify-start items-center gap-2"
      >
        <span>{{ currentUser.name }} {{ currentUser.lastname }}</span>
        <div
          ref="menuHeadingRef"
          class="relative"
        >
          <button
            class="rounded-sm w-max px-1 py-0.5 text-sm leading-none bg-green-200 inline-flex"
            @click="handleTypeClick"
          >
            {{ activeType }} <IArrowDown
              v-if="currentUser.types.length > 1"
              class="size-4 transition-transform duration-200"
              :class="{
                'rotate-180': typeDropdownOpen
              }"
            />
          </button>
          <div
            v-if="currentUser.types.length > 1 && typeDropdownOpen"
            class="absolute top-6 p-1 bg-white rounded-lg shadow-md mt-1"
          >
            <button
              v-for="type in currentUser.types"
              :key="type"
              class="w-full h-max text-sm hover:underline"
              @click="navigateTo({query: {
                userType: type
              }}, {
                external: true
              })"
            >
              {{ type }}
            </button>
          </div>
        </div>
      </div>
      <nav class="py-3">
        <ul class="">
          <li
            v-for="(link, index) in links"
            :key="link.title"
            class="
              min-h-11
              flex items-center justify-start
              hover:bg-gray-200 has-[.router-link-active]:bg-gray-200
              border-b-2 border-solid border-b-primary
              flex-col
            "
          >
            <div class="w-full flex gap-2 flex-nowrap">
              <NuxtLink
                :href="link.link" 
                class="px-3 py-2 font-semibold text-black w-full flex gap-2"
              >
                <div
                  v-if="link.prependIcon"
                  class="flex w-max justify-center items-center"
                >
                  <ISearch
                    v-if="link.prependIcon === 'search'"
                    class="size-4"
                  />
                </div>
                {{ link.title }}
              </NuxtLink>
              <button
                v-if="link.links"
                class="w-max flex justify-center items-center"
                @click="openDropdown(index)"
              >
                <IArrowDown
                  class="size-8 transition-transform duration-100 mx-2"
                  :class="{
                    'rotate-180': openedDropdown === index
                  }"
                />
              </button>
            </div>
            <div
              v-if="link.links"
              class="transition-[height] duration-200 w-full"
              :class="{
                'h-0': openedDropdown !== index,
                'h-full': openedDropdown === index
              }"
            >
              <ul
                class="transition-[visibility] duration-100"
                :class="{
                  'invisible': openedDropdown !== index || !isExpanded,
                  'visible': openedDropdown === index,
                  'delay-100': isExpanded
                }"
              >
                <li
                  v-for="childLink in link.links"
                  :key="childLink.title"
                  class="
                    min-h-11 pl-3
                    flex items-center justify-start
                    bg-gray-100
                    hover:bg-gray-200 has-[.router-link-exact-active]:bg-gray-200
                    border-b border-solid border-b-primary
                  "
                >
                  <NuxtLink
                    :href="childLink.link" 
                    class="px-3 py-2 font-semibold text-black w-full"
                  >
                    {{ childLink.title }}
                  </NuxtLink>
                </li>
              </ul>
            </div>
          </li>
        </ul>
      </nav>
    </div>
    <button
      class="absolute top-3 right-4 flex justify-center items-center"
      @click="isExpanded = !isExpanded"
    >
      <IClose
        v-if="isExpanded"
        class="size-8"
      />
      <IMenu
        v-else
        class="size-6"
      />
    </button>
  </div>
</template>

<script setup lang="ts">
import { UserTypes, type User } from '~/types';
import { onClickOutside } from '@vueuse/core';

defineOptions({
  name: 'MenuSection'
});

type MenuLink = {
  link: string,
  title: string,
  links?: MenuLink[],
  prependIcon?: 'search',
};

const isExpanded = ref(false);
const route = useRoute();
const openedDropdown = ref<number | undefined>();
const menuHeadingRef = ref<HTMLDivElement>();
onClickOutside(menuHeadingRef, () => {
  typeDropdownOpen.value = false;
});

const currentUser = computed((): User => ({ 
  name: 'Some',
  lastname: 'Dude',
  password: '',
  city: '',
  apartment: '',
  country: '',
  email: '',
  houseNumber: '',
  phone: '',
  phoneCountry: '',
  street: '',
  types: [UserTypes.Musician, UserTypes.Manager]
}));

const activeType = ref(route.query.userType as UserTypes ?? currentUser.value.types[0]);

function openDropdown(index: number) {
  if (openedDropdown.value === index) {
    openedDropdown.value = undefined;
  } else {
    openedDropdown.value = index;
  }
}

const typeDropdownOpen = ref(false);

function handleTypeClick() {
  if (typeDropdownOpen.value === true) {
    typeDropdownOpen.value = false;
    return;
  }
  if (currentUser.value.types.length === 1) {
    return;
  }
  typeDropdownOpen.value = true;

}

const links = computed((): MenuLink[] => {
  return [
    {
      link: '/',
      title: 'My profile',
    },
    {
      link: '/bands',
      title: 'Bands',
      links: [
        {
          link: '/bands/1',
          title: 'Band 1'
        },
        {
          link: '/bands/2',
          title: 'Band 2'
        }
      ]
    },
    {
      link: '/chats',
      title: 'Chats',
    },
    
    {
      link: '/explore',
      title: 'Explore',
      prependIcon: 'search'
    },
  ];
});
</script>