import type { Orientation } from '../types'
import { ChevronDown } from 'lucide-react'
// packages/frontend/src/views/dashboard/components/matrix-toolbar.tsx
import { useTranslation } from 'react-i18next'
import { DensityToggle } from '~/components/data-table/density'
import { EntityPickerCombobox } from '~/components/entity-picker'
import { Badge } from '~/components/ui/badge'
import { Button } from '~/components/ui/button'
import { DebouncedInput } from '~/components/ui/debounced-input'
import { Label } from '~/components/ui/label'
import { Popover, PopoverContent, PopoverTrigger } from '~/components/ui/popover'
import { Switch } from '~/components/ui/switch'
import { Tabs, TabsList, TabsTrigger } from '~/components/ui/tabs'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '~/components/ui/tooltip'
import { useDashboardStore } from '../state/dashboard-store'

export interface MatrixToolbarProps {
  contractors: Array<{ id: string, label: string }>
  searchQuery: string
  onSearchChange: (q: string) => void
}

export function MatrixToolbar({ contractors, searchQuery, onSearchChange }: MatrixToolbarProps) {
  const { t } = useTranslation('dashboard')
  const {
    contractorId,
    orientation,
    showType,
    showBase,
    productGroupTotals,
    productTypeTotals,
    warehouseTotals,
    baseTotals,
    setContractorId,
    setOrientation,
    setShowType,
    setShowBase,
    setProductGroupTotals,
    setProductTypeTotals,
    setWarehouseTotals,
    setBaseTotals,
  } = useDashboardStore()

  const structureActive = Number(showType) + Number(showBase)
  const totalsActive
    = Number(productGroupTotals) + Number(productTypeTotals)
      + Number(warehouseTotals) + Number(baseTotals)

  return (
    <TooltipProvider>
      <div className="flex flex-wrap items-center gap-3 py-2">
        <div className="w-36 md:w-48 lg:w-56">
          <DebouncedInput
            value={searchQuery}
            onChange={v => onSearchChange(String(v))}
            placeholder={t('toolbar.search')}
            debounce={200}
          />
        </div>

        <div className="w-48 md:w-64 lg:w-80">
          <EntityPickerCombobox
            value={contractorId}
            onChange={setContractorId}
            items={contractors}
            placeholder={t('toolbar.contractor')}
          />
        </div>

        <div className="ml-auto flex items-center gap-3">
          <Tabs value={orientation} onValueChange={v => setOrientation(v as Orientation)}>
            <TabsList>
              <TabsTrigger value="products-as-rows">{t('toolbar.orientation.productsAsRows')}</TabsTrigger>
              <TabsTrigger value="storages-as-rows">{t('toolbar.orientation.storagesAsRows')}</TabsTrigger>
            </TabsList>
          </Tabs>

          <DensityToggle />

          <Popover>
            <PopoverTrigger asChild>
              <Button variant="outline" size="sm" className="gap-2">
                {t('toolbar.structure.label')}
                {structureActive > 0 && <Badge variant="secondary">{structureActive}</Badge>}
                <ChevronDown className="size-4 opacity-50" />
              </Button>
            </PopoverTrigger>
            <PopoverContent className="w-64" align="start">
              <div className="flex flex-col gap-3">
                <div className="flex items-center justify-between">
                  <Label htmlFor="dash-show-type">{t('toolbar.structure.productType')}</Label>
                  <Switch id="dash-show-type" checked={showType} onCheckedChange={setShowType} />
                </div>
                <div className="flex items-center justify-between">
                  <Label htmlFor="dash-show-base">{t('toolbar.structure.base')}</Label>
                  <Switch id="dash-show-base" checked={showBase} onCheckedChange={setShowBase} />
                </div>
              </div>
            </PopoverContent>
          </Popover>

          <Popover>
            <PopoverTrigger asChild>
              <Button variant="outline" size="sm" className="gap-2">
                {t('toolbar.totals.label')}
                {totalsActive > 0 && <Badge variant="secondary">{totalsActive}</Badge>}
                <ChevronDown className="size-4 opacity-50" />
              </Button>
            </PopoverTrigger>
            <PopoverContent className="w-72" align="start">
              <div className="flex flex-col gap-3">
                <ToggleRow
                  id="dash-totals-group"
                  label={t('toolbar.totals.productGroup')}
                  checked={productGroupTotals}
                  onCheckedChange={setProductGroupTotals}
                />
                <ToggleRow
                  id="dash-totals-type"
                  label={t('toolbar.totals.productType')}
                  checked={productTypeTotals}
                  onCheckedChange={setProductTypeTotals}
                  disabled={!showType}
                  disabledTooltip={t('toolbar.totals.requiresType')}
                />
                <ToggleRow
                  id="dash-totals-warehouse"
                  label={t('toolbar.totals.warehouse')}
                  checked={warehouseTotals}
                  onCheckedChange={setWarehouseTotals}
                />
                <ToggleRow
                  id="dash-totals-base"
                  label={t('toolbar.totals.base')}
                  checked={baseTotals}
                  onCheckedChange={setBaseTotals}
                  disabled={!showBase}
                  disabledTooltip={t('toolbar.totals.requiresBase')}
                />
              </div>
            </PopoverContent>
          </Popover>
        </div>
      </div>
    </TooltipProvider>
  )
}

function ToggleRow({
  id,
  label,
  checked,
  onCheckedChange,
  disabled,
  disabledTooltip,
}: {
  id: string
  label: string
  checked: boolean
  onCheckedChange: (v: boolean) => void
  disabled?: boolean
  disabledTooltip?: string
}) {
  const row = (
    <div
      className="flex items-center justify-between opacity-100 data-[disabled=true]:opacity-50"
      data-disabled={disabled}
    >
      <Label htmlFor={id} className={disabled ? 'text-muted-foreground' : ''}>{label}</Label>
      <Switch id={id} checked={checked} onCheckedChange={onCheckedChange} disabled={disabled} />
    </div>
  )
  if (disabled && disabledTooltip) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>
          <div>{row}</div>
        </TooltipTrigger>
        <TooltipContent>{disabledTooltip}</TooltipContent>
      </Tooltip>
    )
  }
  return row
}
