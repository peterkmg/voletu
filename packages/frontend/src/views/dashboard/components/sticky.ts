// Shared sticky-positioning tokens for the inventory matrix.
// Keeping these in one place prevents z-index drift between
// inventory-matrix.tsx and matrix-total-cell.tsx when new sticky cells are
// added. Layer ordering (low → high): body < bodyRight = foot < head < headRight < corner.

export const Z = {
  body: 20, // sticky-left cells in <tbody>
  bodyRight: 25, // sticky-right cells in <tbody> (row-total column)
  foot: 25, // sticky-bottom cells in <tfoot>
  head: 30, // sticky-top cells in <thead> (non-corner)
  headRight: 35, // sticky top-right (row-total) header cells
  corner: 40, // corners sticky on two axes
} as const

// Shadow strings indicate the direction scrolling content exits the sticky
// edge. `stickyLeft` sits on a left-pinned cell and casts to its right, etc.
export const EDGE_SHADOW = {
  stickyLeft: '2px 0 4px -2px var(--border)',
  stickyRight: '-2px 0 4px -2px var(--border)',
  stickyBottom: '0 -2px 4px -2px var(--border)',
  stickyBottomLeft: '2px -2px 4px -2px var(--border)',
  stickyBottomRight: '-2px -2px 4px -2px var(--border)',
} as const
