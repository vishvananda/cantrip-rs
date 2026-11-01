// Drop Tower
// status: planned
// requires: ability Visit
// requires: duration EndOfTurn
// Visit — Target creature gains flying until end of turn, or until any player rolls a 1, whichever comes first.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_visit_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_VISIT_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_visit_target_0)];
fn ability_0_visit(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("duration-end-of-turn", CreateContinuousEffect, "affected" = "{\"Objects\":{\"args\":{\"args\":{\"index\":0,\"slot\":0},\"kind\":\"Target\"},\"kind\":\"One\"}}", "changes" = "[{\"args\":{\"kind\":\"Flying\"},\"kind\":\"GrantKeyword\"}]", "duration" = "{\"Either\":[\"EndOfTurn\",{\"UntilEvent\":{\"Filtered\":{\"condition\":{\"args\":{\"comparison\":\"Equal\",\"left\":{\"kind\":\"EventAmount\"},\"right\":{\"args\":1,\"kind\":\"Constant\"}},\"kind\":\"Compare\"},\"event\":{\"Player\":{\"event\":\"RollsADie\",\"objects\":(),\"players\":{\"kind\":\"All\"}}}}}}]}", "evaluation" = "\"Snapshot\"", "source" = { let q: &dyn Query = g; Some(asker.source) });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("ability-visit" => abilities: cantrip_mechanics::attractions::visit(spell(ABILITY_0_VISIT_TARGETS, ability_0_visit), None, "Change characteristics until end of turn."));

pub static CARD: CardDef = CardDef { name: "Drop Tower", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Attraction]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
