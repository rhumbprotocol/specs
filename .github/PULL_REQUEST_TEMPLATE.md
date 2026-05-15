## Summary

Describe the protocol, schema, template, validator, documentation, or integration change.

## Change Type

- [ ] Protocol/specification
- [ ] Schema
- [ ] Template
- [ ] Validator/conformance
- [ ] Integration adapter
- [ ] Documentation/site
- [ ] Governance/legal/trademark
- [ ] Examples/fixtures

## RWP Contract Impact

- [ ] No protocol contract change
- [ ] Backward-compatible clarification or fix
- [ ] Breaking pre-1.0 protocol change

If this changes artifact shape, IDs, statuses, lifecycle rules, template names, or validator behavior, explain the exact contract impact.

## Validation

Run the relevant checks before requesting review:

```bash
cargo test --manifest-path conformance/Cargo.toml
cargo run --manifest-path conformance/Cargo.toml -- --all --target conformance/fixtures/valid
cargo run --manifest-path conformance/Cargo.toml -- --category template --target templates
cargo run --manifest-path conformance/Cargo.toml -- --category workflow --target examples
cargo run --manifest-path conformance/Cargo.toml -- --category adapter --target integrations
```

Paste results:

## Checklist

- [ ] Version impact assessed against `VERSION` and `CHANGELOG.md`.
- [ ] New or changed schema behavior has valid and invalid fixtures.
- [ ] New or changed templates are included in the template validator list.
- [ ] Examples use `MP-NNNN-short-name` plan IDs where plan IDs are required.
- [ ] Status values use current RWP vocabulary.
- [ ] Documentation links use `https://github.com/rhumbprotocol/specs`.

