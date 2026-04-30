import { useEffect } from 'react'

const APP_NAME = 'Voletu'

export function usePageTitle(pageTitle?: string) {
  useEffect(() => {
    document.title = pageTitle ? `${pageTitle} — ${APP_NAME}` : APP_NAME
    return () => {
      document.title = APP_NAME
    }
  }, [pageTitle])
}
