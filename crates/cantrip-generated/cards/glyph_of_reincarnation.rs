// Glyph of Reincarnation
// status: planned
// requires: player reference AsOf
// Cast this spell only after combat.
// Destroy all creatures that were blocked by target Wall this turn. They can't be regenerated. For each creature that died this way, put a creature card from the graveyard of the player who controlled that creature the last time it became blocked by that Wall onto the battlefield under its owner's control.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { (q.state().turn.step() as u8) > (Step::EndOfCombat as u8) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Wall)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Blocks { blocker, attacker, .. } = event { let id = *attacker; let other = *blocker; let subject = Target::Object(id); let data = [other.0, u32::MAX]; if Some(id) == Some(id) && t_0_raw.get(0).copied().and_then(|t: Target| t.object()) == Some(other) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64))).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all_no_regen(&objects_0);
g.end_sentence();
for bound_0 in { let q: &dyn Query = g; objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() } {
planned!("player-reference-as-of", ChooseObjects, "constraints" = "[]", "count" = "{\"AsManyAsCan\":{\"args\":1,\"kind\":\"Constant\"}}", "objects" = "{\"args\":{\"filter\":{\"args\":\"Creature\",\"kind\":\"CardType\"},\"kind\":\"Card\",\"scope\":{\"Players\":{\"players\":{\"args\":{\"args\":{\"moment\":{\"LastEvent\":{\"event\":{\"Pair\":{\"event\":\"BecomesBlockedBy\",\"objects\":{\"args\":{\"args\":0,\"kind\":\"Bound\"},\"kind\":\"SameObject\"},\"others\":{\"args\":{\"args\":{\"index\":0,\"slot\":0},\"kind\":\"Target\"},\"kind\":\"SameObject\"}}},\"window\":\"ThisTurn\"}},\"player\":{\"args\":{\"args\":0,\"kind\":\"Bound\"},\"kind\":\"ControllerOf\"}},\"kind\":\"AsOf\"},\"kind\":\"One\"},\"zone\":\"Graveyard\"}}},\"kind\":\"Zone\"}", "player" = { let q: &dyn Query = g; Some(asker.controller) });
let objects_2 = ({ let q: &dyn Query = g; planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_1) }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_2 = { let rows_2: Vec<(ObjectId, PlayerId, bool)> = objects_2.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_2 = g.put_all_onto_battlefield_as(&rows_2)?; came_2.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_3.extend(moved_2.clone());
hoisted_4.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Glyph of Reincarnation", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_only_if: Some(ability_0_if), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
