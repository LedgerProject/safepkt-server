import { Component, Watch, Vue } from 'nuxt-property-decorator'
import { Route } from 'vue-router/types/router'

@Component
class MetaMixin extends Vue {
    meta: any

    @Watch('$route')
    onRouteChange (newRoute?: Route) {
      this.updateMeta(newRoute)
    }

    get getMeta (): (title: string) => any {
      return (title: string) => {
        const twitter_handle = '@pkt_cash'
        const description = 'Static analysis tools for rust-based smart programs.'
        const titleSuffix = ' - SafePKT'
        const getTitle = (title: string) => `${title}${titleSuffix}`

        return {
          title: getTitle(title),
          htmlAttrs: {
            lang: 'en'
          },
          meta: [
            { charset: 'utf-8' },
            { name: 'viewport', content: 'width=device-width, initial-scale=1' },
            { hid: 'description', name: 'description', content: '' },
            { hid: 'author', name: 'author', content: twitter_handle },
            {
              hid: 'og:url',
              property: 'og:url',
              content: 'https://pkt.cash'
            },
            {
              hid: 'twitter:creator',
              name: 'twitter:creator',
              content: twitter_handle
            },
            {
              hid: 'twitter:title',
              name: 'twitter:title',
              content: getTitle(title)
            },
            {
              hid: 'twitter:description',
              name: 'twitter:description',
              content: description
            },
            { name: 'format-detection', content: 'telephone=no' }
          ],
          noscript: [
            {
              innerHTML:
                            'SafePKT requires JavaScript to work as intended.'
            }
          ],
          link: [
            { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
          ]
        }
      }
    }

    head () {
      const route: Route = this.$route
      this.updateMeta(route)

      return this.meta
    }

    updateMeta (route?: Route) {
      if (!route || !route.name || ['index', 'homepage'].includes(route.name)) {
        this.meta = this.getMeta('Welcome!')
        return
      }

      const suffix = `for project having id: "${route.params.projectId}"`

      if (route.name === 'llvm-bitcode-generation') {
        this.meta = this.getMeta(`LLVM bitcode generation ${suffix}`)
        return
      }

      if (route.name === 'symbolic-execution') {
        this.meta = this.getMeta(`Symbolic execution ${suffix}`)
      }
    }
}

export default MetaMixin
