import type { i18n as I18n } from 'i18next'

export const languageOptions = [
  { code: 'en', label: 'English' },
  { code: 'ru', label: 'Русский' },
] as const

export function changeLanguagePreference(i18n: Pick<I18n, 'changeLanguage'>, languageCode: string) {
  i18n.changeLanguage(languageCode)
  localStorage.setItem('voletu.language', languageCode)
}
