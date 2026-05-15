# RWP Branching Model

This document describes the git branching strategy for the Rhumb Workflow
Protocol specification repository. It complements
[RELEASES.md](../committee/RELEASES.md) (which defines release authority and
versioning) and [CONTRIBUTING.md](../CONTRIBUTING.md) (which defines how to
participate).

---

## Long-Lived Branches

| Branch    | Purpose                                                                 | Direct commits |
|-----------|-------------------------------------------------------------------------|----------------|
| `main`    | Canonical released spec. Each commit on `main` corresponds to a tagged release. | Maintainers only, via merge from `develop`. No direct edits, ever. |
| `develop` | Integration branch. All approved changes accumulate here until the next release cut. | Maintainers only, via merge from short-lived branches. No direct edits. |

`main` is the branch consumers trust. Implementors pinning to `rhumbprotocol/specs#main`
get the latest tagged release with no surprises. `develop` is what the working
group is actively shaping toward the next release.

---

## Short-Lived Branches

All work happens on short-lived branches that target `develop`:

| Prefix       | Use for                                              | Lifetime |
|--------------|------------------------------------------------------|----------|
| `feature/*`  | New artifacts, schema additions, capability changes  | Until merged into `develop`, then deleted |
| `fix/*`      | Bug fixes, typo corrections, schema validation fixes | Until merged into `develop`, then deleted |
| `docs/*`     | Documentation-only changes                           | Until merged into `develop`, then deleted |
| `wip/*`      | Exploratory work, not yet PR-ready                   | May be force-pushed, may be abandoned |
| `release/*`  | Release stabilization (maintainers only)             | Until tagged and merged into `main` + `develop` |
| `hotfix/*`   | Critical fixes against `main` (maintainers only)     | Until tagged and merged into both `main` and `develop` |

**Branch naming rule**: `<prefix>/<short-kebab-case-description>` - e.g.,
`feature/add-aep-artifact`, `fix/intake-schema-required-fields`,
`docs/clarify-handoff-section`.

---

## Standard Contribution Flow

```
   contributor                    maintainer                    repo
   ───────────                    ──────────                    ────
   fork repo
   git checkout develop
   git pull
   git checkout -b feature/x
   ...edit, commit...
   git push origin feature/x
   open PR  ──────────────►       review
                                  request changes ──┐
   ...address feedback...                           │
   git push                       re-review  ◄──────┘
                                  approve
                                  merge into develop  ─────►   develop updated
                                                               feature/x deleted
```

Steps in detail:

1. Fork `rhumbprotocol/specs` (external contributors) or create a branch
   directly (maintainers).
2. Branch from the latest `develop`: `git checkout develop && git pull && git
   checkout -b feature/<descriptive-name>`.
3. Make your changes following [CONTRIBUTING.md](../CONTRIBUTING.md) style
   guidelines.
