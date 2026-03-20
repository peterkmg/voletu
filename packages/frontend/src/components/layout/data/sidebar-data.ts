import type { TFunction } from 'i18next'
import type { SidebarData } from '../types'
import {
  Anchor,
  ArrowRightLeft,
  BookOpen,
  Building2,
  ClipboardCheck,
  FileInput,
  FileOutput,
  FlaskConical,
  LayoutDashboard,
  Package,
  RefreshCw,
  Settings,
  TrainFront,
  Truck,
  Users,
  Warehouse,
} from 'lucide-react'

export function getSidebarData(t: TFunction): SidebarData {
  return {
    navGroups: [
      {
        title: t('common:nav.overview'),
        items: [
          { title: t('common:nav.dashboard'), url: '/', icon: LayoutDashboard },
        ],
      },
      {
        title: t('common:nav.catalog'),
        items: [
          { title: t('common:nav.companies'), url: '/catalog/companies', icon: Building2 },
          {
            title: t('common:nav.products'),
            icon: Package,
            items: [
              { title: t('common:nav.productTypes'), url: '/catalog/product-types' },
              { title: t('common:nav.productGroups'), url: '/catalog/product-groups' },
              { title: t('common:nav.products'), url: '/catalog/products' },
            ],
          },
          {
            title: t('common:nav.infrastructure'),
            icon: Warehouse,
            items: [
              { title: t('common:nav.bases'), url: '/catalog/bases' },
              { title: t('common:nav.warehouses'), url: '/catalog/warehouses' },
              { title: t('common:nav.storages'), url: '/catalog/storages' },
            ],
          },
          { title: t('common:nav.ports'), url: '/catalog/ports', icon: Anchor },
        ],
      },
      {
        title: t('common:nav.documents'),
        items: [
          { title: t('common:nav.acceptance'), url: '/documents/acceptance', icon: FileInput },
          { title: t('common:nav.dispatch'), url: '/documents/dispatch', icon: FileOutput },
          { title: t('common:nav.blending'), url: '/documents/blending', icon: FlaskConical },
          {
            title: t('common:nav.transfers'),
            icon: ArrowRightLeft,
            items: [
              { title: t('common:nav.physicalTransfer'), url: '/documents/physical-transfer' },
              { title: t('common:nav.ownershipTransfer'), url: '/documents/ownership-transfer' },
            ],
          },
          { title: t('common:nav.reconciliation'), url: '/documents/inventory-reconciliation', icon: ClipboardCheck },
        ],
      },
      {
        title: t('common:nav.transport'),
        items: [
          { title: t('common:nav.truckWaybills'), url: '/transport/truck-waybills', icon: Truck },
          { title: t('common:nav.railWaybills'), url: '/transport/rail-waybills', icon: TrainFront },
        ],
      },
      {
        title: t('common:nav.system'),
        items: [
          { title: t('common:nav.ledger'), url: '/ledger', icon: BookOpen },
          { title: t('common:nav.users'), url: '/system/users', icon: Users },
          { title: t('common:nav.syncStatus'), url: '/system/sync', icon: RefreshCw },
          { title: t('common:nav.settings'), url: '/settings', icon: Settings },
        ],
      },
    ],
  }
}
