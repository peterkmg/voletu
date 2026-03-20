import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { useBlendingDocumentList } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { BlendingDialogs } from './components/blending-dialogs'
import { BlendingPrimaryButtons } from './components/blending-primary-buttons'
import { BlendingProvider } from './components/blending-provider'
import { BlendingTable } from './components/blending-table'

export function BlendingDocuments() {
  const { t } = useTranslation(['documents'])

  const { data: listData, isLoading } = useBlendingDocumentList()
  const documents = listData?.data ?? []

  return (
    <BlendingProvider>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('documents:blending.title')}
            </h2>
          </div>
          <BlendingPrimaryButtons />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <BlendingTable data={documents} />
            )}
      </Main>

      <BlendingDialogs />
    </BlendingProvider>
  )
}
