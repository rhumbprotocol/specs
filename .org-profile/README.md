# GitHub Org Profile Staging

This directory mirrors the org profile content now staged in `.github/`.
The deploy target is the `rhumbprotocol/.github` repository, which controls
what visitors see at https://github.com/rhumbprotocol.

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
2. Copy `.github/profile/` and `.github/assets/` into that repo.
3. Commit and push to `main`.
4. Visit https://github.com/rhumbprotocol to verify the README renders.

## Why This Is Staged Here

The `rhumbprotocol/.github` repository may be managed separately from the
specification repository. Keeping this mirror in `specs` makes the org profile
reviewable with the protocol source, while `.github/` contains the files GitHub
uses directly when this repository itself is viewed.
