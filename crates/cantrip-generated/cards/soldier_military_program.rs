// SOLDIER Military Program
// status: planned
// requires: engine: no commander
// At the beginning of combat on your turn, choose one. If you control a commander, you may choose both instead.
// • Create a 1/1 white Soldier creature token.
// • Put a +1/+1 counter on each of up to two Soldiers you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Soldier).name, token_words: token!(Soldier).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
{ let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Soldier)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((2i64).max(0) as u32) }; let chosen = cantrip_mechanics::choose::select(g, player, asker.source, Purpose::PutCounters(CounterKind::PlusOne, amount.clamp(0, 255) as u8), candidates, cantrip_mechanics::select::Selection::of(count, &[]))?; for object in chosen { counters_2.push((object, CounterKind::PlusOne, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn ability_0_more_modes(q: &dyn Query, asker: &Asker) -> bool { q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && planned_value!("engine: no commander", false => q.is_commander(id))).count() >= 1 }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Create a 1/1 white Soldier creature token.").targets(&[]),mode(ability_0_mode_1, "Put a +1/+1 counter on each of up to two Soldiers you control.").targets(&[])]).choose(1, 1).up_to_if(ability_0_more_modes, 2);

pub static CARD: CardDef = CardDef { name: "SOLDIER Military Program", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), &ABILITY_0_MODES, "At the beginning of combat on your turn, choose one. If you control a commander, you may choose both instead. Create a 1/1 white Soldier creature token; or Put a +1/+1 counter on each of up to two Soldiers you control.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
