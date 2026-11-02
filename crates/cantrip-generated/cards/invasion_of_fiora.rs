// Invasion of Fiora // Marchesa, Resolute Monarch
// status: ready
// Menace, deathtouch
// Whenever Marchesa attacks, remove all counters from up to one target permanent.
// At the beginning of your upkeep, if you haven't been dealt combat damage since your last turn, you draw a card and you lose 1 life.
// (As a Siege enters, choose an opponent to protect it. You and others can attack it. When it's defeated, exile it, then cast it transformed.)
// When this Siege enters, choose one or both —
// • Destroy all legendary creatures.
// • Destroy all nonlegendary creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).supertypes.legendary && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).supertypes.legendary) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let destroyed_1 = g.destroy_all(&objects_1);
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Destroy all legendary creatures.").targets(&[]),mode(ability_0_mode_1, "Destroy all nonlegendary creatures.").targets(&[])]).choose(1, 2);

pub static CARD: CardDef = CardDef { name: "Invasion of Fiora", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Battle]), subtypes: SubtypeSet::of(&[Subtype::Siege]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: Some(4), keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), &ABILITY_0_MODES, "When this Siege enters, choose one or both — Destroy all legendary creatures; or Destroy all nonlegendary creatures.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0).between(0, 1)];
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut removed_0: i64 = 0;
let objects_0_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() };
cantrip_api::instruction(g, |g| { for object in objects_0_0 { cantrip_mechanics::effects::remove_all_counters(g, object); } });
g.end_sentence();
 Ok(()) }
fn ability_3_if(q: &dyn Query, asker: &Asker) -> bool { !({ let history = q.history(); history.all()[history.all().iter().rposition(|r| Some(r.active) == Some(asker.controller) && r.turn != history.turn).map_or(0, |at| at + 1)..].iter() }.any(|r| matches!(r.event, Event::Damage { target: Target::Player(hit), kind, .. } if Some(hit) == Some(asker.controller) && kind.is_combat()))) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.lose_life(player, amount_2); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Marchesa, Resolute Monarch", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Noble]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(4)), power: Some(3), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace,Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever Marchesa attacks, remove all counters from up to one target permanent.").targets(ABILITY_2_TARGETS),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_3_event), ability_3, "At the beginning of your upkeep, if you haven't been dealt combat damage since your last turn, you draw a card and you lose 1 life.").targets(&[]).condition(ability_3_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
