// Tranquil Frillback
// status: ready
// When this creature enters, you may pay {G} up to three times. When you pay this cost one or more times, choose up to that many —
// • Destroy target artifact or enchantment.
// • Exile target player's graveyard.
// • You gain 4 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_2_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Enchantment)))) }
static REFLEXIVE_2_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_2_mode_0_target_0)];
fn reflexive_2_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_3 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_3 = g.destroy_all(&objects_3);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn reflexive_2_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static REFLEXIVE_2_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_2_mode_1_target_0)];
fn reflexive_2_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_4 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
 Ok(()) }
fn reflexive_2_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_5 = { let q: &dyn Query = g; (4i64).max(0) as i32 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.gain_life(player, amount_5); }
g.end_sentence();
 Ok(()) }
static REFLEXIVE_2_MODES: ModalDef = modes(&[mode(reflexive_2_mode_0, "Destroy target artifact or enchantment.").targets(REFLEXIVE_2_MODE_0_TARGETS),mode(reflexive_2_mode_1, "Exile target player's graveyard.").targets(REFLEXIVE_2_MODE_1_TARGETS),mode(reflexive_2_mode_2, "You gain 4 life.").targets(&[])]).up_to_that_many();
fn reflexive_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let most_0: u16 = { let q: &dyn Query = g; (3i64).max(0) as u16 }; let bound_0: i64 = if most_0 == 0 { 0 } else { i64::from(g.may_pay_times(payer_0, asker.source, asker.source, ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, most_0, &PAY_QUESTION)?) }; let done_0: i64 = bound_0;
if done_0 > 0 && true { g.reflexive_trigger_counted(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None, (bound_0).clamp(0, i64::from(u16::MAX)) as u16); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Tranquil Frillback", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, you may pay {G} up to three times. When you pay this cost one or more times, choose up to that many — Destroy target artifact or enchantment; or Exile target player's graveyard; or You gain 4 life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_2, "When you pay this cost one or more times, choose up to that many —").targets(&[]).modal(&REFLEXIVE_2_MODES)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
