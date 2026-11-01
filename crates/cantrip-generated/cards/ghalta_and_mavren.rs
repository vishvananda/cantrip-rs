// Ghalta and Mavren
// status: planned
// requires: dynamic defining token characteristics
// Trample
// Whenever you attack, choose one —
// • Create a tapped and attacking X/X green Dinosaur creature token with trample, where X is the greatest power among other attacking creatures.
// • Create X 1/1 white Vampire creature tokens with lifelink, where X is the number of other attacking creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AttackersDeclared { player, count } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if *count > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Dinosaur).name, token_words: token!(Dinosaur).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&planned_value!("dynamic defining token characteristics", token_1::CARD => token_1::CARD.sized_as_created(|q: &dyn Query, asker: &Asker| ((q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && (q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))).max().unwrap_or(0), (q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && (q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))).max().unwrap_or(0)))));
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let mut chosen_0: Vec<Target> = Vec::new(); for &(_, player) in &tokens_0 { chosen_0.push(cantrip_mechanics::combat::choose_attack_target(g, player, asker.source)?.unwrap_or(Target::None)); } let mut created_0: Vec<ObjectId> = Vec::new(); let mut done_0: Vec<Target> = Vec::new(); for &target in &chosen_0 { if done_0.contains(&target) { continue; } done_0.push(target); let group: Vec<_> = tokens_0.iter().zip(&chosen_0).filter(|&(_, &t)| t == target).map(|(&token, _)| token).collect(); created_0.extend(g.create_tokens_worded(&group, true, Some(target), words_0)?); }
g.end_sentence();
 Ok(()) }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Vampire).name, token_words: token!(Vampire).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_2 = Vec::new();
let count_2_0 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && (q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).len() as i64).max(0) as u32 }; let def_2_0 = g.card_id(&token_3::CARD);
for player in players_2 {
tokens_2.extend(std::iter::repeat((def_2_0,player)).take(count_2_0 as usize));
} let words_2 = g.words(asker.source);
let created_2 = g.create_tokens_worded(&tokens_2, false, None, words_2)?;
g.end_sentence();
 Ok(()) }
static ABILITY_1_MODES: ModalDef = modes(&[mode(ability_1_mode_0, "Create a tapped and attacking X/X green Dinosaur creature token with trample, where X is the greatest power among other attacking creatures.").targets(&[]),mode(ability_1_mode_1, "Create X 1/1 white Vampire creature tokens with lifelink, where X is the number of other attacking creatures.").targets(&[])]).choose(1, 1);

pub static CARD: CardDef = CardDef { name: "Ghalta and Mavren", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur,Subtype::Vampire]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(12), toughness: Some(12), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::ATTACKERS_DECLARED.0), ability_1_event), &ABILITY_1_MODES, "Whenever you attack, choose one — Create a tapped and attacking X/X green Dinosaur creature token with trample, where X is the greatest power among other attacking creatures; or Create X 1/1 white Vampire creature tokens with lifelink, where X is the number of other attacking creatures.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD,&token_3::CARD];
