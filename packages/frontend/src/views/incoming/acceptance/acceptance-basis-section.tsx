/**
 * Tabbed "based on" header section for AcceptanceMutateDialog.
 *
 * The component encapsulates the discriminated-union UX described in spec
 * section 3.1: a tab strip selects `arrivalType`, and the visible
 * tab-specific field (`sourceEntity` / `truckWaybillId` / `railWaybillId`)
 * follows the active tab. Common fields (`documentNumber`, `dateAccepted`,
 * `contractorId`) are intentionally NOT rendered here — they live in the
 * sibling `<DocHeaderSection>` so the basis section only owns the
 * arrival-type discriminator and its conditional payload.
 *
 * Lock states:
 *   locked=false (incoming/external "+ New acceptance"): all three tabs
 *     visible and switchable; switching clears the previous tab's
 *     specific field and resets `items` (with a confirm dialog when items
 *     are non-empty).
 *   locked=true (row trigger / edit mode): only the active tab is
 *     rendered with `aria-disabled` and a "lock" hint; switching is
 *     blocked at the source.
 *
 * The waybill picker (used in unlocked TRUCK / RAIL tabs) is implemented
 * as a sibling `<WaybillPicker>` subcomponent below. It is filtered to
 * `pipelineStatus === 'PENDING'` to enforce the 1:1 cardinality rule from
 * spec section 2.1 — a waybill that already has an acceptance is not a
 * candidate for issuing another one.
 */

