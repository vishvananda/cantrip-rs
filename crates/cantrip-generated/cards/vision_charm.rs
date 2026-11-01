// Vision Charm
// status: planned
// requires: engine: no resolving effect sets land types
// requires: object filter NthChosen
// requires: operation Together
// Choose one —
// • Target player mills four cards.
// • Choose a land type and a basic land type. Each land of the first chosen type becomes the second chosen type until end of turn.
// • Target artifact phases out. (While it's phased out, it's treated as though it doesn't exist. It phases in before its controller untaps during their next untap step.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_0 = { let q: &dyn Query = g; (4i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-together" => g.begin_simultaneous());
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let choices_2 = Subtype::ALL.into_iter().filter(|&kind| SubtypeSet::LAND_TYPES.has(kind)).collect::<Vec<Subtype>>(); let Answer::Subtype(chosen_2) = g.ask(player_2, RequestKind::ChooseSubtype { source: asker.source, choices: choices_2, purpose: WordUse::Chosen })? else { return Ok(()) };
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Subtype(chosen_3) = g.ask(player_3, RequestKind::ChooseSubtype { source: asker.source, choices: vec![Subtype::Plains, Subtype::Island, Subtype::Swamp, Subtype::Mountain, Subtype::Forest], purpose: WordUse::Chosen })? else { return Ok(()) };
planned!("operation-together" => g.end_simultaneous());
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && planned_value!("object filter NthChosen", false => q.nth_chosen_value(asker, 0).matches(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::EndOfTurn;
let kind_4_0_0 = { let q: &dyn Query = g; planned_value!("engine: no resolving effect sets land types", EffectKind::Pump(0, 0) => EffectKind::SetLandType(q.nth_chosen_value(asker, 1).subtype())) };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4);  } }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_mode_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Artifact))) }
static ABILITY_0_MODE_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_2_target_0)];
fn ability_0_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if let Some(object) = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.phase_out(&[object]); }
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Target player mills four cards.").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, "Choose a land type and a basic land type. Each land of the first chosen type becomes the second chosen type until end of turn.").targets(&[]),mode(ability_0_mode_2, "Target artifact phases out.").targets(ABILITY_0_MODE_2_TARGETS)]).choose(1, 1);
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Vision Charm", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
