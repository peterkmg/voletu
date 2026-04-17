/**
 * Returns whether a document can be edited based on its lifecycle state.
 *
 * Editable states (per spec §4.4): documents in `draft` are editable; executed
 * documents are immutable. The backend `DocumentStatus` enum currently defines
 * only two variants -- `Draft` and `Executed` (see
 * `packages/core/src/enums/mod.rs`). Variants are serialized via serde with
 * `rename_all = "SCREAMING_SNAKE_CASE"`, so the wire values are `"DRAFT"` and
 * `"EXECUTED"`.
 *
 * Unknown / missing status returns `false` (safe default — not editable). If
 * a `reverted_to_draft` (or any other editable) lifecycle state is added later,
 * extend the comparison below. The single source of truth for the enum remains
 * `packages/core/src/enums/mod.rs`.
 *
 * Pure function (no React hooks) — named without the `use` prefix so the
 * `react/no-unnecessary-use-prefix` lint stays clean.
 */
export function isDocEditable(
  doc: { status?: string } | null | undefined,
): boolean {
  if (!doc?.status)
    return false
  return doc.status === 'DRAFT'
}