import type { AcceptanceCreate } from './acceptance-form-config'
import type { ArrivalType } from '~/generated/types/ArrivalType'
import type { RailReceiptPipelineResponse } from '~/generated/types/RailReceiptPipelineResponse'
import type { TruckReceiptPipelineResponse } from '~/generated/types/TruckReceiptPipelineResponse'
import { LockIcon } from 'lucide-react'
import { useState } from 'react'
import { useFormContext, useWatch } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '~/components/ui/alert-dialog'
import { EntityPickerInput } from '~/components/entity-picker/entity-picker-input'
import { Input } from '~/components/ui/input'
import { Tabs, TabsList, TabsTrigger } from '~/components/ui/tabs'
import { useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { cn } from '~/lib/utils'

interface AcceptanceBasisSectionProps {
  mode: 'create' | 'edit'
  /** When true, only the active tab is rendered and tab switching is disabled. */
  locked?: boolean
  /** Optional human-readable hint shown next to the lock glyph (e.g. waybill number). */
  lockedHintNumber?: string
}

type BasisTab = Extract<ArrivalType, 'EXTERNAL' | 'TRUCK' | 'RAIL'>

const TAB_VALUES: readonly BasisTab[] = ['EXTERNAL', 'TRUCK', 'RAIL'] as const

export function AcceptanceBasisSection({
  mode: _mode,
  locked = false,
  lockedHintNumber,
}: AcceptanceBasisSectionProps) {
  const { t } = useTranslation(['acceptance'])
  const form = useFormContext<AcceptanceCreate>()
  const arrivalType = useWatch({
    control: form.control,
    name: 'arrivalType',
  }) as BasisTab | undefined
  const activeTab: BasisTab
    = arrivalType === 'TRUCK' || arrivalType === 'RAIL' ? arrivalType : 'EXTERNAL'

  const [pendingTab, setPendingTab] = useState<BasisTab | null>(null)

  function applyTabSwitch(next: BasisTab) {
    // Clear all three basis-discriminator fields unconditionally; the active
    // tab's field will be re-populated by user input. This guards against
    // stale FKs from a partial reset / programmatic mutation surviving the
    // switch and tripping the discriminated-union refine on submit. Common
    // fields (documentNumber, dateAccepted, contractorId) are intentionally
    // preserved across switches.
    form.setValue('sourceEntity', null, { shouldDirty: true })
    form.setValue('truckWaybillId', null, { shouldDirty: true })
    form.setValue('railWaybillId', null, { shouldDirty: true })
    form.setValue('items', [], { shouldDirty: true })
    form.setValue('arrivalType', next as ArrivalType, { shouldDirty: true })
  }

  function handleTabChange(next: string) {
    if (locked)
      return
    const target = next as BasisTab
    if (target === activeTab)
      return
    const itemCount = form.getValues('items')?.length ?? 0
    if (itemCount > 0) {
      setPendingTab(target)
      return
    }
    applyTabSwitch(target)
  }

  function confirmSwitch() {
    if (pendingTab) {
      applyTabSwitch(pendingTab)
      setPendingTab(null)
    }
  }

  function cancelSwitch() {
    setPendingTab(null)
  }

  return (
    <section
      data-slot="acceptance-basis-section"
      className="space-y-4"
    >
      <h3 className="text-sm font-semibold uppercase text-muted-foreground tracking-wide">
        {t('acceptance:section.basis')}
      </h3>

      <Tabs value={activeTab} onValueChange={handleTabChange}>
        <TabsList>
          {TAB_VALUES.map((value) => {
            if (locked && value !== activeTab)
              return null
            const labelKey = `acceptance:basis.tab.${value.toLowerCase()}` as const
            return (
              <TabsTrigger
                key={value}
                value={value}
                aria-disabled={locked || undefined}
                disabled={locked && value !== activeTab}
                className={cn(locked && 'cursor-default opacity-100')}
              >
                {locked && value === activeTab && (
                  <LockIcon className="size-3" data-slot="basis-tab-lock" />
                )}
                {t(labelKey)}
              </TabsTrigger>
            )
          })}
        </TabsList>
      </Tabs>

      {locked && lockedHintNumber && (
        <p className="text-xs text-muted-foreground">
          {t('acceptance:basis.lockedHint', { number: lockedHintNumber })}
        </p>
      )}

      <BasisTabContent locked={locked} activeTab={activeTab} />

      <AlertDialog
        open={pendingTab !== null}
        onOpenChange={open => !open && cancelSwitch()}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>
              {t('acceptance:basis.confirmSwitch.title')}
            </AlertDialogTitle>
            <AlertDialogDescription>
              {t('acceptance:basis.confirmSwitch.message', {
                count: form.getValues('items')?.length ?? 0,
              })}
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel onClick={cancelSwitch}>
              {t('acceptance:basis.confirmSwitch.cancel')}
            </AlertDialogCancel>
            <AlertDialogAction onClick={confirmSwitch}>
              {t('acceptance:basis.confirmSwitch.confirm')}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </section>
  )
}

function BasisTabContent({
  locked,
  activeTab,
}: {
  locked: boolean
  activeTab: BasisTab
}) {
  // Each branch is a separate component so that React unmounts the previous
  // branch on tab switch — keeping the hook count of *this* component stable
  // while the unmount/mount cycle gives the active branch its own clean
  // hook lifecycle.
  if (activeTab === 'EXTERNAL')
    return <SourceEntityField />
  if (activeTab === 'TRUCK') {
    return (
      <WaybillPicker
        kind="truck"
        locked={locked}
        labelKey="acceptance:basis.tab.truck"
      />
    )
  }
  return (
    <WaybillPicker
      kind="rail"
      locked={locked}
      labelKey="acceptance:basis.tab.rail"
    />
  )
}

function SourceEntityField() {
  const { t } = useTranslation(['acceptance'])
  const form = useFormContext<AcceptanceCreate>()
  const sourceEntity = useWatch({
    control: form.control,
    name: 'sourceEntity',
  })
  return (
    <div className="grid gap-2">
      <label className="text-sm font-medium">
        {t('acceptance:field.sourceEntity')}
      </label>
      <Input
        value={sourceEntity ?? ''}
        onChange={(e) => {
          const v = e.target.value
          form.setValue('sourceEntity', v === '' ? null : v, {
            shouldDirty: true,
          })
        }}
        placeholder={t('acceptance:field.sourceEntity')}
      />
    </div>
  )
}

interface WaybillPickerProps {
  kind: 'truck' | 'rail'
  locked: boolean
  labelKey: string
}

/**
 * PENDING-scoped waybill picker.
 *
 * Renders the project-standard `<EntityPickerInput>` (combobox + browse-all
 * dialog) so the picker behaves identically to every other entity selector
 * in the app — typeahead filtering, scrollable result list, browse-all
 * fallback, accessible keyboard handling. Replacing the prior raw
 * Radix `<Select>` fixes the jumpy long-list UX that drops the scrollbar.
 *
 * The flow query (`useFlowTruckReceiptQuery` / `useRailReceiptPipelineQuery`)
 * returns rows keyed by `id` with `basisDocumentNumber` (primary label) and
 * `contractorName` (secondary label). Filtering to
 * `pipelineStatus === 'PENDING'` is applied client-side via the picker's
 * `filter` prop — the dataset is small enough that a server-side filter is
 * not worth the API surface cost today.
 */
function WaybillPicker({ kind, locked, labelKey }: WaybillPickerProps) {
  const { t } = useTranslation(['acceptance'])
  const form = useFormContext<AcceptanceCreate>()
  const fieldName = kind === 'truck' ? 'truckWaybillId' : 'railWaybillId'
  const currentValue = useWatch({
    control: form.control,
    name: fieldName,
  })

  const truckQ = useFlowTruckReceiptQuery(undefined, {
    query: { enabled: kind === 'truck' && !locked },
  })
  const railQ = useRailReceiptPipelineQuery(undefined, {
    query: { enabled: kind === 'rail' && !locked },
  })
  const queryResult = kind === 'truck' ? truckQ : railQ

  // In locked mode the picker is informational only — the value is already set
  // by `useBasisPrefill` and the user cannot change it from this UI.
  if (locked) {
    return (
      <div className="grid gap-2">
        <label className="text-sm font-medium">{t(labelKey)}</label>
        <Input value={currentValue ?? ''} readOnly disabled />
      </div>
    )
  }

  return (
    <div className="grid gap-2">
      <label className="text-sm font-medium">{t(labelKey)}</label>
      <EntityPickerInput
        value={(currentValue as string | null) ?? null}
        onChange={(val) => {
          form.setValue(fieldName, val, {
            shouldDirty: true,
            shouldValidate: true,
          })
        }}
        queryResult={queryResult}
        displayField="basisDocumentNumber"
        secondaryField="contractorName"
        filter={item =>
          (item as TruckReceiptPipelineResponse | RailReceiptPipelineResponse)
            .pipelineStatus === 'PENDING'}
        placeholder={t('acceptance:basis.picker.placeholder')}
        dialogTitle={t(labelKey)}
        nullable
      />
    </div>
  )
}
