// Djinni Windseer
// status: ready
// Flying
// When this creature enters, roll a d20.
// 1—9 | Scry 1.
// 10—19 | Scry 2.
// 20 | Scry 3.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_0 = i64::from(g.roll_die(player_0, 20).result);
g.end_sentence();
if { let q: &dyn Query = g; ((value_0) >= (1i64) && (value_0) <= (9i64)) } {
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_1, asker.source, amount_1)?;
g.end_sentence();
}
if { let q: &dyn Query = g; ((value_0) >= (10i64) && (value_0) <= (19i64)) } {
let amount_2 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_2, asker.source, amount_2)?;
g.end_sentence();
}
if { let q: &dyn Query = g; (value_0) == (20i64) } {
let amount_3 = { let q: &dyn Query = g; (3i64).max(0) as usize }; let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_3, asker.source, amount_3)?;
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Djinni Windseer", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Djinn]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, roll a d20.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
