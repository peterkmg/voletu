import { describe, expect, it } from 'vitest'
import { isSeniorOrHigher, isSupervisorOrHigher } from '../lifecycle-actions'

const ADMIN = '019c8cc2-8913-774a-a432-4dee8eb3f194'
const SENIOR = '019c8cc4-3538-7b66-8ce5-6faad856b217'
const SUPERVISOR = '019c8cc4-9048-7b61-9443-52858a953a17'
const OPERATOR = '019c8cc4-d965-7f4a-9f9d-c8d299180c6e'

describe('isSupervisorOrHigher', () => {
  it('returns true for Admin', () => {
    expect(isSupervisorOrHigher(ADMIN)).toBe(true)
  })

  it('returns true for SeniorSupervisor', () => {
    expect(isSupervisorOrHigher(SENIOR)).toBe(true)
  })

  it('returns true for Supervisor', () => {
    expect(isSupervisorOrHigher(SUPERVISOR)).toBe(true)
  })

  it('returns false for Operator', () => {
    expect(isSupervisorOrHigher(OPERATOR)).toBe(false)
  })

  it('returns false for undefined', () => {
    expect(isSupervisorOrHigher(undefined)).toBe(false)
  })
})

describe('isSeniorOrHigher', () => {
  it('returns true for Admin', () => {
    expect(isSeniorOrHigher(ADMIN)).toBe(true)
  })

  it('returns true for SeniorSupervisor', () => {
    expect(isSeniorOrHigher(SENIOR)).toBe(true)
  })

  it('returns false for Supervisor', () => {
    expect(isSeniorOrHigher(SUPERVISOR)).toBe(false)
  })

  it('returns false for Operator', () => {
    expect(isSeniorOrHigher(OPERATOR)).toBe(false)
  })

  it('returns false for undefined', () => {
    expect(isSeniorOrHigher(undefined)).toBe(false)
  })
})
