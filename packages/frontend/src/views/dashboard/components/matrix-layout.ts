import type { Axis, AxisKind, AxisNode, MatrixVM, Orientation, SubtotalToggles, Uuid } from '../types'

export interface MatrixLeaf {
  id: Uuid
  label: string
}

export interface GroupSlot {
  id: string
  label: string
  level: AxisNode['level']
  leafSpan: number
}

export type MatrixColumnSlot
  = | {
    kind: 'leaf'
    key: `leaf:${Uuid}`
    leaf: MatrixLeaf
    topGroupId: string | null
    midGroupId: string | null
  }
  | {
    kind: 'mid-subtotal'
    key: `mid-subtotal:${string}`
    group: GroupSlot
    topGroupId: string | null
    midGroupId: string
  }
  | {
    kind: 'top-subtotal'
    key: `top-subtotal:${string}`
    group: GroupSlot
    topGroupId: string
    midGroupId: null
  }

export interface MatrixLayout {
  rowAxis: Axis
  colAxis: Axis
  rowTotals: Map<Uuid, number>
  colTotals: Map<Uuid, number>
  rowAxisKind: AxisKind
  colAxisKind: AxisKind
  colTop: AxisNode['level'] | null
  colMid: AxisNode['level']
  colTopGroups: GroupSlot[]
  colMidGroups: GroupSlot[]
  colSlots: MatrixColumnSlot[]
  topGroupSpans: Map<string, number>
  midGroupSpans: Map<string, number>
  leafLabels: Record<AxisKind, Map<Uuid, string>>
  storageWarehouseLabels: Map<Uuid, string>
  totalCols: number
  hasColTop: boolean
}

export function deriveMatrixLayout(
  vm: MatrixVM,
  orientation: Orientation,
  subtotals: SubtotalToggles,
): MatrixLayout {
  const rowAxis = orientation === 'products-as-rows' ? vm.productAxis : vm.storageAxis
  const colAxis = orientation === 'products-as-rows' ? vm.storageAxis : vm.productAxis
  const rowTotals = orientation === 'products-as-rows' ? vm.rowTotals : vm.colTotals
  const colTotals = orientation === 'products-as-rows' ? vm.colTotals : vm.rowTotals
  const rowAxisKind = rowAxis.kind
  const colAxisKind = colAxis.kind
  const colTop = topGroupLevelFor(colAxis)
  const colMid = midGroupLevelFor(colAxis)
  const colTopGroups = colTop ? flattenGroupsAtLevel(colAxis.root, colTop) : []
  const colMidGroups = flattenGroupsAtLevel(colAxis.root, colMid)
  const colSlots = buildColumnSlots(colAxis, colTop, colMid, subtotals)

  return {
    rowAxis,
    colAxis,
    rowTotals,
    colTotals,
    rowAxisKind,
    colAxisKind,
    colTop,
    colMid,
    colTopGroups,
    colMidGroups,
    colSlots,
    topGroupSpans: countGroupSpans(colSlots, 'topGroupId'),
    midGroupSpans: countGroupSpans(colSlots, 'midGroupId'),
    leafLabels: {
      product: buildLeafLabelMap(vm.productAxis.root),
      storage: buildLeafLabelMap(vm.storageAxis.root),
    },
    storageWarehouseLabels: buildStorageWarehouseLabelMap(vm.storageAxis.root),
    totalCols: 1 + colSlots.length + 1,
    hasColTop: colTopGroups.length > 0,
  }
}

export function shouldRenderSubtotal(level: AxisNode['level'], axisKind: AxisKind, s: SubtotalToggles): boolean {
  if (axisKind === 'product') {
    if (level === 'group')
      return s.productGroup
    if (level === 'type')
      return s.productType
  }
  else {
    if (level === 'warehouse')
      return s.warehouse
    if (level === 'base')
      return s.base
  }
  return false
}

