# GitHub Org Profile Staging

This directory stages the contents of the `rhumbprotocol/.github` repository,
which controls what visitors see at https://github.com/rhumbprotocol.

## How GitHub Org Profiles Work

GitHub renders a `profile/README.md` from a special repository named `.github`
inside the organization. The repository must be **public** for the README to
display on the org landing page.

```
github.com/rhumbprotocol/.github
└── profile/
    └── README.md   ← rendered at github.com/rhumbprotocol
```

## To Deploy

1. Create the repository: `rhumbprotocol/.github` (must be public).
2. Copy the `profile/` subdirectory from this staging location into that repo.
3. Commit and push to `main`.
4. Visit https://github.com/rhumbprotocol to verify the README renders.

## Why This Is Staged Here

The `rhumbprotocol/.github` repository does not yet exist. This staging
directory keeps the content version-controlled alongside the spec and lets
us iterate before the org profile goes live. Once the `.github` repo exists,
this directory may be removed or kept as a mirror copy, at your discretion.
