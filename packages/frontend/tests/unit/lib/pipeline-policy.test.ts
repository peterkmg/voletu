import { describe, expect, it } from 'vitest'
import {
  canDeleteAcceptance,
  canDeleteBasis,
  canEditAcceptance,
  canEditBasis,
  canExecuteAcceptance,
  canIssueAcceptance,
  canRevertAcceptance,
} from '~/lib/pipeline-policy'

describe('pipeline-policy', () => {
  describe('canEditBasis', () => {
    it.each([
      ['PENDING', true],
      ['DRAFT', true],
      ['EXECUTED', false],
    ] as const)('returns %s for pipelineStatus=%s', (status, expected) => {
      expect(canEditBasis(status)).toBe(expected)
    })
  })

  describe('canIssueAcceptance', () => {
    it.each([
      ['PENDING', true],
      ['DRAFT', false],
      ['EXECUTED', false],
    ] as const)('returns %s for pipelineStatus=%s', (status, expected) => {
      expect(canIssueAcceptance(status)).toBe(expected)
    })
  })

  describe('canEditAcceptance', () => {
    it.each([
      ['DRAFT', true],
      ['EXECUTED', false],
    ] as const)('returns %s for status=%s', (status, expected) => {
      expect(canEditAcceptance(status)).toBe(expected)
    })
  })

  describe('canExecuteAcceptance', () => {
    it.each([
      ['DRAFT', true],
      ['EXECUTED', false],
    ] as const)('returns %s for status=%s', (status, expected) => {
      expect(canExecuteAcceptance(status)).toBe(expected)
    })
  })

  describe('canRevertAcceptance', () => {
    it.each([
      ['DRAFT', false],
      ['EXECUTED', true],
    ] as const)('returns %s for status=%s', (status, expected) => {
      expect(canRevertAcceptance(status)).toBe(expected)
    })
  })

  describe('canDeleteBasis', () => {
    it.each([
      ['PENDING', true],
      ['DRAFT', false],
      ['EXECUTED', false],
    ] as const)('returns %s for pipelineStatus=%s', (status, expected) => {
      expect(canDeleteBasis(status)).toBe(expected)
    })
  })

  describe('canDeleteAcceptance', () => {
    it.each([
      ['DRAFT', true],
      ['EXECUTED', false],
    ] as const)('returns %s for status=%s', (status, expected) => {
      expect(canDeleteAcceptance(status)).toBe(expected)
    })
  })
})
