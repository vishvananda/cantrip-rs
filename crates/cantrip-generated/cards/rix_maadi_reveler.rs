// Rix Maadi Reveler
// status: ready
// Spectacle {2}{B}{R} (You may cast this spell for its spectacle cost rather than its mana cost if an opponent lost life this turn.)
// When this creature enters, discard a card, then draw a card. If this creature's spectacle cost was paid, instead discard your hand, then draw three cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| { let o = q.state().obj(candidate); o.cast_way > 0 && q.card(o.card).ways.get(o.cast_way as usize - 1).is_some_and(|w| w.text == "Spectacle") }) } {
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_0 = Vec::new(); for player in players_0 { cards_0.extend(g.state().zone(Zone::hand(player)).to_vec()); } let discarded_0 = g.discard_together(&cards_0);
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
hoisted_2.extend(discarded_0.clone());
hoisted_3.extend({ let q: &dyn Query = g; discarded_0.clone() });
} else {
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_4: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_4 = Vec::new(); for player in players_4 { cards_4.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_4)?); } let discarded_4 = g.discard_together(&cards_4);
g.end_sentence();
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.draw(player, amount_5); }
g.end_sentence();
hoisted_6.extend(discarded_4.clone());
hoisted_7.extend({ let q: &dyn Query = g; discarded_4.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Rix Maadi Reveler", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Shaman]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, discard a card, then draw a card. If this creature's spectacle cost was paid, instead discard your hand, then draw three cards.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::spectacle::way(ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Spectacle {2}{B}{R}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
