/**
 * Pre-fill the acceptance form from a transport waybill basis.
 *
 * When `AcceptanceMutateDialog` is opened from a row trigger or detail-page
 * CTA on `incoming/truck` or `incoming/rail`, the dialog needs to seed the
 * form with values derived from the originating waybill: contractor,
 * arrival type, the basis FK itself, and one item row per waybill item /
 * wagon manifest. This hook wraps that pre-fetch + mapping behind a single
 * imperative result that the dialog merges with `emptyAcceptanceCreate`.
 *
 * Risk note (spec §6.1): the produced item rows are *starting points*, not
 * locked rows. The user must remain able to add and delete rows freely;
 * splitting one waybill item across multiple storages is a normal flow.
 * This hook does not enforce any read-only behavior on the items table —
 * that's a UI concern handled in the items-column wiring.
 */

import type { AcceptanceCreate, AcceptanceItem } from './acceptance-form-config'
import { useTransportRailWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet'
import { useTransportTruckWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet'
import { emptyAcceptanceCreate } from './acceptance-form-config'

export interface BasisPrefillRef {
  kind: 'truck' | 'rail'
  basisId: string
}

export interface BasisPrefillResult {
  /** True while the basis composite is being fetched. */
  isLoading: boolean
  /** Pre-filled defaults for the form, or `null` if data not yet loaded. */
  prefilled: AcceptanceCreate | null
  /** The locked tab's display number (waybill / wagon document number). */
  lockedHintNumber: string | null
}

function emptyItemFor(productId: string): AcceptanceItem {
  return {
    productId,
    storageId: '',
    acceptedAmount: '',
  }
}

/**
 * Fetches the basis composite for the given prefill ref and derives the
 * `defaultValues` payload for `AcceptanceMutateDialog`.
 *
 * Both queries are gated on `enabled`: only the matching kind's hook
 * actually fires, the other returns `undefined` data and `isLoading: false`.
 * That keeps the hook count stable across both kinds without conditional
 * hook calls.
 */
export function useBasisPrefill(
  ref: BasisPrefillRef | undefined,
  enabled: boolean,
): BasisPrefillResult {
  const wantTruck = enabled && ref?.kind === 'truck'
  const wantRail = enabled && ref?.kind === 'rail'

  const truckQ = useTransportTruckWaybillCompositeGet(
    wantTruck ? ref!.basisId : '',
    undefined,
    { query: { enabled: wantTruck } },
  )
  const railQ = useTransportRailWaybillCompositeGet(
    wantRail ? ref!.basisId : '',
    undefined,
    { query: { enabled: wantRail } },
  )

  if (!enabled || !ref) {
    return { isLoading: false, prefilled: null, lockedHintNumber: null }
  }

  if (wantTruck) {
    if (truckQ.isLoading || !truckQ.data?.data) {
      return { isLoading: true, prefilled: null, lockedHintNumber: null }
    }
    const composite = truckQ.data.data
    const wb = composite.waybill
    const items = (composite.items ?? []).map(it => emptyItemFor(it.productId))
    const prefilled: AcceptanceCreate = {
      ...emptyAcceptanceCreate,
      arrivalType: 'TRUCK',
      truckWaybillId: wb.id,
      contractorId: wb.senderId,
      items,
    }
    return {
      isLoading: false,
      prefilled,
      lockedHintNumber: wb.documentNumber ?? null,
    }
  }

  if (wantRail) {
    if (railQ.isLoading || !railQ.data?.data) {
      return { isLoading: true, prefilled: null, lockedHintNumber: null }
    }
    const composite = railQ.data.data
    const wb = composite.waybill
    const items = (composite.wagonManifests ?? []).map(m => emptyItemFor(m.productId))
    const prefilled: AcceptanceCreate = {
      ...emptyAcceptanceCreate,
      arrivalType: 'RAIL',
      railWaybillId: wb.id,
      contractorId: wb.senderId,
      items,
    }
    return {
      isLoading: false,
      prefilled,
      lockedHintNumber: wb.documentNumber ?? null,
    }
  }

  return { isLoading: false, prefilled: null, lockedHintNumber: null }
}
