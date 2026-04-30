export function isDocEditable(
  doc: { status?: string } | null | undefined,
): boolean {
  if (!doc?.status)
    return false

  return doc.status === 'DRAFT'
}
