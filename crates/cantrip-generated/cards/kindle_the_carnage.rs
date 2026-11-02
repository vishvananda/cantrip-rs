// Kindle the Carnage
// status: ready
// Discard a card at random. If you do, Kindle the Carnage deals damage equal to that card's mana value to each creature. You may repeat this process any number of times.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
loop { let mut again_0 = false;
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_1, after_1, before_1, picked_1): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if g.state().zone(Zone::hand(payer_1)).len() >= 1 && cantrip_mechanics::effects::ask_may(g, payer_1, asker.source, Purpose::Optional)? {
let owed_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 } as usize; let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_2 = g.state().zone(Zone::hand(player_2)).len() >= owed_2; let hand_2 = g.state().zone(Zone::hand(player_2)).to_vec(); if paid_2 { cantrip_mechanics::discard_at_random(g, player_2, owed_2); } let discarded_2: Vec<ObjectId> = hand_2.iter().filter_map(|&o| g.state().obj(o).became).collect();
(paid_2, { let q: &dyn Query = g; (discarded_2).clone() }, { let q: &dyn Query = g; (discarded_2.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_1 as i64) != 0) } {
let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let amount_3 = { let q: &dyn Query = g; ((before_1.clone()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).min(u16::MAX.into()) as u16 }; let recipients_3 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_3 { let batch: Vec<(ObjectId, Target, u16)> = recipients_3.iter().map(|t| (source, *t, amount_3)).collect(); g.damage_together(&batch); }
g.end_sentence();
let decider_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_4, asker.source, Purpose::Optional)? {
again_0 = true;
g.end_sentence();
}
g.end_sentence();
}
g.end_sentence();
hoisted_6.extend(after_1.clone());
hoisted_7.extend({ let q: &dyn Query = g; before_1.clone() });
if !again_0 { break; } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Kindle the Carnage", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
