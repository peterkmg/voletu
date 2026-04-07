import type { ClassValue } from 'clsx'
import { clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

/** Matches one or more trailing slashes. Shared across API client, auth, and setup. */
export const TRAILING_SLASHES = /\/+$/

export function getPageNumbers(currentPage: number, totalPages: number) {
  const pages: (number | '...')[] = []
  if (totalPages <= 7) {
    for (let i = 1; i <= totalPages; i++) pages.push(i)
    return pages
  }
  pages.push(1)
  if (currentPage > 3)
    pages.push('...')
  const start = Math.max(2, currentPage - 1)
  const end = Math.min(totalPages - 1, currentPage + 1)
  for (let i = start; i <= end; i++) pages.push(i)
  if (currentPage < totalPages - 2)
    pages.push('...')
  pages.push(totalPages)
  return pages
}
