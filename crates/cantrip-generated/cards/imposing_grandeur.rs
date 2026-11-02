// Imposing Grandeur
// status: planned
// requires: engine: no commander
// Each player may discard their hand and draw cards equal to the greatest mana value of a commander they own on the battlefield or in the command zone.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut guard_0_0 = vec![false; players_0.len()];
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_0_0[index_0] = true;
gathered_1.extend(g.state().zone(Zone::hand(bound_0)).to_vec());
   }
 } 
let cards_3 = { let q: &dyn Query = g; gathered_1.clone() }; let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if guard_0_0[index_0] {
let amount_4 = { let q: &dyn Query = g; (({ let mut all: Vec<ObjectId> = Vec::new(); for id in (q.state().zone(Zone::COMMAND).to_vec()).into_iter().filter(|&id| true && (planned_value!("engine: no commander", false => q.is_commander(id)) && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in q.state().battlefield().iter().copied().filter(|&id| (planned_value!("engine: no commander", false => q.is_commander(id)) && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).max().unwrap_or(0)).clamp(0, 4294967295) as u32 }; let players_4 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
   }
 } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Imposing Grandeur", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
