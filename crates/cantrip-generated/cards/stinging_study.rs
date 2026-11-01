// Stinging Study
// status: planned
// requires: engine: no commander
// You draw X cards and you lose X life, where X is the mana value of a commander you own on the battlefield or in the command zone.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in (q.state().battlefield().to_vec()).into_iter().filter(|&id| true && (planned_value!("engine: no commander", false => q.is_commander(id)) && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in (q.state().zone(Zone::COMMAND).to_vec()).into_iter().filter(|&id| true && (planned_value!("engine: no commander", false => q.is_commander(id)) && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::ChooseAsEnters, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
g.end_sentence();
let value_1: i64 = { let q: &dyn Query = g; (selected_0.clone()).get(0 as usize).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o))) };
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (value_1).clamp(0, 4294967295) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (value_1).clamp(0, 2147483647) as i32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.lose_life(player, amount_3); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Stinging Study", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
