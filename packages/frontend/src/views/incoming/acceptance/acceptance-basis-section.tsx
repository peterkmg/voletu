import type { AcceptanceCreate } from './acceptance-form-config'
import type { ArrivalType } from '~/generated/types/ArrivalType'
import type { RailReceiptPipelineResponse } from '~/generated/types/RailReceiptPipelineResponse'
import type { TruckReceiptPipelineResponse } from '~/generated/types/TruckReceiptPipelineResponse'
import { LockIcon } from 'lucide-react'
import { useState } from 'react'
import { useFormContext, useWatch } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { EntityPickerInput } from '~/components/entity-picker/entity-picker-input'
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
import { Input } from '~/components/ui/input'
import { Tabs, TabsList, TabsTrigger } from '~/components/ui/tabs'
import { useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { cn } from '~/lib/utils'

interface AcceptanceBasisSectionProps {
  mode: 'create' | 'edit'

  locked?: boolean

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
