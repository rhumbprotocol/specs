# Example: Simple Feature - Add Dark Mode Toggle

This example demonstrates RWP for a small project with 2 phases, a single developer,
and minimal artifacts. It shows the **REQUIRED-only** approach - the lightest way to
use RWP while remaining protocol-compliant.

## What's Included

| File | RWP Artifact Type | Purpose |
|------|-------------------|---------|
| `INTAKE.yaml` | Intake | Captures the problem and requirements |
| `PLAN.md` | Plan | 2-phase plan with tasks and verification |
| `state.yaml` | State | Tracks execution progress |
| `handoffs/HO-RWP-0042-P-01-2026-03-01.md` | Handoff | Session boundary document |

## Why This Example

This is the simplest realistic RWP workflow:
- **2 phases** (implement + polish), no sub-phases needed
- **Single developer**, no team coordination
- **~2 hours** of total work
- **No audit** (small scope doesn't justify formal audit overhead)
- **Minimal custom fields** - only REQUIRED fields populated

## How to Use

1. Read `INTAKE.yaml` to understand the problem capture format
2. Read `PLAN.md` to see how phases decompose the work
3. Read `state.yaml` to see execution tracking after P-01 completes
4. Read the handoff to see session continuity between phases

## Key Takeaway

RWP works for small tasks too. You don't need every field - just the REQUIRED ones
from the [Protocol Specification](../../docs/PROTOCOL.md).

---

Rhumb Workflow Protocol (RWP) v0.25.1 - https://rhumbprotocol.dev
