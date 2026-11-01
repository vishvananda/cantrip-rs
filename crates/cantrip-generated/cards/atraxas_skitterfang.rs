// Atraxa's Skitterfang
// status: ready
// This creature enters with three oil counters on it.
// At the beginning of combat on your turn, you may remove an oil counter from this creature. When you do, target creature you control gains your choice of flying, vigilance, deathtouch, or lifelink until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (3i64).max(0) as i16 }; entering.counters[CounterKind::Oil as usize] = entering.counters[CounterKind::Oil as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static REFLEXIVE_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_4_target_0)];
fn reflexive_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let chosen_5 = cantrip_mechanics::choose::option(g, player_5, asker.source, item.ability.card, 0..4)? - 0;
g.end_sentence();
let objects_6 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_6 = { let q: &dyn Query = g; Some(asker.source) }; let duration_6 = Duration::EndOfTurn;
let kind_6_0_0 = { let q: &dyn Query = g; match chosen_5 { 0 => EffectKind::AddKeywords(Keywords::of(&[Keyword::Flying])), 1 => EffectKind::AddKeywords(Keywords::of(&[Keyword::Vigilance])), 2 => EffectKind::AddKeywords(Keywords::of(&[Keyword::Deathtouch])), _ => EffectKind::AddKeywords(Keywords::of(&[Keyword::Lifelink])), } };
if let Some(source) = source_6 { cantrip_api::instruction(g, |g| { for &object in &objects_6 { g.add_effect(source, object, kind_6_0_0, duration_6);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
let mut removed_2: i64 = 0;
let objects_2_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_2_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_2_0 { removed_2 += i64::from(g.remove_counters(object, CounterKind::Oil, amount_2_0)); } });
if removed_2 > 0 && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Atraxa's Skitterfang", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Insect]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of combat on your turn, you may remove an oil counter from this creature. When you do, target creature you control gains your choice of flying, vigilance, deathtouch, or lifelink until end of turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_4, "At the beginning of combat on your turn, you may remove an oil counter from this creature. When you do, target creature you control gains your choice of flying, vigilance, deathtouch, or lifelink until end of turn.").targets(REFLEXIVE_4_TARGETS)], enters: Some(ability_0_enters), modal: None, enchant: None, labels: &["flying", "vigilance", "deathtouch", "lifelink"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
