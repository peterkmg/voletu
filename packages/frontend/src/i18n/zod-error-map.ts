import type { $ZodErrorMap } from 'zod/v4/core'
import i18next from 'i18next'
import { z } from 'zod'
import { isFormsValidationMessageKey, toFormsValidationTranslationKey } from './form-validation-message'

function translateValidationKey(key: string) {
  const translationKey = toFormsValidationTranslationKey(key)
  const translated = i18next.t(translationKey, { defaultValue: key })

  if (import.meta.env?.DEV && translated === key) {
    console.warn(`[i18n] Missing translation for ${translationKey} (key: ${key})`)
  }
  return translated
}

export const zodI18nErrorMap: $ZodErrorMap = (issue) => {
  let key = 'forms.validation.invalid'
  switch (issue.code) {
    case 'invalid_type':
      key = `forms.validation.invalidType.${String(issue.expected)}`
      break
    case 'too_small':
      key = `forms.validation.tooSmall.${String(issue.origin)}`
      break
    case 'too_big':
      key = `forms.validation.tooBig.${String(issue.origin)}`
      break
    case 'invalid_format':
      key = `forms.validation.invalidString.${String(issue.format)}`
      break
    case 'invalid_value':
      key = 'forms.validation.invalidEnum'
      break
    case 'custom':

      return {
        message: issue.message && isFormsValidationMessageKey(issue.message)
          ? translateValidationKey(issue.message)
          : (issue.message ?? key),
      }
    default:
      break
  }
  const translated = translateValidationKey(key)
  return { message: translated }
}

export { z }
