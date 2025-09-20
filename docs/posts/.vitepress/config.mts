import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
base: '/zoomer/',
  title: "Zoomer",
  description: "A VitePress Site",
  themeConfig: {
      search: {
	      provider: 'local'
    },
      footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2025-present YrnCollo'
    },

    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Examples', link: '/markdown-examples' },
      { text: 'Examples', link: '/markdown-examples' }
    ],

    sidebar: [
      {
        text: 'Examples',
        items: [
          { text: 'Markdown Examples', link: '/markdown-examples' },
          { text: 'Markdown ', link: '/markdown-examples' },
          { text: 'Runtime API Examples', link: '/api-examples' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/yrncollo/zoomer' }
    ]
  }
})
