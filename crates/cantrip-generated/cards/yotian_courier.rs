// Yotian Courier
// status: planned
// requires: modes limited by earlier choices
// Flying
// Whenever this creature attacks, choose one that wasn't chosen during your last combat —
// • Create a tapped Powerstone token.
// • Seek a nonland card with mana value equal to the number of Powerstones you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Powerstone).name, token_words: token!(Powerstone).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Powerstone]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_each(&[]) }.only_for(ManaRestriction::NotNonartifactSpells), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = Words::NONE;
let created_0 = g.create_tokens_worded(&tokens_0, true, None, words_0)?;
g.end_sentence();
 Ok(()) }
fn ability_1_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_2 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (!(q.chars(id).is(CardType::Land)) && (i64::from(q.mana_value(id))) == ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Powerstone)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64))).collect::<Vec<ObjectId>>() }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_2: Vec<ObjectId> = Vec::new(); while picked_2.len() < count_2 && !pool_2.is_empty() { let at = g.choose_at_random(pool_2.len()); picked_2.push(pool_2.remove(at)); }
let objects_3 = { let q: &dyn Query = g; picked_2.clone() }; let moved_3 = g.return_to_hand(&objects_3);
g.end_sentence();
 Ok(()) }
static ABILITY_1_MODES: ModalDef = planned_value!("modes limited by earlier choices", modes(&[mode(ability_1_mode_0, "Create a tapped Powerstone token.").targets(&[]),mode(ability_1_mode_1, "Seek a nonland card with mana value equal to the number of Powerstones you control.").targets(&[])]).choose(1, 1) => modes(&[mode(ability_1_mode_0, "Create a tapped Powerstone token.").targets(&[]),mode(ability_1_mode_1, "Seek a nonland card with mana value equal to the number of Powerstones you control.").targets(&[])]).choose(1, 1).allowed(cantrip_mechanics::history::mode_not_chosen_last_combat(|q: &dyn Query, asker: &Asker| Some(asker.controller).expect("a reference that always names a player"))));

pub static CARD: CardDef = CardDef { name: "Yotian Courier", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), &ABILITY_1_MODES, "Whenever this creature attacks, choose one that wasn't chosen during your last combat — Create a tapped Powerstone token; or Seek a nonland card with mana value equal to the number of Powerstones you control.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
