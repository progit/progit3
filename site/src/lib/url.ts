/** Site base path (e.g. `/progit3`), without a trailing slash. */
export const base = import.meta.env.BASE_URL.replace(/\/+$/, '');

/** Prefix a site-absolute path with the base path. */
export function url(path: string): string {
  return `${base}${path.startsWith('/') ? '' : '/'}${path}`;
}
