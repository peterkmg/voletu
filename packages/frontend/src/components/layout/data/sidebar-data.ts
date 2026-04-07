import type { TFunction } from 'i18next'
import type { SidebarData } from '../types'
import {
  Anchor,
  ArrowDownToLine,
  ArrowLeftRight,
  ArrowUpFromLine,
  BarChart3,
  Building2,
  FileText,
  LayoutDashboard,
  Package,
  RefreshCw,
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
        title: t('common:nav.operations'),
        items: [
          { title: t('common:nav.cargoFlow'), url: '/cargo-flow', icon: BarChart3 },
          {
            title: t('common:nav.incoming'),
            icon: ArrowDownToLine,
            items: [
              { title: t('common:nav.truckReceipt'), url: '/incoming/truck' },
              { title: t('common:nav.railReceipt'), url: '/incoming/rail' },
              { title: t('common:nav.externalAcceptance'), url: '/incoming/external' },
            ],
          },
          {
            title: t('common:nav.outgoing'),
            icon: ArrowUpFromLine,
            items: [
              { title: t('common:nav.truckDispatch'), url: '/outgoing/truck' },
              { title: t('common:nav.directDispatch'), url: '/outgoing/direct' },
              { title: t('common:nav.bunkering'), url: '/outgoing/bunkering' },
            ],
          },
          {
            title: t('common:nav.internal'),
            icon: ArrowLeftRight,
            items: [
              { title: t('common:nav.physicalTransfer'), url: '/internal/physical-transfer' },
              { title: t('common:nav.ownershipTransfer'), url: '/internal/ownership-transfer' },
              { title: t('common:nav.blending'), url: '/internal/blending' },
              { title: t('common:nav.reconciliation'), url: '/internal/reconciliation' },
            ],
          },
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
          { title: t('common:nav.auditLogs'), url: '/system/audit-logs', icon: FileText, roles: ['ADMIN'] },
        ],
      },
    ],
  }
}
