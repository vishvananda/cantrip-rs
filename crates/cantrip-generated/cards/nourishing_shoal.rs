// Nourishing Shoal
// status: planned
// requires: a cost's filter reading the X announced
// You may exile a green card with mana value X from your hand rather than pay this spell's mana cost.
// You gain X life.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (i64::from(item.x)) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Nourishing Shoal", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[Subtype::Arcane]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("a cost's filter reading the X announced", Cost::NONE => Cost::moves(&[Selection { objects: Picks::Matching(|q: &dyn Query, asker: &Asker, id: ObjectId| (q.state().obj(id).zone.kind() == ZoneKind::Hand && Some(asker.controller) == Some(q.state().obj(id).owner)) && (q.chars(id).colors.has(q.words(asker.source).color(Color::Green)) && (i64::from(q.mana_value(id))) == (i64::from(item.x)))), count: Count::Exactly(1i64) }], Destination::Exile, Arrival { tapped: false, face_down: false, states: &[] }))), ZoneKind::Hand, "You may exile a green card with mana value X from your hand rather than pay this spell's mana cost.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
