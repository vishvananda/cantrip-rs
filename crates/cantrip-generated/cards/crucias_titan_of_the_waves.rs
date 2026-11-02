// Crucias, Titan of the Waves
// status: ready
// At the beginning of your end step, you may discard a card. If you do, create a Treasure token and choose ambitious or expedient. If you chose ambitious, seek a card with greater mana value than the discarded card. If you chose expedient, seek a card with lesser mana value than the discarded card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
pub mod token_4 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Treasure).name, token_words: token!(Treasure).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Treasure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_22: Vec<ObjectId> = Vec::new();
let mut hoisted_21: Vec<ObjectId> = Vec::new();
let mut hoisted_20: Vec<ObjectId> = Vec::new();
let mut hoisted_19: Vec<ObjectId> = Vec::new();
let mut hoisted_18: Vec<ObjectId> = Vec::new();
let mut hoisted_17: Vec<ObjectId> = Vec::new();
let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_2 = Vec::new(); for player in players_2 { cards_2.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_2, true)? }); } let discarded_2 = g.discard_together(&cards_2);
let paid_1 = !discarded_2.clone().is_empty();
(paid_1, { let q: &dyn Query = g; (discarded_2).clone() }, { let q: &dyn Query = g; (discarded_2.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_3 = Vec::new();
let count_3_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_3_0 = g.card_id(&token_4::CARD);
for player in players_3 {
tokens_3.extend(std::iter::repeat((def_3_0,player)).take(count_3_0 as usize));
} let words_3 = Words::NONE;
let created_3 = g.create_tokens_worded(&tokens_3, false, None, words_3)?;
g.end_sentence();
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let card_5 = g.card_id(&CARD); let chosen_5: u8 = cantrip_mechanics::choose::option(g, player_5, asker.source, card_5, 0..2)?;
g.end_sentence();
if { let q: &dyn Query = g; chosen_5 == 0 } {
let mut pool_6 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (i64::from(q.mana_value(id))) > ((before_0.clone()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o))))).collect::<Vec<ObjectId>>() }; let count_6 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_6: Vec<ObjectId> = Vec::new(); while picked_6.len() < count_6 && !pool_6.is_empty() { let at = g.choose_at_random(pool_6.len()); picked_6.push(pool_6.remove(at)); }
let objects_7 = { let q: &dyn Query = g; picked_6.clone() }; let moved_7 = g.return_to_hand(&objects_7);
g.end_sentence();
hoisted_8.extend(moved_7.clone());
hoisted_9.extend(picked_6.clone());
hoisted_10.extend({ let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
if { let q: &dyn Query = g; chosen_5 == 1 } {
let mut pool_11 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (i64::from(q.mana_value(id))) < ((before_0.clone()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o))))).collect::<Vec<ObjectId>>() }; let count_11 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_11: Vec<ObjectId> = Vec::new(); while picked_11.len() < count_11 && !pool_11.is_empty() { let at = g.choose_at_random(pool_11.len()); picked_11.push(pool_11.remove(at)); }
let objects_12 = { let q: &dyn Query = g; picked_11.clone() }; let moved_12 = g.return_to_hand(&objects_12);
g.end_sentence();
hoisted_13.extend(moved_12.clone());
hoisted_14.extend(picked_11.clone());
hoisted_15.extend({ let q: &dyn Query = g; objects_12.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_16.extend(hoisted_8.clone());
hoisted_17.extend(hoisted_13.clone());
hoisted_18.extend(hoisted_9.clone());
hoisted_19.extend(hoisted_14.clone());
hoisted_20.extend(created_3.clone());
hoisted_21.extend({ let q: &dyn Query = g; hoisted_10.clone() });
hoisted_22.extend({ let q: &dyn Query = g; hoisted_15.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Crucias, Titan of the Waves", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Pirate]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your end step, you may discard a card. If you do, create a Treasure token and choose ambitious or expedient. If you chose ambitious, seek a card with greater mana value than the discarded card. If you chose expedient, seek a card with lesser mana value than the discarded card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Ambitious", "Expedient"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_4::CARD];
