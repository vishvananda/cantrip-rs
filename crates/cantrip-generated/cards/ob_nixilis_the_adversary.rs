// Ob Nixilis, the Adversary
// status: planned
// requires: engine: no CasualtyX
// Casualty X. The copy isn't legendary and has starting loyalty X. (As you cast this spell, you may sacrifice a creature with power X. When you do, copy this spell. The copy becomes a token.)
// [+1]: Each opponent loses 2 life unless they discard a card. If you control a Demon or Devil, you gain 2 life.
// [−2]: Create a 1/1 red Devil creature token with "When this token dies, it deals 1 damage to any target."
// [−7]: Target player draws seven cards and loses 7 life.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut guard_0_0 = vec![false; players_0.len()];
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
let mut each_6 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let payer_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let (paid_1, after_1, before_1, picked_1): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_3 = Vec::new(); for player in players_3 { cards_3.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_3, true)? }); } let discarded_3 = g.discard_together(&cards_3);
let paid_2 = !discarded_3.clone().is_empty();
(paid_2, { let q: &dyn Query = g; (discarded_3).clone() }, { let q: &dyn Query = g; (discarded_3.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
 each_4[index_0] = after_1.clone(); all_4.extend(each_4[index_0].iter().copied());
each_5[index_0] = { let q: &dyn Query = g; before_1.clone() }; all_5.extend(each_5[index_0].iter().copied());
each_6[index_0] = paid_1;
   } 
g.end_sentence();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_6[index_0] as i64) != 0)) } { guard_0_0[index_0] = true;
let amount_7 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_7 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_7 { g.lose_life(player, amount_7); }
   }
 } 
g.end_sentence();
if { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Demon)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Devil)))).count() >= 1 } {
let amount_8 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_8 { g.gain_life(player, amount_8); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
pub mod token_10 {
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_0 = { let q: &dyn Query = g; item.subject.object() }; let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: token!(Devil).name, token_words: token!(Devil).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Devil]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(8)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this token dies, it deals 1 damage to any target.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_9 = Vec::new();
let count_9_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_9_0 = g.card_id(&token_10::CARD);
for player in players_9 {
tokens_9.extend(std::iter::repeat((def_9_0,player)).take(count_9_0 as usize));
} let words_9 = g.words(asker.source);
let created_9 = g.create_tokens_worded(&tokens_9, false, None, words_9)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_3_target_0)];
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_11 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let players_11 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
for player in players_11 { g.draw(player, amount_11); }
g.end_sentence();
let amount_12 = { let q: &dyn Query = g; (7i64).max(0) as i32 }; let players_12 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
for player in players_12 { g.lose_life(player, amount_12); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-casualty-x" => optional_cost: Some(cantrip_mechanics::casualty::optional_x()));

pub static CARD: CardDef = CardDef { name: "Ob Nixilis, the Adversary", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Nixilis]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(3), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_1, "+1: Each opponent loses 2 life unless they discard a card. If you control a Demon or Devil, you gain 2 life.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-2)), ability_2, "−2: Create a 1/1 red Devil creature token with \"When this token dies, it deals 1 damage to any target.\"").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-7)), ability_3, "−7: Target player draws seven cards and loses 7 life.").targets(ABILITY_3_TARGETS).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_10::CARD];
