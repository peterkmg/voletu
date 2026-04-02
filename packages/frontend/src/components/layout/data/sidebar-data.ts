import type { TFunction } from 'i18next'
import type { SidebarData } from '../types'
import {
  Anchor,
  ArrowDownToLine,
  ArrowLeftRight,
  BarChart3,
  Building2,
  ClipboardCheck,
  FlaskConical,
  Fuel,
  LayoutDashboard,
  Package,
  RefreshCw,
  Ship,
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
          { title: t('common:nav.cargoFlow'), url: '/cargo-flow', icon: BarChart3 },
        ],
      },
      {
        title: t('common:nav.incoming'),
        items: [
          { title: t('common:nav.truckReceipt'), url: '/incoming/truck', icon: Truck },
          { title: t('common:nav.railReceipt'), url: '/incoming/rail', icon: TrainFront },
          { title: t('common:nav.externalAcceptance'), url: '/incoming/external', icon: ArrowDownToLine },
        ],
      },
      {
        title: t('common:nav.outgoing'),
        items: [
          { title: t('common:nav.truckDispatch'), url: '/outgoing/truck', icon: Truck },
          { title: t('common:nav.directDispatch'), url: '/outgoing/direct', icon: Ship },
          { title: t('common:nav.bunkering'), url: '/outgoing/bunkering', icon: Fuel },
        ],
      },
      {
        title: t('common:nav.internal'),
        items: [
          { title: t('common:nav.physicalTransfer'), url: '/internal/physical-transfer', icon: ArrowLeftRight },
          { title: t('common:nav.ownershipTransfer'), url: '/internal/ownership-transfer', icon: ArrowLeftRight },
          { title: t('common:nav.blending'), url: '/internal/blending', icon: FlaskConical },
          { title: t('common:nav.reconciliation'), url: '/internal/reconciliation', icon: ClipboardCheck },
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
        title: t('common:nav.system'),
        items: [
          { title: t('common:nav.users'), url: '/system/users', icon: Users, roles: ['ADMIN'] },
          { title: t('common:nav.syncStatus'), url: '/system/sync', icon: RefreshCw, roles: ['ADMIN'] },
        ],
      },
    ],
  }
}
