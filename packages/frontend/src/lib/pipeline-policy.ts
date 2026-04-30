import type { DocumentStatus, PipelineStatus } from '~/generated/types'

export function canEditBasis(pipelineStatus: PipelineStatus): boolean {
  return pipelineStatus === 'PENDING' || pipelineStatus === 'DRAFT'
}

export function canIssueAcceptance(pipelineStatus: PipelineStatus): boolean {
  return pipelineStatus === 'PENDING'
}

export function canEditAcceptance(documentStatus: DocumentStatus): boolean {
  return documentStatus === 'DRAFT'
}

export function canExecuteAcceptance(documentStatus: DocumentStatus): boolean {
  return documentStatus === 'DRAFT'
}

export function canRevertAcceptance(documentStatus: DocumentStatus): boolean {
  return documentStatus === 'EXECUTED'
}

export function canDeleteBasis(pipelineStatus: PipelineStatus): boolean {
  return pipelineStatus === 'PENDING'
}

export function canDeleteAcceptance(documentStatus: DocumentStatus): boolean {
  return documentStatus === 'DRAFT'
}
