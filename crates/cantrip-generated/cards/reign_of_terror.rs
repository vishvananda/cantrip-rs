// Reign of Terror
// status: planned
// requires: instruction ChooseOne
// Destroy all green creatures or all white creatures. They can't be regenerated. You lose 2 life for each creature that died this way.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_0, asker.source, vec![Label::Card(card_0, 0), Label::Card(card_0, 1)])?) {
0 => {
let objects_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).colors.has(q.words(asker.source).color(Color::Green)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let destroyed_1 = g.destroy_all_no_regen(&objects_1);
g.end_sentence();
hoisted_2.extend(destroyed_1.clone());
hoisted_3.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
_ => {
let objects_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).colors.has(q.words(asker.source).color(Color::White)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let destroyed_4 = g.destroy_all_no_regen(&objects_4);
g.end_sentence();
hoisted_5.extend(destroyed_4.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
let mut joined_7: Vec<ObjectId> = Vec::new();
joined_7.extend(hoisted_2.clone());
joined_7.extend(hoisted_5.clone());
let mut joined_8: Vec<ObjectId> = Vec::new();
joined_8.extend(hoisted_2.clone());
joined_8.extend(hoisted_5.clone());
g.end_sentence();
let amount_9 = { let q: &dyn Query = g; ((2i64 * ((joined_7.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).len() as i64)).clamp(0, 2147483647) as i32 }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_9 { g.lose_life(player, amount_9); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Reign of Terror", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
