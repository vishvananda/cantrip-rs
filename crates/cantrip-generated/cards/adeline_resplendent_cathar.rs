// Adeline, Resplendent Cathar
// status: ready
// Vigilance
// Adeline's power is equal to the number of creatures you control.
// Whenever you attack, for each opponent, create a 1/1 white Human creature token that's tapped and attacking that player or a planeswalker they control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.power = Some(((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16); }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AttackersDeclared { player, count } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if *count > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Human).name, token_words: token!(Human).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut gathered_1: Vec<(CardId, PlayerId, Target)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let def_2 = g.card_id(&token_3::CARD); let count_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let defender_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let candidates_2 = cantrip_mechanics::combat::player_or_their_planeswalkers(g, defender_2); for _ in 0..count_2 { if let Some(target) = cantrip_mechanics::combat::choose_among(g, player_2, asker.source, &candidates_2)? { gathered_1.push((def_2, player_2, target)); } }
    } 
let words_1 = g.words(asker.source); let mut created_1: Vec<ObjectId> = Vec::new();
let mut done_1: Vec<Target> = Vec::new(); for &(_, _, target) in &gathered_1 { if done_1.contains(&target) { continue; } done_1.push(target); let group: Vec<(CardId, PlayerId)> = gathered_1.iter().filter(|row| row.2 == target).map(|row| (row.0, row.1)).collect(); created_1.extend(g.create_tokens_worded(&group, true, Some(target), words_1)?); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Adeline, Resplendent Cathar", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Knight]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(0), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKERS_DECLARED.0), ability_2_event), ability_2, "Whenever you attack, for each opponent, create a 1/1 white Human creature token that's tapped and attacking that player or a planeswalker they control.").targets(&[])], statics: &[characteristic_defining(Layer::PtDefine, ability_1_apply_0, "Adeline's power is equal to the number of creatures you control.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD];
