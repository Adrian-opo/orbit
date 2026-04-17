# Spec: OpenCode Custom Providers (opencode.jsonc)

## Objective

Read user-defined custom providers from `~/.config/opencode/opencode.jsonc` and merge them
into the list of OpenCode sub-providers shown in Orbit's provider selection UI.

Currently, Orbit reads only the provider cache from `~/.cache/opencode/models.json`. Providers
defined inline by the user in `opencode.jsonc` (e.g. CrofAI, custom OpenAI-compatible endpoints)
are ignored, leaving those models unavailable in Orbit.

---

## Expected Behavior

1. On each call to `get_providers()`, after loading `models.json`, Orbit also reads
   `~/.config/opencode/opencode.jsonc`.
2. Each entry under the `"provider"` key becomes a `SubProvider` with:
   - `id`: the object key (e.g. `"CrofAI"`)
   - `name`: `provider.name` if present, otherwise the key
   - `env`: empty vec — custom providers use inline `options.apiKey`, not env vars
   - `configured`: `true` if `provider.options.apiKey` is a non-empty string
   - `models`: derived from `provider.models` — same shape as `models.json` entries
     (`id`, `name`, `limit.context`, `limit.output`)
3. Providers whose `id` already exists in the `models.json` cache are **not duplicated** —
   `models.json` takes precedence for any ID that appears in both sources.
4. The merged list is sorted alphabetically by `name` before being returned.
5. If `opencode.jsonc` does not exist, cannot be read, or cannot be parsed, the function
   returns silently with no error — the existing `models.json` providers are unaffected.

---

## Edge Cases

| Scenario | Expected result |
|----------|----------------|
| File does not exist | `None` — silent, no error logged |
| File exists but is empty | `None` |
| File is valid JSON (no comments) | Parsed correctly |
| File has `// line` and `/* block */` comments | Comments stripped before JSON parse |
| File has `\"` escape inside a JSON string value | Escape preserved correctly |
| `"provider"` key absent | Returns empty vec — no sub-providers from this source |
| Provider entry has no `"name"` field | Falls back to the key as name |
| Provider entry has no `"options"` | `configured: false` |
| `options.apiKey` is `""` (empty string) | `configured: false` |
| `options.apiKey` is a non-empty string | `configured: true` |
| Provider entry has no `"models"` | `models: []` |
| Same provider ID in both `models.json` and `opencode.jsonc` | `models.json` entry wins; jsonc entry skipped |
| Multiple custom providers defined | All are merged and sorted alphabetically |

---

## Acceptance Criteria

- [ ] A new function `strip_jsonc_comments(s: &str) -> String` correctly removes `//` line
      comments and `/* */` block comments while preserving string contents.
- [ ] A new function `read_opencode_jsonc_providers() -> Option<Vec<SubProvider>>` reads and
      parses `~/.config/opencode/opencode.jsonc`, returning `None` silently on any failure.
- [ ] `get_providers()` merges custom providers after the `models.json` cache, deduplicating by
      `id`, and returns a sorted combined list.
- [ ] No `unwrap()` or `expect()` calls in the new code paths.
- [ ] Passes `cargo clippy -- -D warnings` and `cargo fmt` without issues.
- [ ] If the user has a CrofAI entry in `opencode.jsonc`, its models appear in the Orbit provider
      selector under the OpenCode backend.
