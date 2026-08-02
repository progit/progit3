// @ts-check
import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';

// GitHub Pages project site: https://progit.github.io/progit3/
// Override SITE_URL / BASE_PATH for forks or a custom domain.
const site = process.env.SITE_URL ?? 'https://progit.github.io';
const base = process.env.BASE_PATH ?? '/progit3';

export default defineConfig({
  site,
  base,
  trailingSlash: 'ignore',
  integrations: [mdx()],
  build: {
    format: 'directory',
  },
});
