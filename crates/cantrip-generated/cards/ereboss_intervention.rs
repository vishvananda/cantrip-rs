// Erebos's Intervention
// status: ready
// Choose one —
// • Target creature gets -X/-X until end of turn. You gain X life.
// • Exile up to twice X target cards from graveyards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::Pump(((-1i64 * i64::from(item.x))).clamp(-32768, 32767) as i16, ((-1i64 * i64::from(item.x))).clamp(-32768, 32767) as i16) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_mode_1_0_up_to_most(q: &dyn Query, source: ObjectId, chooser: PlayerId, x: u32) -> usize { let asker = &Asker::new(source, chooser); ((2i64 * i64::from(x))).clamp(0, 4294967295) as u32 as usize }
fn ability_0_mode_1_0_up_to(q: &dyn Query, choice: &Choice, x: u32) -> Verdict { let most = ability_0_mode_1_0_up_to_most(q, choice.source, choice.chooser, x); Verdict { complete: choice.selected.len() <= most, addable: Bits::new(choice.candidates.len(), choice.selected.len() < most) } }
fn largest_ability_0_mode_1_0_up_to(q: &dyn Query, choice: &Choice, x: u32) -> Vec<ObjectId> { choice.candidates.iter().copied().take(ability_0_mode_1_0_up_to_most(q, choice.source, choice.chooser, x)).collect() }
#[allow(unused_variables)] fn ability_0_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (true)) }
static ABILITY_0_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_1_target_0).between(0, cantrip_api::TargetSpec::ANY_NUMBER).together(cantrip_api::select::set_part(&(ability_0_mode_1_0_up_to as SelectFn), &(largest_ability_0_mode_1_0_up_to as cantrip_api::select::LargestFn), TargetSpec::COUNTED, "up to X"))];
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_2 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Target creature gets -X/-X until end of turn. You gain X life.").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, "Exile up to twice X target cards from graveyards.").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1);
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Erebos's Intervention", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
