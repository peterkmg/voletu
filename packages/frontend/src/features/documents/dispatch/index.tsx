import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useDispatchDocumentList } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentList'
import { DispatchDialogs } from './components/dispatch-dialogs'
import { DispatchPrimaryButtons } from './components/dispatch-primary-buttons'
import { DispatchProvider } from './components/dispatch-provider'
import { DispatchTable } from './components/dispatch-table'

export function DispatchDocuments() {
  const { t } = useTranslation(['documents'])

  const { data: listData, isLoading } = useDispatchDocumentList()
  const documents = listData?.data ?? []

  return (
    <DispatchProvider>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('documents:dispatch.title')}
            </h2>
          </div>
          <DispatchPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <DispatchTable data={documents} />
            )}
      </Main>

      <DispatchDialogs />
    </DispatchProvider>
  )
}
