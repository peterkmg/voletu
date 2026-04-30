import type { AxisLevel, AxisNode } from './types'

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
