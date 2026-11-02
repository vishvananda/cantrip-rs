// The Phasing of Zhalfir
// status: planned
// requires: operation GameEffect
// Read ahead (Choose a chapter and start with that many lore counters. Add one after your draw step. Skipped chapters don't trigger. Sacrifice after III.)
// I, II — Another target nonland permanent phases out. It can't phase in for as long as you control this Saga.
// III — Destroy all creatures. For each creature destroyed this way, its controller creates a 2/2 black Phyrexian creature token.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(Some(asker.source) == Some(id)) && !(q.chars(id).is(CardType::Land))))) }
static ABILITY_1_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_0_target_0)];
#[allow(unused_variables)] fn lasting_0(q: &dyn Query, e: &ContinuousEffect) -> bool { let asker = &Asker::new(e.source, e.controller); is_live(q, e.source) && q.is_on_battlefield(e.source) && q.controller(e.source) == e.controller }
fn ability_1_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if let Some(object) = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.phase_out(&[object]); }
g.end_sentence();
planned!("operation-game-effect" => cantrip_mechanics::game_rules::for_a_time(g, asker, { let q: &dyn Query = g; Duration::AsLongAs({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) }, GameEffect::PermanentCantPhaseIn({ let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) })));
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(Some(asker.source) == Some(id)) && !(q.chars(id).is(CardType::Land))))) }
static ABILITY_1_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_1_target_0)];
#[allow(unused_variables)] fn lasting_1(q: &dyn Query, e: &ContinuousEffect) -> bool { let asker = &Asker::new(e.source, e.controller); is_live(q, e.source) && q.is_on_battlefield(e.source) && q.controller(e.source) == e.controller }
fn ability_1_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if let Some(object) = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.phase_out(&[object]); }
g.end_sentence();
planned!("operation-game-effect" => cantrip_mechanics::game_rules::for_a_time(g, asker, { let q: &dyn Query = g; Duration::AsLongAs({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 1) }, GameEffect::PermanentCantPhaseIn({ let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) })));
g.end_sentence();
 Ok(()) }
pub mod token_6 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Phyrexian).name, token_words: token!(Phyrexian).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let objects_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let destroyed_4 = g.destroy_all(&objects_4);
g.end_sentence();
for bound_0 in { let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() } {
let players_5 = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_5 = Vec::new();
let count_5_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_5_0 = g.card_id(&token_6::CARD);
for player in players_5 {
tokens_5.extend(std::iter::repeat((def_5_0,player)).take(count_5_0 as usize));
} let words_5 = g.words(asker.source);
let created_5 = g.create_tokens_worded(&tokens_5, false, None, words_5)?;
g.end_sentence();
hoisted_7.extend(created_5.clone());
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Phasing of Zhalfir", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_1_0, "I, II — Another target nonland permanent phases out. It can't phase in for as long as you control this Saga.").targets(ABILITY_1_0_TARGETS),chapter(2, ability_1_1, "I, II — Another target nonland permanent phases out. It can't phase in for as long as you control this Saga.").targets(ABILITY_1_1_TARGETS),chapter(3, ability_1_2, "III — Destroy all creatures. For each creature destroyed this way, its controller creates a 2/2 black Phyrexian creature token.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, lasting: &[Lasting { text: "you control {source}", holds: lasting_0 },Lasting { text: "you control {source}", holds: lasting_1 }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_6::CARD];
