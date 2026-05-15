# RWP Version Embedding Format Specification

> Part of the [Rhumb Workflow Protocol (RWP)](../docs/PROTOCOL.md) - see also: [Protocol Versioning](../docs/PROTOCOL.md#protocol-versioning)

## Overview

The Rhumb Workflow Protocol (RWP) supports version declaration and embedding in artifacts to enable compatibility checking, schema validation, and migration support across implementations and versions.

## Version Format

RWP uses **semantic versioning** (SemVer 2.0.0) for all version identifiers, with one important nuance described in [Pre-1.0 Stability](#pre-10-stability).

### Semantic Versioning Syntax

```
MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]
```

Examples:

- `0.26.0` - Current RWP version (pre-1.0)
- `0.27.0` - A future minor release; may include breaking changes (see [Pre-1.0 Stability](#pre-10-stability))
- `1.0.0` - The first stable release (target; not yet shipped)
- `1.1.0` - A future minor update under stable semantics (backward compatible)
- `2.0.0` - A future major update (breaking changes)
- `1.0.1+build-20260304` - Build metadata illustration

### Components

| Component | Purpose | Rules |
|-----------|---------|-------|
| **MAJOR** | Compatibility boundary | Increment when incompatible changes ship. While MAJOR is `0`, every MINOR bump is allowed to break compatibility. |
| **MINOR** | Backward-compatible features (post-1.0) | Increment when adding features. Pre-1.0, MINOR may also include breaking changes. |
| **PATCH** | Bug fixes | Increment for backward-compatible fixes. Always safe within a MINOR series. |
| **PRERELEASE** | Pre-release identifier | `-alpha`, `-beta`, `-rc.1` |
| **BUILD** | Build metadata | `+build-20260304`, `+sha.123abc` |

### Current Version

```
0.26.0
```

**Status**: Pre-1.0, advisory. The protocol is shaping toward a stable v1.0. Until v1.0 ships, MINOR releases (e.g., `0.26.0` → `0.27.0`) are permitted to introduce breaking changes; PATCH releases (`0.26.0` → `0.26.1`) remain backward compatible within a MINOR series.

## Pre-1.0 Stability

While `MAJOR == 0`, RWP follows the SemVer 2.0.0 §4 convention: anything may change between MINOR releases. The protocol uses this latitude deliberately during the pre-1.0 phase to incorporate feedback from implementors.

The compatibility commitment during 0.x is therefore narrower than post-1.0:

| Bump type | 0.x behavior | Post-1.0 behavior |
|-----------|--------------|-------------------|
| PATCH (`0.26.0` → `0.26.1`) | Backward compatible. Safe to upgrade. | Backward compatible. Safe to upgrade. |
| MINOR (`0.26.0` → `0.27.0`) | **May break**. Treat as a major bump for compatibility purposes. | Backward compatible. Safe to upgrade. |
| MAJOR (`0.x` → `1.0`) | Stability boundary. Migration guide required. | Breaking changes. Migration guide required. |

v1.0 is the boundary at which RWP commits to standard SemVer compatibility rules going forward.

## Version Declaration in Artifacts

### In PLAN.md (YAML Frontmatter)

```yaml
---
title: "Build Analytics Platform"
rwp_version: "0.26.0"
created: "2026-04-27T08:00:00Z"
---
```

### In INTAKE.yaml

```yaml
id: INT-0001
title: "Build Real-Time Analytics Platform"
rwp_version: "0.26.0"
captured: "2026-04-27T09:00:00Z"
```

### In manifest.yaml

```yaml
plan_id: MP-0001-analytics-platform
manifest_version: "1.0"  # Schema version (independent of RWP version)
rwp_version: "0.26.0"    # Protocol version
created: "2026-04-27T08:00:00Z"
```

### In state.yaml

```yaml
plan_id: MP-0001-analytics-platform
state_version: "1.0"     # Schema version
rwp_version: "0.26.0"    # Protocol version
created: "2026-04-27T08:00:00Z"
```

### In Handoff (HO-*.yaml)

Handoff documents don't require explicit version declaration in most cases, but may reference it:

```markdown
# Handoff: P-01 → P-02

**RWP Version**: 0.26.0

## Summary
...
```

## Version Validation

Implementations should validate that artifact versions are compatible before processing.

### Compatibility Matrix

The matrix below applies to RWP's pre-1.0 phase. Once v1.0 ships, the compatibility rules expand to standard post-1.0 SemVer.

| Artifact Version | Implementation Supports | Result |
|------------------|-------------------------|--------|
| `0.26.0` | `0.26.0` | ✓ Full support |
| `0.26.1` | `0.26.0` | ✓ Full support (patch within same minor) |
| `0.27.0` | `0.26.0` | ✗ May fail - pre-1.0 minor bumps may break |
| `0.26.0` | `1.0.0` | Migration required - see release notes |
| `0.26.1` | `0.26.0` | ✗ Implementation predates artifact's patch only if a tool requires exact patch; RWP does not |
| `1.0.0` | `0.26.0` | ✗ Major version mismatch |

### Validation Strategy

1. **Extract version**: Read the `rwp_version` field from the artifact.
2. **Compare MAJOR**: If artifact MAJOR > implementation MAJOR, reject.
3. **Pre-1.0 special case**: If both MAJOR are `0`, also reject when artifact MINOR != implementation MINOR (since 0.x minor bumps may break). PATCH differences within the same MINOR remain safe.
4. **Post-1.0 case**: If artifact MAJOR == implementation MAJOR > `0`, warn on MINOR > implementation MINOR but allow.
5. **Log compatibility**: Record version comparisons for the audit trail.

### Implementation Examples

**Python**:

```python
from packaging import version

def is_compatible(artifact_version: str, impl_version: str) -> bool:
    """Check if an artifact version is compatible with an implementation version."""
    av = version.parse(artifact_version)
    iv = version.parse(impl_version)

    if av.major != iv.major:
        return False

    # Pre-1.0: minor bumps may break, so require exact minor match
    if av.major == 0:
        return av.minor == iv.minor

    # Post-1.0: implementation minor must be >= artifact minor
    return iv.minor >= av.minor

# Usage
assert is_compatible("0.26.0", "0.26.0")           # Same minor in 0.x
assert is_compatible("0.26.1", "0.26.0")           # Patch bump within 0.26.x
assert not is_compatible("0.27.0", "0.26.0")       # 0.x minor bumps may break
assert not is_compatible("0.26.0", "1.0.0")        # Major version mismatch
```

**TypeScript**:

```typescript
function isCompatible(artifactVersion: string, implVersion: string): boolean {
  const parse = (v: string) => {
    const m = v.match(/^(\d+)\.(\d+)\.(\d+)/);
    if (!m) throw new Error(`Invalid version: ${v}`);
    return { major: +m[1], minor: +m[2], patch: +m[3] };
  };

  const av = parse(artifactVersion);
  const iv = parse(implVersion);

  if (av.major !== iv.major) return false;

  // Pre-1.0: minor bumps may break, so require exact minor match
  if (av.major === 0) return av.minor === iv.minor;

  // Post-1.0: implementation minor must be >= artifact minor
  return iv.minor >= av.minor;
}

// Usage
console.assert(isCompatible("0.26.0", "0.26.0"));        // Same minor in 0.x
console.assert(isCompatible("0.26.1", "0.26.0"));        // Patch bump within 0.26.x
console.assert(!isCompatible("0.27.0", "0.26.0"));       // 0.x minor bumps may break
console.assert(!isCompatible("0.26.0", "1.0.0"));        // Major version mismatch
```

## Version History & Roadmap

### Released Versions

| Version | Status | Notes |
|---------|--------|-------|
| **0.26.0** | Current | Pre-1.0; advisory; subject to MINOR-bump breakage during 0.x maturation. |

### Planned Versions

| Version | Status | Goals |
|---------|--------|-------|
| **0.26.x** and beyond | Planned | Continued maturation of artifact schemas, conformance suite, and integration adapters based on implementor feedback. |
| **1.0.0** | Target | Stability boundary. Ships once at least one external implementation passes the conformance suite and the charter committee ratifies the release. |
| **1.1.0** and beyond | Future | Backward-compatible additions under standard SemVer rules: optional fields, advisory features, conformance-level extensions. |
| **2.0.0** | Future | Reserved for the next breaking-change cycle, if any becomes necessary. |

## Migration Guide

### Within 0.x: Upgrading Between MINOR Releases

While the protocol is pre-1.0, MINOR bumps may include breaking changes. Each MINOR release will document its breaking changes in [CHANGELOG.md](../CHANGELOG.md) and provide a migration note in the release entry.

Within a single MINOR series (e.g., `0.26.0` → `0.26.1`), upgrades are backward compatible. PATCH bumps fix bugs and clarify behavior; they do not change schemas or lifecycle semantics.

### Within 0.x: Upgrading Between PATCH Releases

PATCH releases are always safe to adopt. Update the `rwp_version` field in artifacts during routine maintenance.

### From 0.x to 1.0.0

When v1.0 ships, a dedicated migration guide will document any required changes. Most consumers should expect at least a `rwp_version` bump and a review of any `rwp_version_support` declarations in their tooling.

## Version Declaration Best Practices

### Recommended

```yaml
# Always declare RWP version in top-level artifacts.
plan_id: MP-0001-analytics-platform
rwp_version: "0.26.0"
created: "2026-04-27T08:00:00Z"
```

```yaml
# Schema versions are declared independently of the protocol version.
manifest_version: "1.0"        # Manifest schema version
rwp_version: "0.26.0"          # Protocol version
```

### Not Recommended

```yaml
# Avoid non-standard version formats.
rwp_version: "v0"              # Should be "0.26.0"
rwp_version: "0"               # Should be "0.26.0"
rwp_version: "0.25"            # Should be "0.26.0"
```

```yaml
# Avoid omitting version declarations.
plan_id: P-0001
created: "2026-04-27T08:00:00Z"
# rwp_version field missing - implementations may default but this is
# fragile during 0.x and should be made explicit.
```

## Conformance Levels

RWP defines three conformance levels. The level reflects how completely an implementation supports the protocol; the version reflects which protocol revision the implementation targets.

### Level 1: Minimal Conformance

- Supports the core artifact triplet (Plan, Intake, State) at the current RWP version.
- Implements basic phase lifecycle tracking.
- Validates the MAJOR version of incoming artifacts.
- Identifies itself as "RWP 0.25 Minimal Conformant" (or the current MINOR series).

### Level 2: Standard Conformance (Recommended)

- Supports all artifact types at the current RWP version (including Manifest and Handoff).
- Implements the full phase lifecycle with error recovery.
- Validates MAJOR + MINOR (per [Pre-1.0 Stability](#pre-10-stability) rules during 0.x).
- Identifies itself as "RWP 0.25 Standard Conformant".

### Level 3: Advanced Conformance

- Supports all current-version features plus the extension mechanism.
- Validates custom fields against published schema extensions.
- Plans for forward compatibility across the upcoming MINOR series.
- Identifies itself as "RWP 0.25 Advanced Conformant".

### Declaring Conformance

Implementations should declare their conformance level and supported versions:

```yaml
# In tool configuration or metadata
rwp_conformance: "standard"    # minimal | standard | advanced
rwp_version_support:           # Versions the tool fully supports
  - "0.26.0"
```

## Schema Versioning

Each artifact type has its own **schema version**, declared independently of the RWP protocol version.

### Version Independence

```yaml
# Example: manifest.yaml
manifest_version: "1.0"        # Manifest schema version
rwp_version: "0.26.0"          # RWP protocol version
```

This separation allows individual schemas to evolve at different paces from the protocol itself. A schema may add optional fields without requiring a protocol-version bump, and a protocol release may ship without changing every schema.

| RWP Version | Manifest Schema | State Schema | Plan Schema |
|-------------|-----------------|--------------|-------------|
| `0.26.0` | `1.0` | `1.0` | `1.0` |

Future RWP versions will document their corresponding schema versions in their release entries in [CHANGELOG.md](../CHANGELOG.md).

## References

- **SemVer 2.0.0**: Semantic Versioning - https://semver.org/
- **RFC 4122**: UUID URN Namespace - https://tools.ietf.org/html/rfc4122
- **ISO 8601**: Date and Time Formats - https://en.wikipedia.org/wiki/ISO_8601

## Revision History

| Date | Version | Changes |
|------|---------|---------|
| 2026-04-27 | 0.26.0 | Current version reference; pre-1.0 stability rules clarified. |

---

*Specification produced by YAKKL® - https://yakkl.com*
*RWP Reference Implementation: YAKKL® Meridian™ - https://meridian.yakkl.com*
