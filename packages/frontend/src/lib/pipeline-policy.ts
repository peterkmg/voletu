// Pure lifecycle gating predicates for the acceptance pipeline.
//
// These intentionally generalize the `isDocEditable` hook in `~/hooks/use-doc-editable`
// to add pipeline awareness for basis (transport waybill) documents. The two stay
// consistent: `isDocEditable` remains the doc-status-only check used by simpler
// (non-pipeline) detail pages, while these predicates are consumed by pipeline-aware
// row actions and detail-page toolbars.

import type { DocumentStatus, PipelineStatus } from '~/generated/types'

/** True when the basis (transport waybill) is editable. */
export function canEditBasis(pipelineStatus: PipelineStatus): boolean {
  return pipelineStatus === 'PENDING' || pipelineStatus === 'DRAFT'
}

/** True when an acceptance can be issued from a basis row. */
export function canIssueAcceptance(pipelineStatus: PipelineStatus): boolean {
  return pipelineStatus === 'PENDING'
}

/** True when an acceptance document is editable. */
export function canEditAcceptance(documentStatus: DocumentStatus): boolean {
  return documentStatus === 'DRAFT'
}

/** True when an acceptance can be executed (posted). */
export function canExecuteAcceptance(documentStatus: DocumentStatus): boolean {
  return documentStatus === 'DRAFT'
}

/** True when an executed acceptance can be reverted to draft. */
export function canRevertAcceptance(documentStatus: DocumentStatus): boolean {
  return documentStatus === 'EXECUTED'
}

/** True when a basis (transport waybill) can be hard-deleted. */
export function canDeleteBasis(pipelineStatus: PipelineStatus): boolean {
  return pipelineStatus === 'PENDING'
}

/** True when an acceptance document can be hard-deleted. */
export function canDeleteAcceptance(documentStatus: DocumentStatus): boolean {
  return documentStatus === 'DRAFT'
}
