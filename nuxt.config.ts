import { NuxtConfig } from '@nuxt/types'

const config: NuxtConfig = {
  pattern: '**/*.{vue,js}',

  // Disable server-side rendering: https://go.nuxtjs.dev/ssr-mode
  ssr: false,

  // Target: https://go.nuxtjs.dev/config-target
  target: 'static',

  // Global CSS: https://go.nuxtjs.dev/config-css
  css: [
  ],

  // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
  plugins: [
    '~/plugins/vue-notification.ts',
    { src: '~/plugins/vuex-persist', ssr: false }
  ],

  // Auto import components: https://go.nuxtjs.dev/config-components
  components: true,

  // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
  buildModules: [
    // https://go.nuxtjs.dev/typescript
    '@nuxt/typescript-build',
    '@nuxtjs/date-fns',
    '@nuxtjs/dotenv',
    '@nuxtjs/fontawesome',
    '@nuxtjs/pwa',
    '@nuxtjs/router',
    '@nuxtjs/svg'
  ],

  fontawesome: {
    icons: {
      solid: [
        'faEye',
        'faEyeSlash',
        'faHistory',
        'faHourglass',
        'faTrash'
      ]
    }
  },

  pwa: {
    icon: {
      source: '~/assets/safepkt.png'
    },
    manifest: {
      name: 'SafePKT',
      lang: 'fr',
      short_name: 'SafePKT',
      useWebmanifestExtension: false
    },
    meta: {
      theme_color: '#3CADEF'
    }
  },

  lazySizes: {
    extendAssetUrls: {
      img: 'data-src'
    }
  },

  // Modules: https://go.nuxtjs.dev/config-modules
  modules: [
    '@nuxtjs/style-resources'
  ],

  env: {
    API_HOST: process.env.API_HOST || '',
    NODE_ENV: process.env.NODE_ENV || ''
  },

  router: {
    extendRoutes (routes, resolve: (dir: string, path: string) => string): void {
      routes.push({
        name: 'homepage',
        path: '/',
        component: resolve(__dirname, 'pages/index.vue'),
        children: [
          {
            path: 'program-verification/:projectId',
            component: resolve(__dirname, 'pages/index.vue'),
            name: 'program-verification'
          },
          {
            path: 'source-restoration/:projectId',
            component: resolve(__dirname, 'pages/index.vue'),
            name: 'source-restoration'
          },
          {
            path: 'llvm-bitcode-generation/:projectId',
            component: resolve(__dirname, 'pages/index.vue'),
            name: 'llvm-bitcode-generation'
          },
          {
            path: 'symbolic-execution/:projectId',
            component: resolve(__dirname, 'pages/index.vue'),
            name: 'symbolic-execution'
          },
          {
            path: '*',
            component: resolve(__dirname, 'pages/index.vue'),
            name: 'not-found'
          }
        ]
      })
    }
  },

  generate: {
    routes: ['/']
  },

  styleResources: {
    scss: ['./styles/variables.scss', './styles/global.scss']
  },

  // Build Configuration: https://go.nuxtjs.dev/config-build
  build: {
    babel: {
      plugins: [['@babel/plugin-proposal-private-property-in-object', { loose: true }]]
    },
    transpile: ['vue-notification']
  },

  typescript: {
    typeCheck: {
      eslint: {
        files: './**/*.{ts,js,vue}'
      }
    }
  }
}

export default config
