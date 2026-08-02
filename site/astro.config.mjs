// @ts-check
import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';

// GitHub Pages with a custom domain: https://progit3.com/
// Override SITE_URL / BASE_PATH for forks (e.g. https://<user>.github.io and /progit3).
const site = process.env.SITE_URL ?? 'https://progit3.com';
const base = process.env.BASE_PATH ?? '/';

export default defineConfig({
  site,
  base,
  trailingSlash: 'ignore',
  integrations: [mdx()],
  build: {
    format: 'directory',
  },
});
