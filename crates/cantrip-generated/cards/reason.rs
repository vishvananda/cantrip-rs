// Reason // Believe
// status: ready
// Scry 3.
// Aftermath (Cast this spell only from your graveyard. Then exile it.)
// Look at the top card of your library. You may put it onto the battlefield if it's a creature card. If you don't, put it into your hand.
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Reason // Believe", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, split: Some((&left::CARD, &right::CARD)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&left::CARD,&right::CARD];
pub mod left {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_0, asker.source, amount_0)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Reason", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
pub mod right {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_0, &objects_0);
g.end_sentence();
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_1 = cantrip_mechanics::effects::ask_may(g, payer_1, asker.source, Purpose::Optional)?; if paid_1 {
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let top_2 = cantrip_mechanics::top_of_library(g, player_2, 1);
let matching_2: Vec<ObjectId> = { let q: &dyn Query = g; top_2.iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect() };
let objects_3 = ({ let q: &dyn Query = g; matching_2.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows_3: Vec<(ObjectId, PlayerId, bool)> = objects_3.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_3 = g.put_all_onto_battlefield_as(&rows_3)?; came_3.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_4.extend(moved_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_1 as i64) != 0)) } {
let objects_6 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_6 = g.return_to_hand(&objects_6);
g.end_sentence();
hoisted_7.extend(moved_6.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Believe", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, aftermath: true, ..CardDef::BLANK };
}
