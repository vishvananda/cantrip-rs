//! Every card as mtgish generates it, tracked by data/cantrip_tracker.json there.
//!
//! `planned!` stands where a card needs something the generator or Cantrip does
//! not have yet. `planned!("id" => code)` holds the code the card will have once
//! that exists, and `planned!("id", Form, "field" = value)` holds what the
//! generator knows so far, its values still type-checked.
//! `planned_value!("feature", placeholder => code)` compiles as the placeholder and
//! will say `code`. The rest of the file is the card's own code. A card with a hole
//! never runs.
#![allow(dead_code, unused_imports, unused_mut, unused_variables, unused_macros)]
#![allow(unused_assignments, unused_must_use, unreachable_code)]

macro_rules! planned {
    ($id:literal => $($code:tt)*) => {};
    ($id:literal, $form:ident $(, $field:literal = $value:expr)* $(,)?) => {
        $(let _ = &$value;)*
    };
}
macro_rules! planned_value {
    ($feature:literal, $placeholder:expr => $($code:tt)*) => {
        $placeholder
    };
}
macro_rules! planned_item {
    ($id:literal => $($code:tt)*) => {};
    ($id:literal, $source:literal) => {
        const _: &str = $source;
    };
}

include!(concat!(env!("OUT_DIR"), "/modules.rs"));
