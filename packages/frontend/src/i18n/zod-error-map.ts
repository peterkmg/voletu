import type { $ZodErrorMap } from 'zod/v4/core'
import i18next from 'i18next'
import { z } from 'zod'
import { isFormsValidationMessageKey, toFormsValidationTranslationKey } from './form-validation-message'

function translateValidationKey(key: string) {
  const translationKey = toFormsValidationTranslationKey(key)
  const translated = i18next.t(translationKey, { defaultValue: key })
  // In dev, surface a missing-key warning so we add the entry to
  // `locales/<lang>/forms.json` instead of letting the raw key slip into
  // the UI. The fallback (`defaultValue: key`) already protects production.
  if (import.meta.env?.DEV && translated === key) {
    console.warn(`[i18n] Missing translation for ${translationKey} (key: ${key})`)
  }
  return translated
}

/**
 * Maps Zod (v4) issues to i18n keys under `forms.validation.*`.
 * The translated value is what RHF surfaces via FormMessage.
 *
 * Zod v4 issue shape changes vs v3:
 *   - `invalid_string` -> `invalid_format` (with `format` instead of `validation`)
 *   - `invalid_enum_value` -> `invalid_value`
 *   - `too_small`/`too_big` carry `origin` (e.g. "string"|"number"|"array") instead of `type`
 *   - the error map signature is single-argument (no separate `ctx`)
 */
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
      // Custom messages from .refine() are already i18n keys per spec §5.7
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

// Re-export `z` for convenience so callers can `import { z } from '~/i18n/zod-error-map'`
// if they need the same instance the error map is registered on.
export { z }
