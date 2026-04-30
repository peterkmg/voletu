const DATE_TIME_LOCAL_PATTERN = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$/
const DATE_TIME_LOCAL_WITH_SECONDS_PATTERN = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?$/

function formatLocalDateTime(date: Date): string {
  const year = String(date.getFullYear())
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  const hour = String(date.getHours()).padStart(2, '0')
  const minute = String(date.getMinutes()).padStart(2, '0')

  return `${year}-${month}-${day}T${hour}:${minute}`
}

export function toDateTimeLocalValue(value: string | null | undefined): string {
  if (!value)
    return ''

  if (DATE_TIME_LOCAL_PATTERN.test(value))
    return value

  if (DATE_TIME_LOCAL_WITH_SECONDS_PATTERN.test(value))
    return value.slice(0, 16)

  const parsed = new Date(value)
  if (Number.isNaN(parsed.getTime()))
    return ''

  return formatLocalDateTime(parsed)
}
