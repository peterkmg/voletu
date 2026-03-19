import { ledgerEntryList } from '~/generated/client'
import { ledgerEntryListQueryKey } from '~/generated/hooks/LedgerHooks/useLedgerEntryList'
import { queryClient } from '~/shared/api/query-client'

export type { LedgerEntryResponse } from '~/generated/types/LedgerEntryResponse'

export const fetchLedgerEntries = ledgerEntryList

export function invalidateLedgerEntries() {
  return queryClient.invalidateQueries({ queryKey: ledgerEntryListQueryKey() })
}
