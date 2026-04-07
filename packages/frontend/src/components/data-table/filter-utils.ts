export type FilterType = 'text' | 'date' | 'number' | 'enum'

const ISO_DATE_PREFIX = /^\d{4}-\d{2}-\d{2}/

export function detectFilterType(facetedValues: Map<unknown, number>): FilterType {
  for (const [value] of facetedValues) {
    if (value == null)
      continue
    if (typeof value === 'number')
      return 'number'
    if (typeof value === 'string') {
      if (ISO_DATE_PREFIX.test(value))
        return 'date'
    }
    break
  }
  return 'text'
}
