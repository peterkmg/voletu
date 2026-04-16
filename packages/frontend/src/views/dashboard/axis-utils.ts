// packages/frontend/src/views/dashboard/axis-utils.ts
// Small pure helpers for walking the Axis tree. Used by presenter components
// and the DashboardView to resolve labels from leaf ids.

import type { AxisLevel, AxisNode } from './types'

/**
 * Find the label of a leaf node by id. Returns null if the id is not present
 * in the subtree rooted at `node`.
 */
export function findLeafLabel(node: AxisNode, id: string): string | null {
  if (node.kind === 'leaf')
    return node.id === id ? node.label : null
  for (const c of node.children) {
    const hit = findLeafLabel(c, id)
    if (hit != null)
      return hit
  }
  return null
}

/**
 * Find the label of the nearest ancestor group node at the given `level` whose
 * subtree contains the leaf with `leafId`. Returns null if no such ancestor exists.
 *
 * Example: findParentGroupLabel(storageAxis.root, storageId, 'warehouse')
 *   returns the warehouse common-name of that storage.
 */
export function findParentGroupLabel(
  node: AxisNode,
  leafId: string,
  level: AxisLevel,
): string | null {
  if (node.kind === 'leaf')
    return null
  for (const child of node.children) {
    if (child.kind === 'leaf' && child.id === leafId && node.level === level)
      return node.label
    const hit = findParentGroupLabel(child, leafId, level)
    if (hit != null)
      return hit
  }
  return null
}
