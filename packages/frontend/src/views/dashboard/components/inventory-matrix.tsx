import type { AxisNode, GroupNode, LeafNode, MatrixVM, Orientation, SubtotalToggles, Uuid } from '../types'
import type { MatrixColumnSlot, MatrixLeaf } from './matrix-layout'
import type { TableDensity } from '~/components/data-table/density-state'
// packages/frontend/src/views/dashboard/components/inventory-matrix.tsx
import { useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { useDensity } from '~/components/data-table/density'
import { formatAmount } from '~/lib/formatters'
import { cn } from '~/lib/utils'
import { MatrixCell } from './matrix-cell'
import { deriveMatrixLayout, shouldRenderSubtotal } from './matrix-layout'
import { MatrixTotalCell } from './matrix-total-cell'
import { EDGE_SHADOW, Z } from './sticky'

export interface InventoryMatrixProps {
  vm: MatrixVM
  orientation: Orientation
  subtotals: SubtotalToggles
  onCellClick: (productId: Uuid, storageId: Uuid) => void
}

// --- Density → CSS variables ------------------------------------------------

const DENSITY_VARS: Record<TableDensity, React.CSSProperties> = {
  compact: {
    '--cell-w': '72px',
    '--row-h': '26px',
    '--row-hdr-w': '192px',
    '--cell-px': '6px',
    '--cell-py': '3px',
    '--font-size': '12px',
    '--group-row-h': '30px',
  } as React.CSSProperties,
  normal: {
    '--cell-w': '96px',
    '--row-h': '32px',
    '--row-hdr-w': '224px',
    '--cell-px': '8px',
    '--cell-py': '4px',
    '--font-size': '13px',
    '--group-row-h': '36px',
  } as React.CSSProperties,
  comfortable: {
    '--cell-w': '128px',
    '--row-h': '40px',
    '--row-hdr-w': '272px',
    '--cell-px': '12px',
    '--cell-py': '6px',
    '--font-size': '14px',
    '--group-row-h': '44px',
  } as React.CSSProperties,
}

// --- Main component ---------------------------------------------------------

export function InventoryMatrix({ vm, orientation, subtotals, onCellClick }: InventoryMatrixProps) {
  const { t } = useTranslation('dashboard')
  const { density } = useDensity()
  const { productGroup, productType, warehouse, base } = subtotals

  const layout = useMemo(
    () => deriveMatrixLayout(vm, orientation, { productGroup, productType, warehouse, base }),
    [vm, orientation, productGroup, productType, warehouse, base],
  )
  const {
    rowAxis,
    colTotals,
    rowTotals,
    rowAxisKind,
    colAxisKind,
    colTopGroups,
    colMidGroups,
    colSlots,
    topGroupSpans,
    midGroupSpans,
    leafLabels,
    storageWarehouseLabels,
    totalCols,
    hasColTop,
  } = layout

  const axisCornerLabel = rowAxisKind === 'product' ? t('toolbar.axis.products') : t('toolbar.axis.storages')
  const subtotalPrefix = t('matrix.subtotalPrefix')
  const grh = 'var(--group-row-h)'

  // --- Cell + subtotal rendering helpers ---

  function buildCellAriaLabel(rowLeafId: Uuid, colLeaf: MatrixLeaf, amt: number | undefined): string {
    const product = orientation === 'products-as-rows'
      ? leafLabels.product.get(rowLeafId) ?? ''
      : colLeaf.label
    const storage = orientation === 'products-as-rows'
      ? colLeaf.label
      : leafLabels.storage.get(rowLeafId) ?? ''
    const warehouseName = orientation === 'products-as-rows'
      ? storageWarehouseLabels.get(colLeaf.id) ?? ''
      : storageWarehouseLabels.get(rowLeafId) ?? ''
    return t('cell.ariaLabel', { product, warehouse: warehouseName, storage, amount: amt ?? '' })
  }

  function renderColValueCell(rowLeafId: Uuid, slot: MatrixColumnSlot): React.ReactNode {
    if (slot.kind === 'leaf') {
      const productId = orientation === 'products-as-rows' ? rowLeafId : slot.leaf.id
      const storageId = orientation === 'products-as-rows' ? slot.leaf.id : rowLeafId
      const amt = vm.cell(productId, storageId)
      return (
        <MatrixCell
          key={slot.key}
          productId={productId}
          storageId={storageId}
          amount={amt}
          ariaLabel={buildCellAriaLabel(rowLeafId, slot.leaf, amt)}
          onClick={() => onCellClick(productId, storageId)}
        />
      )
    }
    const amt = vm.cellSubtotal(colAxisKind, slot.group.id, rowLeafId)
    return <MatrixTotalCell key={slot.key} amount={amt} variant="col-subtotal" />
  }

  // --- <tbody> recursive renderer ---

  function renderRows(node: AxisNode, depth: number): React.ReactNode[] {
    if (node.kind === 'leaf')
      return [renderLeafRow(node, depth)]
    if (node.level === 'root')
      return node.children.flatMap(c => renderRows(c, 0))
    const rows: React.ReactNode[] = [renderGroupRow(node, depth)]
    for (const child of node.children) rows.push(...renderRows(child, depth + 1))
    if (shouldRenderSubtotal(node.level, rowAxisKind, subtotals)) {
      rows.push(renderRowSubtotal(node, depth))
    }
    return rows
  }

  function renderGroupRow(group: GroupNode, depth: number) {
    const isTop = depth === 0
    return (
      <tr
        key={`group-${group.level}-${group.id}`}
        className={cn(
          'font-medium uppercase tracking-wider border-b',
          isTop ? 'bg-muted text-xs font-semibold border-b-2 border-border' : 'bg-muted/50 text-xs border-border',
        )}
        style={{ height: grh }}
      >
        <th
          colSpan={totalCols}
          scope="rowgroup"
          className={cn(
            'text-left px-[var(--cell-px)] py-[var(--cell-py)]',
            !isTop && 'pl-[calc(var(--cell-px)*1.5)]',
          )}
          title={group.label}
        >
          {group.label}
        </th>
      </tr>
    )
  }

  function renderLeafRow(leaf: LeafNode, depth: number) {
    return (
      <tr
        key={`leaf-${leaf.id}`}
        className="leaf-row hover:bg-accent/20"
        style={{ height: 'var(--row-h)' }}
      >
        <th
          scope="row"
          className={cn(
            'sticky left-0 bg-background border-b border-r border-border/50',
            'text-left font-normal truncate',
            depth > 0 ? 'pl-[calc(var(--cell-px)*3)]' : 'px-[var(--cell-px)]',
            'py-[var(--cell-py)] text-[length:var(--font-size)]',
          )}
          style={{ width: 'var(--row-hdr-w)', zIndex: Z.body, boxShadow: EDGE_SHADOW.stickyLeft }}
          title={leaf.label}
        >
          {leaf.label}
        </th>
        {colSlots.map(slot => renderColValueCell(leaf.id, slot))}
        <MatrixTotalCell amount={rowTotals.get(leaf.id)} variant="row-total" sticky="right" />
      </tr>
    )
  }

  function renderRowSubtotal(group: GroupNode, depth: number) {
    const subtotal = vm.groupSubtotals.get(rowAxisKind)?.get(group.id)
    return (
      <tr
        key={`subtotal-${group.level}-${group.id}`}
        className="bg-muted/40 font-medium border-t border-b border-border"
        style={{ height: 'var(--row-h)' }}
      >
        <th
          scope="row"
          className={cn(
            'sticky left-0 bg-muted border-r border-border/50',
            'text-left px-[var(--cell-px)] py-[var(--cell-py)] text-[length:var(--font-size)]',
            depth > 0 && 'pl-[calc(var(--cell-px)*3)]',
          )}
          style={{ width: 'var(--row-hdr-w)', zIndex: Z.body, boxShadow: EDGE_SHADOW.stickyLeft }}
          title={group.label}
        >
          {`${subtotalPrefix} ${group.label}`}
        </th>
        {colSlots.map((slot) => {
          if (slot.kind === 'leaf') {
            const amt = vm.cellSubtotal(rowAxisKind, group.id, slot.leaf.id)
            return <MatrixTotalCell key={`rs-${slot.leaf.id}`} amount={amt} variant="row-subtotal" />
          }
          return (
            <MatrixTotalCell
              key={`rs-${slot.kind}-${slot.group.id}`}
              amount={subtotal}
              variant="subtotal-intersect"
            />
          )
        })}
        <MatrixTotalCell amount={subtotal} variant="subtotal-intersect" sticky="right" />
      </tr>
    )
  }

  // --- Render ---

  return (
    <div
      className="matrix-container flex flex-col min-h-0 flex-1"
      style={DENSITY_VARS[density]}
    >
      <div className="matrix-scroll border rounded-md bg-card overflow-auto flex-1 min-h-0">
        <table
          className="matrix-table"
          style={{
            tableLayout: 'fixed',
            width: 'max-content',
            minWidth: '100%',
            borderCollapse: 'separate',
            borderSpacing: 0,
            fontSize: 'var(--font-size)',
          }}
        >
          <colgroup>
            <col style={{ width: 'var(--row-hdr-w)' }} />
            {colSlots.map(slot => (
              <col
                key={slot.key}
                style={{ width: 'var(--cell-w)' }}
              />
            ))}
            <col style={{ width: 'var(--cell-w)' }} />
          </colgroup>

          <thead>
            {/* Row 1 — top groups (type/base), only when present */}
            {hasColTop && (
              <tr style={{ height: grh }}>
                <th
                  className="sticky left-0 bg-background border-b-2 border-r border-border/50"
                  style={{ top: 0, zIndex: Z.corner, width: 'var(--row-hdr-w)' }}
                />
                {colTopGroups.map((g) => {
                  return (
                    <th
                      key={`top-${g.id}`}
                      colSpan={topGroupSpans.get(g.id) ?? 0}
                      scope="colgroup"
                      className="bg-muted text-xs uppercase tracking-wider font-semibold text-left px-[var(--cell-px)] py-[var(--cell-py)] border-b-2 border-r border-border"
                      style={{ top: 0, position: 'sticky', zIndex: Z.head }}
                      title={g.label}
                    >
                      {g.label}
                    </th>
                  )
                })}
                <th
                  rowSpan={3}
                  scope="col"
                  className="sticky right-0 bg-muted border-b-2 border-l border-border text-right px-[var(--cell-px)] font-medium text-xs uppercase tracking-wider"
                  style={{ top: 0, zIndex: Z.headRight, boxShadow: EDGE_SHADOW.stickyRight }}
                >
                  {t('matrix.rowTotal')}
                </th>
              </tr>
            )}

            {/* Row 2 — mid groups (group/warehouse), always present */}
            <tr style={{ height: grh }}>
              <th
                className="sticky left-0 bg-background border-b border-r border-border/50"
                style={{ top: hasColTop ? grh : 0, zIndex: Z.corner, width: 'var(--row-hdr-w)' }}
              />
              {colMidGroups.map((g) => {
                return (
                  <th
                    key={`mid-${g.id}`}
                    colSpan={midGroupSpans.get(g.id) ?? 0}
                    scope="colgroup"
                    className="bg-muted text-xs uppercase font-medium text-left px-[var(--cell-px)] py-[var(--cell-py)] border-b border-r border-border"
                    style={{ top: hasColTop ? grh : 0, position: 'sticky', zIndex: Z.head }}
                    title={g.label}
                  >
                    {g.label}
                  </th>
                )
              })}
              {!hasColTop && (
                <th
                  rowSpan={2}
                  scope="col"
                  className="sticky right-0 bg-muted border-b border-l border-border text-right px-[var(--cell-px)] font-medium text-xs uppercase tracking-wider"
                  style={{ top: 0, zIndex: Z.headRight, boxShadow: EDGE_SHADOW.stickyRight }}
                >
                  {t('matrix.rowTotal')}
                </th>
              )}
            </tr>

            {/* Row 3 — leaf column headers (+ subtotal column headers interleaved) */}
            <tr style={{ height: 'var(--row-h)' }}>
              <th
                scope="col"
                className="sticky left-0 bg-background border-b-2 border-r border-border/50 text-left px-[var(--cell-px)] py-[var(--cell-py)] text-xs uppercase tracking-wider font-semibold"
                style={{ top: hasColTop ? `calc(2 * ${grh})` : grh, zIndex: Z.corner, width: 'var(--row-hdr-w)' }}
                title={axisCornerLabel}
              >
                {axisCornerLabel}
              </th>
              {colSlots.map((slot) => {
                if (slot.kind === 'leaf') {
                  return (
                    <th
                      key={`leaf-${slot.leaf.id}`}
                      scope="col"
                      className="bg-background text-xs font-normal text-muted-foreground text-right px-[var(--cell-px)] py-[var(--cell-py)] border-b-2 border-r border-border/50 truncate"
                      style={{ top: hasColTop ? `calc(2 * ${grh})` : grh, position: 'sticky', zIndex: Z.head }}
                      title={slot.leaf.label}
                    >
                      {slot.leaf.label}
                    </th>
                  )
                }
                return (
                  <th
                    key={`${slot.kind}-hdr-${slot.group.id}`}
                    scope="col"
                    className="bg-muted text-xs font-medium text-right px-[var(--cell-px)] py-[var(--cell-py)] border-b-2 border-l border-r border-border truncate"
                    style={{ top: hasColTop ? `calc(2 * ${grh})` : grh, position: 'sticky', zIndex: Z.head }}
                    title={slot.group.label}
                  >
                    {`${subtotalPrefix} ${slot.group.label}`}
                  </th>
                )
              })}
            </tr>
          </thead>

          <tbody>{renderRows(rowAxis.root, 0)}</tbody>

          <tfoot>
            <tr
              className="bg-muted font-semibold border-t-2 border-border"
              style={{ height: 'var(--row-h)' }}
            >
              <th
                scope="row"
                className="bg-muted border-b border-r border-border px-[var(--cell-px)] py-[var(--cell-py)] text-left uppercase tracking-wider text-[length:var(--font-size)]"
                style={{
                  position: 'sticky',
                  left: 0,
                  bottom: 0,
                  width: 'var(--row-hdr-w)',
                  zIndex: Z.corner,
                  boxShadow: EDGE_SHADOW.stickyBottomLeft,
                }}
              >
                {t('matrix.colTotal')}
              </th>
              {colSlots.map((slot) => {
                const amount = slot.kind === 'leaf'
                  ? colTotals.get(slot.leaf.id)
                  : vm.groupSubtotals.get(colAxisKind)?.get(slot.group.id)
                const isEmpty = amount == null || amount === 0
                const key = slot.kind === 'leaf' ? `ct-${slot.leaf.id}` : `ct-${slot.kind}-${slot.group.id}`
                const cellClass = slot.kind === 'leaf'
                  ? 'bg-muted font-medium'
                  : 'bg-muted font-semibold border-l border-r border-border'
                return (
                  <td
                    key={key}
                    className={cn(
                      'text-right tabular-nums px-[var(--cell-px)] py-[var(--cell-py)] border-b border-r border-border/50 text-[length:var(--font-size)]',
                      cellClass,
                      isEmpty && 'text-muted-foreground',
                    )}
                    style={{
                      position: 'sticky',
                      bottom: 0,
                      zIndex: Z.foot,
                      boxShadow: EDGE_SHADOW.stickyBottom,
                    }}
                  >
                    {isEmpty ? '\u2014' : formatAmount(amount)}
                  </td>
                )
              })}
              <td
                className="text-right tabular-nums px-[var(--cell-px)] py-[var(--cell-py)] border-b border-border bg-muted font-semibold text-[length:var(--font-size)]"
                style={{
                  position: 'sticky',
                  bottom: 0,
                  right: 0,
                  zIndex: Z.corner,
                  boxShadow: EDGE_SHADOW.stickyBottomRight,
                }}
              >
                {vm.grandTotal === 0 ? '\u2014' : formatAmount(vm.grandTotal)}
              </td>
            </tr>
          </tfoot>
        </table>
      </div>
    </div>
  )
}
