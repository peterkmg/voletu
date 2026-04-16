// packages/frontend/src/views/dashboard/build-matrix-vm.ts
import type {
  Axis,
  AxisKind,
  AxisNode,
  BuilderInput,
  GroupNode,
  LeafNode,
  MatrixVM,
  Uuid,
} from './types'

// --- Parsing ---------------------------------------------------------------

function parseAmount(v: string | number | null | undefined): number {
  if (v == null || v === '')
    return 0
  const n = typeof v === 'string' ? Number.parseFloat(v) : v
  return Number.isNaN(n) ? 0 : n
}

// --- Axis node helpers -----------------------------------------------------

function mkLeaf(level: LeafNode['level'], id: Uuid, label: string, sortKey?: string): LeafNode {
  return { kind: 'leaf', level, id, label, sortKey: sortKey ?? label.toLowerCase() }
}

function mkGroup(level: GroupNode['level'], id: string, label: string, children: AxisNode[], sortKey?: string): GroupNode {
  return { kind: 'group', level, id, label, sortKey: sortKey ?? label.toLowerCase(), children }
}

function sortByKey<T extends { sortKey: string }>(arr: T[]): T[] {
  return [...arr].sort((a, b) => a.sortKey.localeCompare(b.sortKey))
}

// --- Axis builders ---------------------------------------------------------

function buildProductAxis(input: BuilderInput, participatingProductIds: Set<Uuid>): Axis {
  const productsByGroup = new Map<Uuid, LeafNode[]>()
  const groupsById = new Map(input.productGroups.map(g => [g.id, g]))
  const typesById = new Map(input.productTypes.map(t => [t.id, t]))

  for (const p of input.products as any[]) {
    if (!participatingProductIds.has(p.id))
      continue
    const leaf = mkLeaf('product', p.id, p.commonName)
    const groupId = p.productGroupId
    const list = productsByGroup.get(groupId) ?? []
    list.push(leaf)
    productsByGroup.set(groupId, list)
  }

  if (input.showType) {
    const typeBuckets = new Map<string, GroupNode[]>()
    for (const [groupId, leaves] of productsByGroup) {
      const group = groupsById.get(groupId) as any
      const typeId = group?.productTypeId ?? 'ungrouped'
      const groupNode = mkGroup('group', groupId, (group as any)?.commonName ?? 'Ungrouped', sortByKey(leaves))
      const bucket = typeBuckets.get(typeId) ?? []
      bucket.push(groupNode)
      typeBuckets.set(typeId, bucket)
    }
    const typeNodes: GroupNode[] = []
    for (const [typeId, groups] of typeBuckets) {
      const type = typesById.get(typeId) as any
      typeNodes.push(mkGroup('type', typeId, type?.commonName ?? 'Ungrouped', sortByKey(groups)))
    }
    return { kind: 'product', root: mkGroup('root', 'product-root', '', sortByKey(typeNodes)) }
  }
  else {
    const groupNodes: GroupNode[] = []
    for (const [groupId, leaves] of productsByGroup) {
      const group = groupsById.get(groupId) as any
      groupNodes.push(mkGroup('group', groupId, group?.commonName ?? 'Ungrouped', sortByKey(leaves)))
    }
    return { kind: 'product', root: mkGroup('root', 'product-root', '', sortByKey(groupNodes)) }
  }
}

