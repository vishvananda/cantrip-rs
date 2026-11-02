// Lunarch Veteran // Luminous Phantom
// status: planned
// requires: engine: no Disturb
// Flying
// Whenever another creature you control leaves the battlefield, you gain 1 life.
// If Luminous Phantom would be put into a graveyard from anywhere, exile it instead.
// Whenever another creature you control enters, you gain 1 life.
// Disturb {1}{W} (You may cast this card from your graveyard transformed for its disturb cost.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (!(Some(asker.source) == Some(id)) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-disturb" => ways: cantrip_mechanics::disturb::way(ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Disturb {1}{W}"));

pub static CARD: CardDef = CardDef { name: "Lunarch Veteran", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Whenever another creature you control enters, you gain 1 life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && (!(Some(asker.source) == Some(id)) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Graveyard) && Some(asker.source) == Some(id) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_2_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.to = Zone::exile(q.state().obj(z.object).owner); } } }

pub static CARD: CardDef = CardDef { name: "Luminous Phantom", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever another creature you control leaves the battlefield, you gain 1 life.").targets(&[])], statics: &[], replacements: &[replacement(ReplacementKind::ZoneChange, ability_2_applies, ability_2_apply, "If Luminous Phantom would be put into a graveyard from anywhere, exile it instead.").own_everywhere()], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
