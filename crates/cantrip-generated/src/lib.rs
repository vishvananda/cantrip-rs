//! Every card as mtgish generates it, tracked by data/cantrip_tracker.json there.
//!
//! `planned!` stands where a card needs something the generator or Cantrip does
//! not have yet. `planned!("id" => code)` holds the code the card will have once
//! that exists, and `planned!("id", Form, "field" = value)` holds what the
//! generator knows so far, its values still type-checked.
//! `planned_value!("feature", placeholder => code)` compiles as the placeholder and
//! will say `code`. The rest of the file is the card's own code. A card with a hole
//! never runs.
//!
//! With the `proposals` feature a hole is the code it holds instead, so what the
//! generator proposes is type-checked: the planned cards only (planned.txt), against
//! stubs of the Cantrip API they name that mtgish's tools/proposal_check.py writes.
#![allow(dead_code, unused_imports, unused_mut, unused_variables, unused_macros)]
#![allow(unused_assignments, unused_must_use, unreachable_code)]

#[cfg(not(feature = "proposals"))]
macro_rules! planned {
    ($id:literal => $($code:tt)*) => {};
    ($id:literal, $form:ident $(, $field:literal = $value:expr)* $(,)?) => {
        $(let _ = &$value;)*
    };
}
#[cfg(not(feature = "proposals"))]
macro_rules! planned_value {
    ($feature:literal, $placeholder:expr => $($code:tt)*) => {
        $placeholder
    };
}
#[cfg(not(feature = "proposals"))]
macro_rules! planned_item {
    ($id:literal => $($code:tt)*) => {};
    ($id:literal, $source:literal) => {
        const _: &str = $source;
    };
}

#[cfg(feature = "proposals")]
macro_rules! planned {
    ($id:literal => $($code:tt)*) => {
        $($code)*
    };
    ($id:literal, $form:ident $(, $field:literal = $value:expr)* $(,)?) => {
        $(let _ = &$value;)*
    };
}
#[cfg(feature = "proposals")]
macro_rules! planned_value {
    ($feature:literal, $placeholder:expr => $($code:tt)*) => {
        $($code)*
    };
}
/// `planned_item!("id" => list: code)`: `code` is an entry of the `CardDef` list `list`, or an item (a trigger's
/// matcher). An entry is checked as the card's static will hold it: `@value` marks a field that is not a list and `@new`
/// a list `CardDef` does not have yet (proposal_check.py marks them; unmarked is a list).
/// `planned_item!("id" => item)` is the item: a function or static the card's tables will name.
#[cfg(feature = "proposals")]
macro_rules! planned_item {
    ($id:literal => $list:ident : $(#[$meta:meta])* fn $($code:tt)*) => {
        $(#[$meta])* fn $($code)*
    };
    ($id:literal => @value $list:ident : $($code:tt)*) => {
        const _: () = {
            static ENTRY: ::cantrip_api::CardDef =
                ::cantrip_api::CardDef { $list: $($code)*, ..::cantrip_api::CardDef::BLANK };
        };
    };
    ($id:literal => @new $list:ident : $($code:tt)*) => {
        const _: () = {
            static ENTRY: &dyn $crate::proposals::NewCardEntry = &$($code)*;
        };
    };
    ($id:literal => $list:ident : $($code:tt)*) => {
        const _: () = {
            static ENTRY: ::cantrip_api::CardDef =
                ::cantrip_api::CardDef { $list: &[$($code)*], ..::cantrip_api::CardDef::BLANK };
        };
    };
    ($id:literal, $source:literal) => {
        const _: &str = $source;
    };
    ($id:literal => $($item:tt)*) => {
        $($item)*
    };
}

include!(concat!(env!("OUT_DIR"), "/modules.rs"));
