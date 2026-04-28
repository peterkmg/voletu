const formsValidationPrefix = 'forms.validation.'

export function isFormsValidationMessageKey(message: string) {
  return message.startsWith(formsValidationPrefix)
}

export function toFormsValidationTranslationKey(message: string) {
  return `forms:${message.slice('forms.'.length)}`
}
