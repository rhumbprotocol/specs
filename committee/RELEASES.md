# RWP Release Governance

This document describes how new versions of the Rhumb Workflow Protocol are released.

---

## Versioning

RWP follows [Semantic Versioning 2.0.0](https://semver.org/):

- **Major** (X.0.0): Breaking changes to artifact schemas or lifecycle semantics
- **Minor** (1.X.0): New features, new artifact types, additive schema changes
- **Patch** (1.0.X): Bug fixes, clarifications, typo corrections

The current version is embedded in artifacts via the `rwp_version` field. See the [Protocol Specification](../docs/PROTOCOL.md) for version detection and compatibility rules.

---

## Release Authority

| Version Type | Who Approves | Process |
|-------------|--------------|---------|
| Patch (1.0.x) | Any maintainer | Merge PR, tag release |
| Minor (1.x.0) | 2 maintainers | Formal review, CHANGELOG update |
| Major (x.0.0) | 2/3 committee vote | AEP required, 30-day discussion |

---

## Release Process

### Patch Releases

Patch releases fix errors without changing semantics:

1. Fix is submitted as a pull request
2. At least 1 maintainer approves
3. CHANGELOG.md is updated with the fix description
4. Maintainer merges and tags the release (e.g., `v1.0.1`)

### Minor Releases

Minor releases add new capabilities:

1. Changes are submitted and reviewed through the normal process
2. All changes for the release are collected on a release branch
3. CHANGELOG.md is updated with all new features and changes
4. At least 2 maintainers approve the release
5. Release is tagged (e.g., `v1.1.0`)

### Major Releases

Major releases may include breaking changes:

1. An AEP is written describing the breaking changes and migration path
2. The AEP goes through the full proposal process (30-day minimum discussion)
3. A two-thirds committee vote approves the major version
4. A migration guide is published alongside the release
5. The previous major version continues to receive patch fixes for 12 months
6. Release is tagged (e.g., `v2.0.0`)

---

## Backward Compatibility

RWP takes backward compatibility seriously:

- **Patch and minor releases** are always backward compatible
- **New fields** are added as optional (existing documents remain valid)
- **Deprecated features** are marked in one release and removed no earlier than the next major release
- **Migration guides** are provided for all breaking changes

### Deprecation Process

1. Feature is marked as deprecated in a minor release
2. Deprecation notice is added to CHANGELOG.md and the relevant specification section
3. The deprecated feature continues to work for at least one full minor release cycle
4. Removal happens in a subsequent major release with a migration guide

---

## Release Checklist

Before tagging any release, verify:

- [ ] All JSON schemas are valid and consistent
- [ ] CHANGELOG.md is updated with the release notes
- [ ] Version numbers are updated in relevant files
- [ ] All cross-references in PROTOCOL.md are correct
- [ ] Templates reference the correct `rwp_version`
- [ ] README.md reflects any new features or changes
- [ ] No enforcement language in advisory documents

---

## Hotfix Process

For critical issues (security vulnerabilities, data loss risks):

1. A maintainer may fast-track a patch release
2. The fix is reviewed by at least 1 other maintainer
3. Normal review timelines are shortened to 24-48 hours
4. The release is tagged and announced promptly

---

Rhumb Workflow Protocol (RWP) v0.28.1
https://rhumbprotocol.dev
