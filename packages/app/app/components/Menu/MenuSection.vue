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
      <div class="h-14 p-3  flex justify-start items-center">
        Hello title
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
                class="px-3 py-2 font-semibold text-black w-full"
              >
                {{ link.title }}
              </NuxtLink>
              <button
                v-if="link.links"
                class="w-max flex justify-center items-center"
                @click="openDropdown(index)"
              >
                <IArrowDown
                  class="size-8 transition-transform duration-100"
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
                    min-h-11
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
defineOptions({
  name: 'MenuSection'
});

type MenuLink = {
  link: string,
  title: string,
  links?: MenuLink[]
};

const isExpanded = ref(false);
const openedDropdown = ref<number | undefined>();
function openDropdown(index: number) {
  if (openedDropdown.value === index) {
    openedDropdown.value = undefined;
  } else {
    openedDropdown.value = index;
  }
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
  ];
});
</script>