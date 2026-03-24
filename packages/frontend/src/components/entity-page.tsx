import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'

interface EntityPageProps<TRow> {
  provider: React.ComponentType<{ children: React.ReactNode }>
  title: string
  queryResult: { data?: { data?: TRow[] }, isLoading: boolean }
  primaryButtons: React.ComponentType
  table: React.ComponentType<{ data: TRow[] }>
  dialogs: React.ComponentType
}

export function EntityPage<TRow>({
  provider: Provider,
  title,
  queryResult,
  primaryButtons: PrimaryButtons,
  table: Table,
  dialogs: Dialogs,
}: EntityPageProps<TRow>) {
  const data = queryResult.data?.data ?? []

  return (
    <Provider>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {title}
            </h2>
          </div>
          <PrimaryButtons />
        </div>
        {queryResult.isLoading
          ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="text-muted-foreground">Loading...</div>
              </div>
            )
          : (
              <Table data={data} />
            )}
      </Main>

      <Dialogs />
    </Provider>
  )
}
