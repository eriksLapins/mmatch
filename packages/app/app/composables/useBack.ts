export function useBack(path: string) {
  const config = useRuntimeConfig();
  const headers = new Headers();
  headers.append("Content-Type", "application/json");
  console.log(headers.keys().next());
  return {
    path: config.public.backendUrl + path,
    headers,
  };
}