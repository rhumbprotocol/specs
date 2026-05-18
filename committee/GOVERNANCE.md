# RWP Governance

This document describes how changes to the Rhumb Workflow Protocol are proposed, reviewed, and approved.

---

## Decision-Making Process

RWP uses a **lazy consensus** model for most decisions, with formal voting reserved for significant changes.

### Lazy Consensus

Most changes (documentation fixes, template improvements, new integration adapters) are approved through lazy consensus:

1. A contributor opens a pull request
2. Maintainers review the change
3. If no objections are raised within a reasonable period (typically 7 days for non-trivial changes), the change is merged
4. Any maintainer can merge after the review period if there are no objections

Lazy consensus keeps the process lightweight and encourages contribution.

### Formal Review

Changes that affect the core protocol specification or schemas go through formal review:

1. **Proposal** - Open an issue describing the change, its motivation, and potential impact
2. **Discussion** - Community discussion period (minimum 14 days for specification changes)
3. **Pull Request** - Submit the change with updated documentation
4. **Review** - At least 2 maintainer approvals required
5. **Merge** - Chair or designated maintainer merges after approval

### Voting

Formal voting is reserved for:

- **Breaking changes** to the protocol specification
- **New major versions** (v2.0.0, v3.0.0)
- **Committee membership** changes
- **Charter amendments**

Voting rules:
- Each maintainer gets one vote
- Simple majority for most decisions
- Two-thirds majority for breaking changes and major versions
- Chair breaks ties
- Voting period: 14 days minimum
- Votes are recorded in the pull request or issue

---

## Change Categories

| Category | Process | Approvals | Timeline |
|----------|---------|-----------|----------|
| Typo/grammar fix | Lazy consensus | 1 maintainer | 1-3 days |
| Documentation improvement | Lazy consensus | 1 maintainer | 3-7 days |
| New template | Lazy consensus | 1 maintainer | 7 days |
| New integration adapter | Lazy consensus | 1 maintainer | 7 days |
| Schema change (additive) | Formal review | 2 maintainers | 14 days |
| Specification change | Formal review | 2 maintainers | 14 days |
| Breaking change | Formal vote | 2/3 majority | 14-30 days |
| New major version | Formal vote | 2/3 majority | 30 days |

---

## Proposal Process (RWP Enhancement Proposals)

For significant changes, contributors are encouraged to write an **RWP Enhancement Proposal (AEP)**:

### AEP Structure

```
Title: [Short descriptive title]
Author: [Name/handle]
Status: Draft | Discussion | Accepted | Rejected | Withdrawn
Created: [Date]

## Summary
[1-2 paragraph overview]

## Motivation
[Why is this change needed?]

## Proposal
[Detailed description of the change]

## Backward Compatibility
[How does this affect existing users?]

## Alternatives Considered
[What other approaches were evaluated?]
```

AEPs are stored in `committee/proposals/` and discussed in GitHub issues.

### AEP Lifecycle

```
Draft → Discussion → Accepted → Implemented
                  ↘ Rejected
                  ↘ Withdrawn
```

- **Draft**: Initial proposal, open for early feedback
- **Discussion**: Formally under review (minimum 14 days)
- **Accepted**: Approved for implementation
- **Rejected**: Not accepted (with documented rationale)
- **Withdrawn**: Author chose not to proceed

---

## Conflict Resolution

When disagreements arise:

1. **Discussion** - Participants present their positions with technical justification
2. **Compromise** - Look for solutions that address all concerns
3. **Chair mediation** - The chair facilitates discussion and suggests paths forward
4. **Vote** - If consensus cannot be reached, a formal vote decides the outcome

The goal is always to reach consensus. Voting is a last resort.

---

## Transparency

All governance activities are conducted in public:

- Decisions are recorded in GitHub issues, PRs, or meeting notes
- No private channels for protocol decisions
- Committee meeting notes are published in `committee/minutes/`
- Voting results are recorded in the relevant issue or PR

---

Rhumb Workflow Protocol (RWP) v0.31.0
https://rhumbprotocol.dev