4. Run the validator locally before pushing (see
   [Validation](#validation-rhumbproto-utility) below).
5. Push and open a pull request against `develop` (never against `main`).
6. Address review feedback. PRs require approval per
   [RELEASES.md](../committee/RELEASES.md) review criteria.
7. On approval, a maintainer merges the PR into `develop`. The contributor
   branch is deleted automatically.

---

## Release Cuts (`develop` → `main`)

When `develop` is in shape for the next release, a maintainer cuts a release:

1. Open a `release/v<version>` branch from `develop`.
2. On the release branch:
   - Bump `rwp_version` in all artifacts that carry it.
   - Update `CHANGELOG.md` with the release notes.
   - Run the full validator suite.
   - Resolve any last-minute issues (no new features - fixes only).
3. Open a PR from `release/v<version>` into `main`.
4. After approval (per [RELEASES.md](../committee/RELEASES.md) authority
   matrix), merge into `main`.
5. Tag the merge commit on `main`: `git tag -s v<version> -m "RWP v<version>"`.
6. Push the tag: `git push origin v<version>`.
7. Merge `main` back into `develop` so `develop` carries the version bump and
   any release-branch fixes.
8. Delete the `release/*` branch.
9. Publish the release notice (GitHub Release, mailing list, site updates).

A release tag on `main` is the canonical reference for any
`rwp_version: X.Y.Z` artifact in the wild.

---

## Hotfix Flow (`main` → both `main` and `develop`)

For critical issues that cannot wait for the next normal release:

1. Branch `hotfix/v<patch-version>` from `main` (not from `develop`).
2. Apply the minimal fix.
3. Bump the patch version in `CHANGELOG.md` and any version-bearing artifacts.
4. Open a PR into `main`.
5. After expedited review (per RELEASES.md hotfix process), merge into `main`
   and tag.
6. Open a second PR from the same branch into `develop` to carry the fix
   forward, OR merge `main` into `develop` if `develop` has not diverged
   significantly from the patched area.
7. Delete the `hotfix/*` branch.

---

## Branch Protection Rules

The following protections are enforced by GitHub on `rhumbprotocol/specs`:

### `main`
- Direct pushes blocked. Merges via PR only.
- Required reviewers: per [RELEASES.md](../committee/RELEASES.md) authority
  matrix (1 maintainer for patch, 2 for minor, 2/3 committee for major).
- Required status checks: validator suite, schema validation, CI build.
- Linear history required (rebase or squash merges; no merge commits except
  release-branch and hotfix merges, which preserve the merge commit for
  traceability).
- Force pushes blocked.
- Branch deletion blocked.

### `develop`
- Direct pushes blocked. Merges via PR only.
- Required reviewers: 1 maintainer minimum.
- Required status checks: validator suite, schema validation, CI build.
- Force pushes blocked.
- Branch deletion blocked.

### Short-lived branches (`feature/*`, `fix/*`, `docs/*`, `wip/*`)
- No protections. Authors may force-push their own branches.
- Auto-deleted on PR merge.

---

## Validation: `rhumbproto` Utility

Every PR runs an automated validator suite before merge eligibility. The
validator is shipped from this repository as a public utility so contributors
can run it locally and downstream implementations can validate their own
artifacts.

**Location** (current sketch - subject to refinement before v1.0):

```
util/
  cli/                   # CLI entry point (rhumbproto)
  validators/
    schema-meta/         # Validates JSON Schemas in spec/schemas/* against
                         #   the schema metaschema.
    template-conformance/  # Validates each template against its corresponding
                         #   schema.
    artifact/            # Validates user PLAN.md / INTAKE.yaml / state.yaml /
                         #   handoff artifacts against the spec.
    conformance-level/   # Runs conformance-levels.md checks per level.
```

**Scope of validation**:

1. JSON Schemas in `spec/schemas/*` are valid against the JSON Schema
   metaschema.
2. Each template file (`templates/*.template`) parses as the expected format
   and conforms to its schema.
3. User-provided artifact files (`PLAN.md`, `INTAKE.yaml`, `state.yaml`,
   `manifest.yaml`, `HO-*.yaml`) parse and validate against the published
   schemas.
4. Conformance levels (per `spec/conformance-levels.md`) hold for declared
   artifacts.

**Distribution** (planned):

- Run locally: `pnpm exec rhumbproto validate <path>`
- Distribute as `@rhumbprotocol/validator` on npm
- Optional Rust port if implementations need a non-Node runtime

**Status**: design only. The CLI surface and command names are not yet
finalized. See [util/README.md](../util/README.md) for current scaffolding.

---

## Why This Model

RWP is a **versioned specification** with multiple downstream implementors,
not a continuously-deployed application. That makes the constraints:

- Implementors need a stable reference, so `main` is read-only canonical.
- The working group needs an integration surface, so `develop` is where
  work accumulates between release cuts.
- Contributors need clear branching conventions, so short-lived prefixed
  branches auto-delete on merge.
- Releases need formal authority gates, so release branches separate
  stabilization from the normal flow.

Trunk-based development (single `main`, every commit a release) works well
for SaaS but fits poorly for a spec where every release is a publication
event with downstream impact. Classic git-flow, adapted for spec work,
matches what RWP needs.

---

## See Also

- [CONTRIBUTING.md](../CONTRIBUTING.md) - How to contribute
- [committee/RELEASES.md](../committee/RELEASES.md) - Release authority and
  versioning rules
- [committee/GOVERNANCE.md](../committee/GOVERNANCE.md) - Committee structure
- [committee/CHARTER.md](../committee/CHARTER.md) - Project charter

---

Rhumb Workflow Protocol (RWP)
https://rhumbprotocol.dev