function flattenLeaves(node: AxisNode): MatrixLeaf[] {
  if (node.kind === 'leaf')
    return [{ id: node.id, label: node.label }]
  return node.children.flatMap(flattenLeaves)
}

function flattenGroupsAtLevel(node: AxisNode, level: AxisNode['level']): GroupSlot[] {
  if (node.kind === 'leaf')
    return []
  if (node.level === level) {
    return [{ id: node.id, label: node.label, level, leafSpan: flattenLeaves(node).length }]
  }
  return node.children.flatMap(c => flattenGroupsAtLevel(c, level))
}

function topGroupLevelFor(axis: Axis): AxisNode['level'] | null {
  const first = axis.root.children[0]
  if (first?.kind !== 'group')
    return null
  return first.level === 'type' || first.level === 'base' ? first.level : null
}

function midGroupLevelFor(axis: Axis): AxisNode['level'] {
  return axis.kind === 'product' ? 'group' : 'warehouse'
}

function buildColumnSlots(
  axis: Axis,
  colTop: AxisNode['level'] | null,
  colMid: AxisNode['level'],
  subtotals: SubtotalToggles,
): MatrixColumnSlot[] {
  const slots: MatrixColumnSlot[] = []
  const colMidSub = shouldRenderSubtotal(colMid, axis.kind, subtotals)
  const colTopSub = colTop ? shouldRenderSubtotal(colTop, axis.kind, subtotals) : false

  function walk(node: AxisNode, topGroup: GroupSlot | null, midGroup: GroupSlot | null) {
    if (node.kind === 'leaf') {
      slots.push({
        kind: 'leaf',
        key: `leaf:${node.id}`,
        leaf: { id: node.id, label: node.label },
        topGroupId: topGroup?.id ?? null,
        midGroupId: midGroup?.id ?? null,
      })
      return
    }

    const group = { id: node.id, label: node.label, level: node.level, leafSpan: flattenLeaves(node).length }
    const nextTop = node.level === colTop ? group : topGroup
    const nextMid = node.level === colMid ? group : midGroup

    for (const child of node.children) walk(child, nextTop, nextMid)

    if (node.level === colMid && colMidSub) {
      slots.push({
        kind: 'mid-subtotal',
        key: `mid-subtotal:${node.id}`,
        group,
        topGroupId: nextTop?.id ?? null,
        midGroupId: node.id,
      })
    }
    if (node.level === colTop && colTopSub) {
      slots.push({
        kind: 'top-subtotal',
        key: `top-subtotal:${node.id}`,
        group,
        topGroupId: node.id,
        midGroupId: null,
      })
    }
  }

  walk(axis.root, null, null)
  return slots
}

function countGroupSpans(
  slots: MatrixColumnSlot[],
  groupKey: 'topGroupId' | 'midGroupId',
): Map<string, number> {
  const spans = new Map<string, number>()
  for (const slot of slots) {
    const groupId = slot[groupKey]
    if (groupId != null)
      spans.set(groupId, (spans.get(groupId) ?? 0) + 1)
  }
  return spans
}

function buildLeafLabelMap(node: AxisNode): Map<Uuid, string> {
  const labels = new Map<Uuid, string>()
  function walk(n: AxisNode) {
    if (n.kind === 'leaf') {
      labels.set(n.id, n.label)
      return
    }
    for (const child of n.children) walk(child)
  }
  walk(node)
  return labels
}

function buildStorageWarehouseLabelMap(root: AxisNode): Map<Uuid, string> {
  const labels = new Map<Uuid, string>()
  function walk(node: AxisNode, warehouseLabel: string | null) {
    if (node.kind === 'leaf') {
      if (warehouseLabel != null)
        labels.set(node.id, warehouseLabel)
      return
    }

    const nextWarehouseLabel = node.level === 'warehouse' ? node.label : warehouseLabel
    for (const child of node.children) walk(child, nextWarehouseLabel)
  }
  walk(root, null)
  return labels
}