function buildStorageAxis(input: BuilderInput): Axis {
  const storagesByWarehouse = new Map<Uuid, LeafNode[]>()
  const warehousesById = new Map(input.warehouses.map(w => [w.id, w]))
  const basesById = new Map(input.bases.map(b => [b.id, b]))

  for (const s of input.storages as any[]) {
    const leaf = mkLeaf('storage', s.id, s.commonName)
    const whId = s.warehouseId
    const list = storagesByWarehouse.get(whId) ?? []
    list.push(leaf)
    storagesByWarehouse.set(whId, list)
  }

  if (input.showBase) {
    const baseBuckets = new Map<string, GroupNode[]>()
    for (const [whId, leaves] of storagesByWarehouse) {
      const wh = warehousesById.get(whId) as any
      const baseId = wh?.baseId ?? 'unassigned'
      const whNode = mkGroup('warehouse', whId, wh?.commonName ?? 'Unassigned', sortByKey(leaves))
      const bucket = baseBuckets.get(baseId) ?? []
      bucket.push(whNode)
      baseBuckets.set(baseId, bucket)
    }
    const baseNodes: GroupNode[] = []
    for (const [baseId, whs] of baseBuckets) {
      const base = basesById.get(baseId) as any
      baseNodes.push(mkGroup('base', baseId, base?.commonName ?? 'Unassigned', sortByKey(whs)))
    }
    return { kind: 'storage', root: mkGroup('root', 'storage-root', '', sortByKey(baseNodes)) }
  }
  else {
    const whNodes: GroupNode[] = []
    for (const [whId, leaves] of storagesByWarehouse) {
      const wh = warehousesById.get(whId) as any
      whNodes.push(mkGroup('warehouse', whId, wh?.commonName ?? 'Unassigned', sortByKey(leaves)))
    }
    return { kind: 'storage', root: mkGroup('root', 'storage-root', '', sortByKey(whNodes)) }
  }
}

// --- Search prune ----------------------------------------------------------

function pruneBySearch(axis: Axis, needle: string): Axis {
  function prune(node: AxisNode): AxisNode | null {
    if (node.kind === 'leaf')
      return node.label.toLowerCase().includes(needle) ? node : null
    // If this group's own label matches, keep it with all its children intact.
    if (node.level !== 'root' && node.label.toLowerCase().includes(needle))
      return node
    const kept = node.children.map(prune).filter((c): c is AxisNode => c != null)
    return kept.length === 0 ? null : { ...node, children: kept }
  }
  const prunedRoot = prune(axis.root) as GroupNode | null
  return { kind: axis.kind, root: prunedRoot ?? mkGroup('root', axis.root.id, '', []) }
}

// --- Leaf helpers ----------------------------------------------------------

function countLeaves(node: AxisNode): number {
  if (node.kind === 'leaf')
    return 1
  return node.children.reduce((acc, c) => acc + countLeaves(c), 0)
}

function collectLeafIds(node: AxisNode): Uuid[] {
  if (node.kind === 'leaf')
    return [node.id]
  return node.children.flatMap(collectLeafIds)
}

// --- Totals ----------------------------------------------------------------

function computeTotals(
  productAxis: Axis,
  storageAxis: Axis,
  cellMap: Map<string, number>,
): {
  rowTotals: Map<Uuid, number>
  colTotals: Map<Uuid, number>
  groupSubtotals: Map<AxisKind, Map<string, number>>
  cellSubtotalMap: Map<string, number>
  grandTotal: number
  nonEmptyCellCount: number
} {
  const productLeafIds = collectLeafIds(productAxis.root)
  const storageLeafIds = collectLeafIds(storageAxis.root)
  const rowTotals = new Map<Uuid, number>()
  const colTotals = new Map<Uuid, number>()
  const groupSubtotals: Map<AxisKind, Map<string, number>> = new Map([
    ['product', new Map<string, number>()],
    ['storage', new Map<string, number>()],
  ])
  // Key format: `${axisKind}:${groupId}:${crossLeafId}`
  const cellSubtotalMap = new Map<string, number>()
  let grandTotal = 0
  let nonEmptyCellCount = 0

  for (const pId of productLeafIds) {
    for (const sId of storageLeafIds) {
      const v = cellMap.get(`${pId}:${sId}`)
      if (v == null || v === 0)
        continue
      rowTotals.set(pId, (rowTotals.get(pId) ?? 0) + v)
      colTotals.set(sId, (colTotals.get(sId) ?? 0) + v)
      grandTotal += v
      nonEmptyCellCount += 1
    }
  }

  // Subtotals always computed; gating happens in the presenter.
  accumulateGroupSubtotals(productAxis.root, rowTotals, groupSubtotals.get('product')!)
  accumulateGroupSubtotals(storageAxis.root, colTotals, groupSubtotals.get('storage')!)

  accumulateCellSubtotals('product', productAxis.root, storageLeafIds, cellMap, cellSubtotalMap, false)
  accumulateCellSubtotals('storage', storageAxis.root, productLeafIds, cellMap, cellSubtotalMap, true)

  return { rowTotals, colTotals, groupSubtotals, cellSubtotalMap, grandTotal, nonEmptyCellCount }
}

