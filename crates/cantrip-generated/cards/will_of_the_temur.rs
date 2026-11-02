// Will of the Temur
// status: planned
// requires: engine: no commander
// Choose one. If you control a commander as you cast this spell, you may choose both instead.
// • Create a token that's a copy of target permanent, except it's a 4/4 Dragon creature with flying in addition to its other types.
// • Target player draws cards equal to the greatest mana value among permanents you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
static ABILITY_0_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let values_0 = { let q: &dyn Query = g; original_0.map(|original: ObjectId| q.copiable_values(original).sized(4, 4).with_subtype(q.words(asker.source).subtype(Subtype::Dragon)).with_type(CardType::Creature).with_keyword(Keyword::Flying)) }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_0 = match values_0 { Some(values) => g.create_token_copies_tapped(values, player_0, count_0, false)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_1_target_0)];
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_1 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).max().unwrap_or(0)).min(u32::MAX.into()) as u32 }; let players_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }
fn ability_0_more_modes(q: &dyn Query, asker: &Asker) -> bool { q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && planned_value!("engine: no commander", false => q.is_commander(id))).count() >= 1 }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Create a token that's a copy of target permanent, except it's a 4/4 Dragon creature with flying in addition to its other types.").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, "Target player draws cards equal to the greatest mana value among permanents you control.").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1).up_to_if(ability_0_more_modes, 2);
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Will of the Temur", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
