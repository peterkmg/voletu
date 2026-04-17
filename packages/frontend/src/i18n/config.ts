import i18n from 'i18next'
import { initReactI18next } from 'react-i18next'
import { z } from 'zod'

import acceptanceEn from './locales/en/acceptance.json'
import authEn from './locales/en/auth.json'
import blendingEn from './locales/en/blending.json'
import catalogEn from './locales/en/catalog.json'
import commonEn from './locales/en/common.json'
import dashboardEn from './locales/en/dashboard.json'
import directDispatchEn from './locales/en/direct-dispatch.json'
import documentsEn from './locales/en/documents.json'
import formsEn from './locales/en/forms.json'
import ownershipTransferEn from './locales/en/ownership-transfer.json'
import physicalTransferEn from './locales/en/physical-transfer.json'
import railReceiptEn from './locales/en/rail-receipt.json'
import reconciliationEn from './locales/en/reconciliation.json'
import systemEn from './locales/en/system.json'
import transportEn from './locales/en/transport.json'
import truckDispatchEn from './locales/en/truck-dispatch.json'
import truckReceiptEn from './locales/en/truck-receipt.json'

import acceptanceRu from './locales/ru/acceptance.json'
import authRu from './locales/ru/auth.json'
import blendingRu from './locales/ru/blending.json'
import catalogRu from './locales/ru/catalog.json'
import commonRu from './locales/ru/common.json'
import dashboardRu from './locales/ru/dashboard.json'
import directDispatchRu from './locales/ru/direct-dispatch.json'
import documentsRu from './locales/ru/documents.json'
import formsRu from './locales/ru/forms.json'
import ownershipTransferRu from './locales/ru/ownership-transfer.json'
import physicalTransferRu from './locales/ru/physical-transfer.json'
import railReceiptRu from './locales/ru/rail-receipt.json'
import reconciliationRu from './locales/ru/reconciliation.json'
import systemRu from './locales/ru/system.json'
import transportRu from './locales/ru/transport.json'
import truckDispatchRu from './locales/ru/truck-dispatch.json'
import truckReceiptRu from './locales/ru/truck-receipt.json'

import { zodI18nErrorMap } from './zod-error-map'

i18n.use(initReactI18next).init({
  lng: localStorage.getItem('voletu.language') ?? 'en',
  fallbackLng: 'en',
  ns: ['common', 'catalog', 'documents', 'transport', 'system', 'auth', 'dashboard', 'forms', 'acceptance', 'truck-receipt', 'rail-receipt', 'physical-transfer', 'ownership-transfer', 'truck-dispatch', 'direct-dispatch', 'blending', 'reconciliation'],
  defaultNS: 'common',
  interpolation: {
    escapeValue: false,
  },
  resources: {
    en: {
      'common': commonEn,
      'auth': authEn,
      'catalog': catalogEn,
      'dashboard': dashboardEn,
      'documents': documentsEn,
      'forms': formsEn,
      'transport': transportEn,
      'system': systemEn,
      'acceptance': acceptanceEn,
      'truck-receipt': truckReceiptEn,
      'rail-receipt': railReceiptEn,
      'physical-transfer': physicalTransferEn,
      'ownership-transfer': ownershipTransferEn,
      'truck-dispatch': truckDispatchEn,
      'direct-dispatch': directDispatchEn,
      'blending': blendingEn,
      'reconciliation': reconciliationEn,
    },
    ru: {
      'common': commonRu,
      'auth': authRu,
      'catalog': catalogRu,
      'dashboard': dashboardRu,
      'documents': documentsRu,
      'forms': formsRu,
      'transport': transportRu,
      'system': systemRu,
      'acceptance': acceptanceRu,
      'truck-receipt': truckReceiptRu,
      'rail-receipt': railReceiptRu,
      'physical-transfer': physicalTransferRu,
      'ownership-transfer': ownershipTransferRu,
      'truck-dispatch': truckDispatchRu,
      'direct-dispatch': directDispatchRu,
      'blending': blendingRu,
      'reconciliation': reconciliationRu,
    },
  },
})

z.setErrorMap(zodI18nErrorMap)

export default i18n
