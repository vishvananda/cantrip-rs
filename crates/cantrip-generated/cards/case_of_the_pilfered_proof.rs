// Case of the Pilfered Proof
// status: planned
// requires: replacement of token creation with other tokens
// Whenever a Detective you control enters or is turned face up, put a +1/+1 counter on it.
// To solve — You control three or more Detectives. (If unsolved, solve at the beginning of your end step.)
// Solved — If one or more tokens would be created under your control, those tokens plus a Clue token are created instead. (It's an artifact with "{2}, Sacrifice this token: Draw a card.")
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Detective)) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
if let Event::TurnedFaceUp { permanent, .. } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Detective)) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::PlusOne, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn ability_1_to_solve(q: &dyn Query, asker: &Asker) -> bool { ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Detective)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (3i64) }
#[allow(unused_variables)] fn ability_1_solved_0_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::TokenCreation(t) = proposal { let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if t.count > 0 && true && Some(asker.controller) == Some(t.controller) && q.is_solved(asker.source) { return true; } }
 false }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Clue).name, token_words: token!(Clue).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Clue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS), ability_0, "Draw 1 cards.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
#[allow(unused_variables, unused_mut)] fn ability_1_solved_0_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::TokenCreation(seen) = &seen { let t = seen; let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if let Proposal::TokenCreation(t) = proposal { let each = t.count; t.count = t.count.saturating_mul(1); planned_value!("replacement of token creation with other tokens", () => t.also(q.card_id(&token_1::CARD), (1i64).max(0) as u32, Words::NONE)); } } }

pub static CARD: CardDef = CardDef { name: "Case of the Pilfered Proof", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Case]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::TURNED_FACE_UP.0), ability_0_event), ability_0, "Whenever a Detective you control enters or is turned face up, put a +1/+1 counter on it.").targets(&[]),cantrip_mechanics::solved::to_solve(ability_1_to_solve, "To solve (If unsolved, solve at the beginning of your end step.)")], statics: &[], replacements: &[replacement(ReplacementKind::TokenCreation, ability_1_solved_0_0_applies, ability_1_solved_0_0_apply, "To solve — You control three or more Detectives.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
