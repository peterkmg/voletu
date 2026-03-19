import i18n from 'i18next'
import { initReactI18next } from 'react-i18next'

import authEn from './locales/en/auth.json'
import catalogEn from './locales/en/catalog.json'
import commonEn from './locales/en/common.json'
import documentsEn from './locales/en/documents.json'
import systemEn from './locales/en/system.json'
import transportEn from './locales/en/transport.json'

import authRu from './locales/ru/auth.json'
import catalogRu from './locales/ru/catalog.json'
import commonRu from './locales/ru/common.json'
import documentsRu from './locales/ru/documents.json'
import systemRu from './locales/ru/system.json'
import transportRu from './locales/ru/transport.json'

i18n.use(initReactI18next).init({
  lng: localStorage.getItem('voletu.language') ?? 'en',
  fallbackLng: 'en',
  ns: ['common', 'catalog', 'documents', 'transport', 'system', 'auth'],
  defaultNS: 'common',
  interpolation: {
    escapeValue: false,
  },
  resources: {
    en: {
      common: commonEn,
      auth: authEn,
      catalog: catalogEn,
      documents: documentsEn,
      transport: transportEn,
      system: systemEn,
    },
    ru: {
      common: commonRu,
      auth: authRu,
      catalog: catalogRu,
      documents: documentsRu,
      transport: transportRu,
      system: systemRu,
    },
  },
})

export default i18n
