const ROLE_ADMIN = '019c8cc2-8913-774a-a432-4dee8eb3f194'
const ROLE_SENIOR_SUPERVISOR = '019c8cc4-3538-7b66-8ce5-6faad856b217'
const ROLE_SUPERVISOR = '019c8cc4-9048-7b61-9443-52858a953a17'

const SUPERVISOR_OR_HIGHER = new Set([ROLE_SUPERVISOR, ROLE_SENIOR_SUPERVISOR, ROLE_ADMIN])
const SENIOR_OR_HIGHER = new Set([ROLE_SENIOR_SUPERVISOR, ROLE_ADMIN])

export function isSupervisorOrHigher(roleId: string | undefined): boolean {
  return !!roleId && SUPERVISOR_OR_HIGHER.has(roleId)
}

export function isSeniorOrHigher(roleId: string | undefined): boolean {
  return !!roleId && SENIOR_OR_HIGHER.has(roleId)
}
