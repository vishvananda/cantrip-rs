// Tilonalli's Summoner
// status: ready
// Ascend (If you control ten or more permanents, you get the city's blessing for the rest of the game.)
// Whenever this creature attacks, you may pay {X}{R}. If you do, create X 1/1 red Elemental creature tokens that are tapped and attacking. At the beginning of the next end step, exile those tokens unless you have the city's blessing.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Elemental).name, token_words: token!(Elemental).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(8)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; !(Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).citys_blessing)) } {
let objects_5 = { let q: &dyn Query = g; about(q, item, 0) }; let moved_5 = g.exile_all(&objects_5);
g.end_sentence();
hoisted_6.extend(moved_5.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let x_0 = g.may_pay_x(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?; let paid_0 = x_0.is_some(); let bound_0: i64 = i64::from(x_0.unwrap_or(0));
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (bound_0).clamp(0, 4294967295) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let mut chosen_1: Vec<Target> = Vec::new(); for &(_, player) in &tokens_1 { chosen_1.push(cantrip_mechanics::combat::choose_attack_target(g, player, asker.source)?.unwrap_or(Target::None)); } let mut created_1: Vec<ObjectId> = Vec::new(); let mut done_1: Vec<Target> = Vec::new(); for &target in &chosen_1 { if done_1.contains(&target) { continue; } done_1.push(target); let group: Vec<_> = tokens_1.iter().zip(&chosen_1).filter(|&(_, &t)| t == target).map(|(&token, _)| token).collect(); created_1.extend(g.create_tokens_worded(&group, true, Some(target), words_1)?); }
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_1.clone() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
hoisted_8.extend(created_1.clone());
}
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Tilonalli's Summoner", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Shaman]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, you may pay {X}{R}. If you do, create X 1/1 red Elemental creature tokens that are tapped and attacking. At the beginning of the next end step, exile those tokens unless you have the city's blessing.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_4, "Whenever this creature attacks, you may pay {X}{R}. If you do, create X 1/1 red Elemental creature tokens that are tapped and attacking. At the beginning of the next end step, exile those tokens unless you have the city's blessing.")], enters: None, modal: None, enchant: None, ascend: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
