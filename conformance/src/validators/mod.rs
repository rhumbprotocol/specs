// SPDX-License-Identifier: Apache-2.0
//
// Validator modules. Implementations landed across MP-0276 P-02..P-06:
//   pub mod schema;    // P-02 (Category 1) — landed
//   pub mod template;  // P-03 (Category 2) — landed
//   pub mod workflow;  // P-04 (Category 3) — landed
//   pub mod adapter;   // P-05 (Category 4) — landed
//   pub mod grammar;   // P-06 (Category 5) — landed
//
// Shared infrastructure factored from the per-category validators:
//   pub(crate) mod walk; // recursive walker (factored P-05 from
//                        //   schema/template/workflow duplicates)
//
// Each validator exposes `pub fn run(target: &Path, result: &mut CategoryResult)`
// and is wired into `DISPATCH` below. lib::validate() iterates over the
// requested categories, looks each one up in DISPATCH, and runs the
// matching validator. With P-06 landed, all five categories are wired.

use std::path::Path;

use crate::{Category, CategoryResult};

pub(crate) mod walk;

pub mod adapter;
pub mod grammar;
pub mod schema;
pub mod template;
pub mod workflow;

/// Validator function pointer: takes a target tree and writes results into
/// the supplied `CategoryResult`. Mirrors the signature each P-02..P-06
/// validator module implements as `pub fn run(...)`.
pub(crate) type ValidatorFn = fn(&Path, &mut CategoryResult);

/// Dispatch table mapping a category to its validator entry point.
///
/// Order is Category::ALL canonical order for stability of the report's
/// `categories` vec. With P-06 landed, all five categories are wired.
const DISPATCH: &[(Category, ValidatorFn)] = &[
    (Category::Schema, schema::run),
    (Category::Template, template::run),
    (Category::Workflow, workflow::run),
    (Category::Adapter, adapter::run),
    (Category::Grammar, grammar::run),
];

/// Look up the validator for a category, if one is wired.
pub(crate) fn validator(category: Category) -> Option<ValidatorFn> {
    DISPATCH
        .iter()
        .find_map(|(c, f)| if *c == category { Some(*f) } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_has_all_five_categories_in_p06() {
        assert!(validator(Category::Schema).is_some());
        assert!(validator(Category::Template).is_some());
        assert!(validator(Category::Workflow).is_some());
        assert!(validator(Category::Adapter).is_some());
        assert!(validator(Category::Grammar).is_some());
    }

    #[test]
    fn dispatch_table_has_no_duplicate_categories() {
        // Defends against the table being mis-keyed (e.g., two rows
        // for Category::Schema pointing at different validators —
        // first match wins, but the table is malformed).
        let mut seen = std::collections::HashSet::new();
        for (cat, _) in DISPATCH {
            assert!(seen.insert(*cat), "duplicate category in DISPATCH: {cat:?}");
        }
        assert_eq!(seen.len(), Category::ALL.len());
    }
}
