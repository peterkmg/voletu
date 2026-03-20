import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useAcceptanceDocumentList } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentList'
import { AcceptanceDialogs } from './components/acceptance-dialogs'
import { AcceptancePrimaryButtons } from './components/acceptance-primary-buttons'
import { AcceptanceProvider } from './components/acceptance-provider'
import { AcceptanceTable } from './components/acceptance-table'

export function AcceptanceDocuments() {
  const { t } = useTranslation(['documents'])

  const { data: listData, isLoading } = useAcceptanceDocumentList()
  const documents = listData?.data ?? []

  return (
    <AcceptanceProvider>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('documents:acceptance.title')}
            </h2>
          </div>
          <AcceptancePrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <AcceptanceTable data={documents} />
            )}
      </Main>

      <AcceptanceDialogs />
    </AcceptanceProvider>
  )
}
