import { defineNuxtModule, createResolver, addComponentsDir } from '@nuxt/kit';

export default defineNuxtModule({
  meta: {
    name: '@mmatch/common'
  },
  setup() {
    const {resolve} = createResolver(import.meta.url);

    addComponentsDir({path: resolve('./components')});
    
  }
});