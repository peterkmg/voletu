import { describe, expect, it } from 'vitest'
import { isSeniorOrHigher, isSupervisorOrHigher } from '~/lib/rbac'

const ADMIN = '019c8cc2-8913-774a-a432-4dee8eb3f194'
const SENIOR = '019c8cc4-3538-7b66-8ce5-6faad856b217'
const SUPERVISOR = '019c8cc4-9048-7b61-9443-52858a953a17'
const OPERATOR = '019c8cc4-d965-7f4a-9f9d-c8d299180c6e'

describe('isSupervisorOrHigher', () => {
  it('allows Admin users to perform supervisor-level actions', () => {
    expect(isSupervisorOrHigher(ADMIN)).toBe(true)
  })

  it('allows SeniorSupervisor users to perform supervisor-level actions', () => {
    expect(isSupervisorOrHigher(SENIOR)).toBe(true)
  })

  it('allows Supervisor users to perform supervisor-level actions', () => {
    expect(isSupervisorOrHigher(SUPERVISOR)).toBe(true)
  })

  it('denies Operator users from supervisor-level actions', () => {
    expect(isSupervisorOrHigher(OPERATOR)).toBe(false)
  })

  it('denies missing roles from supervisor-level actions', () => {
    expect(isSupervisorOrHigher(undefined)).toBe(false)
  })
})

describe('isSeniorOrHigher', () => {
  it('allows Admin users to perform senior-level actions', () => {
    expect(isSeniorOrHigher(ADMIN)).toBe(true)
  })

  it('allows SeniorSupervisor users to perform senior-level actions', () => {
    expect(isSeniorOrHigher(SENIOR)).toBe(true)
  })

  it('denies Supervisor users from senior-level actions', () => {
    expect(isSeniorOrHigher(SUPERVISOR)).toBe(false)
  })

  it('denies Operator users from senior-level actions', () => {
    expect(isSeniorOrHigher(OPERATOR)).toBe(false)
  })

  it('denies missing roles from senior-level actions', () => {
    expect(isSeniorOrHigher(undefined)).toBe(false)
  })
})
