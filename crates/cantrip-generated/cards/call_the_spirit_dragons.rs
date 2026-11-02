// Call the Spirit Dragons
// status: ready
// Dragons you control have indestructible.
// At the beginning of your upkeep, for each color, put a +1/+1 counter on a Dragon you control of that color. If you put +1/+1 counters on five Dragons this way, you win the game.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Dragon)) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Indestructible); }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Dragon)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let mut rows_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new(); for color in Color::ALL { let eligible: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.chars(o).colors.has(color)).collect(); if let Some(object) = cantrip_mechanics::choose::card(g, asker.controller, asker.source, Purpose::PutCounters(CounterKind::PlusOne, 1), eligible)? { rows_0.push((object, CounterKind::PlusOne, 1)); } } let mut before_0: Vec<(ObjectId, i16)> = rows_0.iter().map(|row| (row.0, g.state().obj(row.0).counters(CounterKind::PlusOne))).collect(); before_0.sort(); before_0.dedup(); g.add_counters_together(&rows_0); let placed_0: Vec<ObjectId> = before_0.iter().filter(|row| g.state().obj(row.0).counters(CounterKind::PlusOne) > row.1).map(|row| row.0).collect();
g.end_sentence();
if { let q: &dyn Query = g; ((placed_0.clone()).len() as i64) == (5i64) } {
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.win_game(player); if g.state().is_over() { return Ok(()); }; }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Call the Spirit Dragons", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 1, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, for each color, put a +1/+1 counter on a Dragon you control of that color. If you put +1/+1 counters on five Dragons this way, you win the game.").targets(&[])], statics: &[static_ability(Layer::Ability, ability_0_affects, ability_0_apply_0, "Dragons you control have indestructible.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
