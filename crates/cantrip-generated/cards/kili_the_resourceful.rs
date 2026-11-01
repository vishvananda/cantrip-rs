// Kíli the Resourceful
// status: ready
// Storied (If you control three or more artifacts, legendaries, and/or Sagas, you have an enduring story for the rest of the game.)
// As long as you have an enduring story, you may pay {0} rather than pay the equip cost of the first equip ability you activate each turn.
// Whenever another Dwarf or Equipment you control enters, draw a card. This ability triggers only once each turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, permanent: ObjectId, def: &ActivatedDef) -> Option<ManaCost> { ((Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).enduring_story)) && player == asker.controller && def.equip && !cantrip_mechanics::history::activated_an_equip_ability_this_turn(q, player)).then_some(ManaCost::ZERO) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (!(Some(asker.source) == Some(id)) && ((q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Dwarf)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment))) && Some(asker.controller) == Some(q.controller(id)))) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Kíli the Resourceful", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dwarf,Subtype::Scout]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "Whenever another Dwarf or Equipment you control enters, draw a card. This ability triggers only once each turn.").targets(&[]).triggers_each_turn(1)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ability_alternatives: &[ability_1_0_0_rule], storied: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
