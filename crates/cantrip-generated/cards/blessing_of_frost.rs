// Blessing of Frost
// status: planned
// requires: engine: snow mana told apart as a spell is paid for
// Distribute X +1/+1 counters among any number of creatures you control, where X is the amount of {S} spent to cast this spell. Then draw a card for each creature you control with power 4 or greater.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_0: u32 = { let q: &dyn Query = g; (planned_value!("engine: snow mana told apart as a spell is paid for", 0i64 => Some(asker.source).map_or(0, |o: ObjectId| q.history().mana_spent_to_cast(o).filter(|(source, _)| *source != ObjectId::NONE && q.chars(*source).supertypes.snow).map(|(_, mana)| i64::from(mana.total())).sum::<i64>()))).max(0) as u32 }; let mut puts_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new(); for (i, &object) in objects_0.iter().enumerate() { let after = (objects_0.len() - i - 1) as u32; let n = if after == 0 { left_0 } else { let most = left_0.saturating_sub(after * 0); cantrip_mechanics::choose::number(g, asker.controller, asker.source, 0.min(most), most, "Distribute")? }; left_0 -= n.min(left_0); if n > 0 { puts_0.push((object, CounterKind::PlusOne, n as i16)); } } g.add_counters_together(&puts_0);
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && q.chars(id).power.is_some_and(|v| (i64::from(v)) >= (4i64)))).collect::<Vec<ObjectId>>()).len() as i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Blessing of Frost", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: true }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