/**
 * Populate cellSubtotalMap with sums keyed by `${axis}:${groupId}:${crossLeafId}`.
 * For every non-root group on `axis`, iterate its leaves and sum each leaf's
 * cell value against every cross-axis leaf.
 *
 * `axisOrderIsStorageFirst` controls how the cellMap key is constructed: the
 * cellMap is keyed `${productId}:${storageId}` regardless of axis, so when
 * walking the storage axis we must swap.
 */
function accumulateCellSubtotals(
  axis: AxisKind,
  node: AxisNode,
  crossLeafIds: Uuid[],
  cellMap: Map<string, number>,
  out: Map<string, number>,
  axisOrderIsStorageFirst: boolean,
): void {
  if (node.kind === 'leaf')
    return
  if (node.level !== 'root') {
    const leafIds = collectLeafIds(node)
    for (const crossId of crossLeafIds) {
      let sum = 0
      for (const leafId of leafIds) {
        const key = axisOrderIsStorageFirst
          ? `${crossId}:${leafId}` // walking storage axis: leafId is storageId, crossId is productId
          : `${leafId}:${crossId}` // walking product axis: leafId is productId, crossId is storageId
        const v = cellMap.get(key)
        if (v != null)
          sum += v
      }
      if (sum !== 0) {
        out.set(`${axis}:${node.id}:${crossId}`, sum)
      }
    }
  }
  for (const child of node.children) {
    accumulateCellSubtotals(axis, child, crossLeafIds, cellMap, out, axisOrderIsStorageFirst)
  }
}

function accumulateGroupSubtotals(
  node: AxisNode,
  leafTotals: Map<Uuid, number>,
  out: Map<string, number>,
): number {
  if (node.kind === 'leaf')
    return leafTotals.get(node.id) ?? 0
  let sum = 0
  for (const child of node.children) {
    sum += accumulateGroupSubtotals(child, leafTotals, out)
  }
  if (node.level !== 'root') {
    out.set(node.id, sum)
  }
  return sum
}

// --- Entry point -----------------------------------------------------------

export function buildMatrixVM(input: BuilderInput): MatrixVM {
  const ledger = input.contractorId
    ? (input.ledgerEntries as any[]).filter(e => e.contractorId === input.contractorId)
    : []

  const productById = new Map(input.products.map(p => [p.id, p]))
  const storageById = new Map(input.storages.map(s => [s.id, s]))

  let orphanCount = 0
  const participating = new Set<Uuid>()
  const cellMap = new Map<string, number>()

  for (const e of ledger) {
    if (!productById.has(e.productId) || !storageById.has(e.storageId)) {
      orphanCount += 1
      continue
    }
    const amt = parseAmount(e.currentAmount)
    if (amt !== 0)
      participating.add(e.productId)
    const k = `${e.productId}:${e.storageId}`
    cellMap.set(k, (cellMap.get(k) ?? 0) + amt)
  }

  let productAxis = buildProductAxis(input, participating)
  let storageAxis = buildStorageAxis(input)

  const q = input.searchQuery.trim().toLowerCase()
  if (q) {
    productAxis = pruneBySearch(productAxis, q)
    storageAxis = pruneBySearch(storageAxis, q)
  }

  const cell = (productLeafId: Uuid, storageLeafId: Uuid) =>
    cellMap.get(`${productLeafId}:${storageLeafId}`)

  const { rowTotals, colTotals, groupSubtotals, cellSubtotalMap, grandTotal, nonEmptyCellCount }
    = computeTotals(productAxis, storageAxis, cellMap)

  const cellSubtotal = (axis: AxisKind, groupId: string, crossLeafId: Uuid) =>
    cellSubtotalMap.get(`${axis}:${groupId}:${crossLeafId}`)

  return {
    productAxis,
    storageAxis,
    cell,
    rowTotals,
    colTotals,
    groupSubtotals,
    cellSubtotal,
    grandTotal,
    stats: {
      leafRowCount: countLeaves(productAxis.root),
      leafColCount: countLeaves(storageAxis.root),
      nonEmptyCellCount,
      orphanCount,
    },
  }
}
