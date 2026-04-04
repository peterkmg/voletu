/**
 * Pastel badge color maps for enum/status values.
 * Each value maps to Tailwind classes for background, text, and border.
 * Uses moderately pastel colors with /30 opacity that work in both light and dark themes.
 */

export type BadgeColorMap = Record<string, string>

export const documentStatusColors: BadgeColorMap = {
  DRAFT: 'bg-slate-100/50 text-slate-700 border-slate-200 dark:bg-slate-800/40 dark:text-slate-300 dark:border-slate-700',
  POSTED: 'bg-emerald-100/30 text-emerald-800 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800',
}

export const companyRoleColors: BadgeColorMap = {
  isContractor: 'bg-blue-100/30 text-blue-800 border-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:border-blue-800',
  isExporter: 'bg-violet-100/30 text-violet-800 border-violet-200 dark:bg-violet-900/30 dark:text-violet-300 dark:border-violet-800',
  isManufacturer: 'bg-teal-100/30 text-teal-800 border-teal-200 dark:bg-teal-900/30 dark:text-teal-300 dark:border-teal-800',
  isSender: 'bg-amber-100/30 text-amber-800 border-amber-200 dark:bg-amber-900/30 dark:text-amber-300 dark:border-amber-800',
}

export const arrivalTypeColors: BadgeColorMap = {
  TRUCK: 'bg-blue-100/30 text-blue-800 border-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:border-blue-800',
  RAIL: 'bg-violet-100/30 text-violet-800 border-violet-200 dark:bg-violet-900/30 dark:text-violet-300 dark:border-violet-800',
  PIPELINE: 'bg-teal-100/30 text-teal-800 border-teal-200 dark:bg-teal-900/30 dark:text-teal-300 dark:border-teal-800',
  WATER: 'bg-cyan-100/30 text-cyan-800 border-cyan-200 dark:bg-cyan-900/30 dark:text-cyan-300 dark:border-cyan-800',
}

export const dispatchMethodColors: BadgeColorMap = {
  TRUCK: 'bg-blue-100/30 text-blue-800 border-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:border-blue-800',
  RAIL: 'bg-violet-100/30 text-violet-800 border-violet-200 dark:bg-violet-900/30 dark:text-violet-300 dark:border-violet-800',
  PIPELINE: 'bg-teal-100/30 text-teal-800 border-teal-200 dark:bg-teal-900/30 dark:text-teal-300 dark:border-teal-800',
  VESSEL: 'bg-cyan-100/30 text-cyan-800 border-cyan-200 dark:bg-cyan-900/30 dark:text-cyan-300 dark:border-cyan-800',
  BUNKER: 'bg-orange-100/30 text-orange-800 border-orange-200 dark:bg-orange-900/30 dark:text-orange-300 dark:border-orange-800',
}

export const dispatchPurposeColors: BadgeColorMap = {
  SALE: 'bg-emerald-100/30 text-emerald-800 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800',
  TRANSIT: 'bg-sky-100/30 text-sky-800 border-sky-200 dark:bg-sky-900/30 dark:text-sky-300 dark:border-sky-800',
  RETURN: 'bg-rose-100/30 text-rose-800 border-rose-200 dark:bg-rose-900/30 dark:text-rose-300 dark:border-rose-800',
  INTERNAL: 'bg-slate-100/50 text-slate-700 border-slate-200 dark:bg-slate-800/40 dark:text-slate-300 dark:border-slate-700',
}

export const pipelineStatusColors: BadgeColorMap = {
  PENDING: 'bg-amber-100/30 text-amber-800 border-amber-200 dark:bg-amber-900/30 dark:text-amber-300 dark:border-amber-800',
  DRAFT: 'bg-slate-100/50 text-slate-700 border-slate-200 dark:bg-slate-800/40 dark:text-slate-300 dark:border-slate-700',
  EXECUTED: 'bg-emerald-100/30 text-emerald-800 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800',
}

export const entityActiveColors: BadgeColorMap = {
  active: 'bg-emerald-100/30 text-emerald-800 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800',
  archived: 'bg-slate-100/50 text-slate-700 border-slate-200 dark:bg-slate-800/40 dark:text-slate-300 dark:border-slate-700',
}

/**
 * Generic fallback: assigns colors from a palette based on value hash.
 */
const fallbackPalette = [
  'bg-blue-100/30 text-blue-800 border-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:border-blue-800',
  'bg-emerald-100/30 text-emerald-800 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800',
  'bg-violet-100/30 text-violet-800 border-violet-200 dark:bg-violet-900/30 dark:text-violet-300 dark:border-violet-800',
  'bg-amber-100/30 text-amber-800 border-amber-200 dark:bg-amber-900/30 dark:text-amber-300 dark:border-amber-800',
  'bg-rose-100/30 text-rose-800 border-rose-200 dark:bg-rose-900/30 dark:text-rose-300 dark:border-rose-800',
  'bg-teal-100/30 text-teal-800 border-teal-200 dark:bg-teal-900/30 dark:text-teal-300 dark:border-teal-800',
]

export function getBadgeColor(value: string, colorMap?: BadgeColorMap): string {
  if (colorMap && value in colorMap)
    return colorMap[value]!
  // Stable hash for consistent color assignment
  let hash = 0
  for (let i = 0; i < value.length; i++) {
    hash = ((hash << 5) - hash + value.charCodeAt(i)) | 0
  }
  return fallbackPalette[Math.abs(hash) % fallbackPalette.length]!
}
