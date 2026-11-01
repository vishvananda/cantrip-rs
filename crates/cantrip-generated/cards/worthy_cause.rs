// Worthy Cause
// status: ready
// Buyback {2} (You may pay an additional {2} as you cast this spell. If you do, put this card into your hand as it resolves.)
// As an additional cost to cast this spell, sacrifice a creature.
// You gain life equal to the sacrificed creature's toughness.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (({ let records = q.history().all(); let spell = q.history().original_spell(item.object); records.iter().rposition(|r| matches!(r.event, Event::Cast(c) if c.spell == spell)).map_or_else(Vec::new, |cast| records[..cast].iter().rev().filter_map(|r| match r.event { Event::ZoneChange { old, from, reason: ZoneReason::Sacrifice, .. } if from.is(ZoneKind::Battlefield) => Some(old), _ => None }).take(1).collect::<Vec<ObjectId>>()) }).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.chars(o).toughness.unwrap_or(0)))).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Worthy Cause", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_2), abilities: &[], statics: &[], replacements: &[cantrip_mechanics::replacements::BUYBACK], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(1, ability_1_0))), kicker: kicker(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
