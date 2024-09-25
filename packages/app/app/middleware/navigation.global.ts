export default defineNuxtRouteMiddleware((to, from) => {
  let addedNewQueries = false;
  const savedQueries = ['userType'];
  const retainedQueries: Record<string, string> = {};
  for (const [key, value] of Object.entries(from.query)) {
    if (value && savedQueries.includes(key) && !Object.keys(to.query).includes(key)) {
      retainedQueries[key] = value as string;
      addedNewQueries = true;
    }
  }

  if (addedNewQueries) {
    return navigateTo({
      path: to.path,
      hash: to.hash,
      query: {
        ...to.query,
        ...retainedQueries,
      },
    });
  }
});