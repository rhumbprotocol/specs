# Rhumb Workflow Protocol: Plan Document

---

plan_id: MP-0042-dark-mode-toggle
request_id: null
name: Add Dark Mode Toggle
classification: public
status: processing
created: 2026-03-01T10:15:00Z
author: Developer
phases: 2
current_phase: P-02
started: 2026-03-01T10:15:00Z
completed: null
parent: null
rwp_version: "0.27.0"
dependencies: []
packages:
  - frontend-app

---

# MP-0042-dark-mode-toggle: Add Dark Mode Toggle

---

## Executive Summary

Add a dark mode toggle to the settings page that switches between light and dark themes.
The user's preference persists in localStorage, and the default respects the OS-level
color scheme via `prefers-color-scheme`. This is a 2-phase effort: implement the core
toggle and CSS variables (P-01), then polish transitions and edge cases (P-02).

---

## Problem Statement

Users report eye strain when using the application at night. Multiple support tickets
have requested a dark mode option. The current app uses hardcoded light-only colors
with no theme switching capability.

---

## Phase Breakdown

### P-01: Core Implementation

**Depends On**: None
**Estimated Duration**: 60 minutes

**Objective**: Implement the dark mode toggle with CSS custom properties and localStorage persistence.

**Tasks**:
1. Create `src/styles/themes.css` with CSS custom properties for light and dark palettes
2. Create `src/components/settings/ThemeToggle.svelte` with toggle switch UI
3. Add `src/lib/theme.ts` utility for reading/writing theme preference to localStorage
4. Wire OS preference detection via `window.matchMedia('(prefers-color-scheme: dark)')`
5. Import `ThemeToggle` into `src/routes/settings/+page.svelte`

**Files to Create/Modify**:
| File | Action | Description |
|------|--------|-------------|
| `src/styles/themes.css` | Create | CSS custom properties for light/dark palettes |
| `src/components/settings/ThemeToggle.svelte` | Create | Toggle switch component |
| `src/lib/theme.ts` | Create | localStorage read/write + OS preference detection |
| `src/routes/settings/+page.svelte` | Modify | Import and render ThemeToggle |

**Verification**:
```bash
# Check files exist
ls src/styles/themes.css src/components/settings/ThemeToggle.svelte src/lib/theme.ts
# Build to verify no errors
pnpm build
```

**Expected Results**:
- Toggle switch visible on settings page
- Clicking toggle switches between light and dark themes
- Theme preference saved to localStorage
- Page reload preserves selected theme

---

### P-02: Polish and Edge Cases

**Depends On**: P-01
**Estimated Duration**: 45 minutes

**Objective**: Add smooth transitions, handle edge cases, and verify cross-browser behavior.

**Tasks**:
1. Add CSS transition on `background-color` and `color` properties (300ms ease)
2. Handle the flash-of-wrong-theme on page load (inline script in `app.html`)
3. Add `aria-label` and keyboard accessibility to the toggle
4. Test in Chrome, Firefox, and Safari
5. Update the settings page documentation

**Files to Create/Modify**:
| File | Action | Description |
|------|--------|-------------|
| `src/styles/themes.css` | Modify | Add transition properties |
| `src/app.html` | Modify | Add inline theme-init script to prevent flash |
| `src/components/settings/ThemeToggle.svelte` | Modify | Add ARIA attributes |

**Verification**:
```bash
pnpm build
pnpm test
```

**Expected Results**:
- Smooth color transitions when toggling
- No flash of wrong theme on page load
- Toggle accessible via keyboard (Tab + Enter/Space)
- Works in Chrome, Firefox, Safari

---

## Phase Dependency Graph

```
P-01 (Core Implementation)
  │
  └──── P-02 (Polish and Edge Cases)
```

---

## Dependencies

| Dependency | Type | Status | Notes |
|------------|------|--------|-------|
| Svelte 5 | Package | Met | Already in project |
| CSS Custom Properties | Browser | Met | Supported in all target browsers |

---

## Files Reference

### New Files

| File | Purpose |
|------|---------|
| `src/styles/themes.css` | Light and dark CSS custom property palettes |
| `src/components/settings/ThemeToggle.svelte` | Toggle switch component |
| `src/lib/theme.ts` | Theme preference utilities (localStorage + OS detection) |

### Modified Files

| File | Changes |
|------|---------|
| `src/routes/settings/+page.svelte` | Import ThemeToggle component |
| `src/app.html` | Inline theme initialization script |

---

## Success Criteria

1. Toggle switch renders and is functional on settings page
2. Theme preference persists across page reloads
3. OS color scheme preference is respected on first visit
4. No flash of wrong theme on page load
5. Build passes with zero errors

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Flash of unstyled content | Medium | Low | Inline script in app.html sets theme before render |
| Third-party components ignore CSS variables | Low | Medium | Audit component styles, add fallbacks |

---

## Rollback Plan

If this plan needs to be reverted:

1. Remove `src/styles/themes.css`, `src/components/settings/ThemeToggle.svelte`, `src/lib/theme.ts`
2. Revert changes to `src/routes/settings/+page.svelte` and `src/app.html`
3. Clear `theme` key from localStorage

---

Produced: 2026-03-01T10:15:00Z
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
