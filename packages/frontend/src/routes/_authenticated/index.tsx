import { createFileRoute } from '@tanstack/react-router'
import {
  Building2,
  Container,
  MapPin,
  Package,
  Truck,
  Warehouse,
} from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { Card, CardContent, CardHeader, CardTitle } from '~/components/ui/card'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogPortList } from '~/generated/hooks/CatalogHooks/useCatalogPortList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'

export const Route = createFileRoute('/_authenticated/')({
  component: Dashboard,
})

function Dashboard() {
  const { t } = useTranslation()

  const { data: companiesData } = useCatalogCompanyList()
  const { data: productsData } = useCatalogProductList()
  const { data: warehousesData } = useCatalogWarehouseList()
  const { data: storagesData } = useCatalogStorageList()
  const { data: basesData } = useCatalogBaseList()
  const { data: portsData } = useCatalogPortList()

  const cards = [
    { title: t('nav.companies'), icon: Building2, count: companiesData?.data?.length },
    { title: t('nav.products'), icon: Package, count: productsData?.data?.length },
    { title: t('nav.warehouses'), icon: Warehouse, count: warehousesData?.data?.length },
    { title: t('nav.storages'), icon: Container, count: storagesData?.data?.length },
    { title: t('nav.bases'), icon: MapPin, count: basesData?.data?.length },
    { title: t('nav.ports'), icon: Truck, count: portsData?.data?.length },
  ]

  return (
    <>
      <Header fixed />
      <Main>
        <div className="flex flex-col gap-6">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {t('nav.dashboard')}
            </h2>
            <p className="text-muted-foreground">
              Voletu — Distributed Inventory Management System
            </p>
          </div>
          <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
            {cards.map(card => (
              <Card key={card.title}>
                <CardHeader className="flex flex-row items-center justify-between pb-2">
                  <CardTitle className="text-sm font-medium">
                    {card.title}
                  </CardTitle>
                  <card.icon className="size-4 text-muted-foreground" />
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold">
                    {card.count ?? '—'}
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </Main>
    </>
  )
}
