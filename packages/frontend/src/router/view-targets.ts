export const signInViewTarget = { to: '/sign-in' } as const

export const settingsViewTarget = { to: '/settings', params: {} } as const

export const createDocumentViewTargets = {
  truckReceipt: { to: '/incoming/truck', params: {}, search: { create: true } },
  railReceipt: { to: '/incoming/rail', params: {}, search: { create: true } },
  externalAcceptance: { to: '/incoming/external', params: {}, search: { create: true } },
  truckDispatch: { to: '/outgoing/truck', params: {}, search: { create: true } },
  directDispatch: { to: '/outgoing/direct', params: {}, search: { create: true } },
  bunkering: { to: '/outgoing/bunkering', params: {}, search: { create: true } },
  physicalTransfer: { to: '/internal/physical-transfer', params: {}, search: { create: true } },
  ownershipTransfer: { to: '/internal/ownership-transfer', params: {}, search: { create: true } },
  blending: { to: '/internal/blending', params: {}, search: { create: true } },
  reconciliation: { to: '/internal/reconciliation', params: {}, search: { create: true } },
} as const
