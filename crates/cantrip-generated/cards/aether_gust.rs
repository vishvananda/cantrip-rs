// Aether Gust
// status: ready
// Choose target spell or permanent that's red or green. Its owner puts it on their choice of the top or bottom of their library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && ((q.chars(id).colors.has(q.words(asker.source).color(Color::Red)) || q.chars(id).colors.has(q.words(asker.source).color(Color::Green))))) }
#[allow(unused_variables)] fn ability_0_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).colors.has(q.words(asker.source).color(Color::Red)) || q.chars(id).colors.has(q.words(asker.source).color(Color::Green))))) }
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { ability_0_mode_0_target_0(q, asker, t) || ability_0_mode_1_target_0(q, asker, t) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_0 { cantrip_mechanics::library::owner_puts_on_top_or_bottom(g, asker.source, o)?; moved.extend(g.state().obj(o).became); } moved };
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_1 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_1 { cantrip_mechanics::library::owner_puts_on_top_or_bottom(g, asker.source, o)?; moved.extend(g.state().obj(o).became); } moved };
g.end_sentence();
 Ok(()) }
fn ability_0_implied(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); let chosen = (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == 0).map(|i| g.state().target(item, i)).next().unwrap_or(Target::None);
if ability_0_mode_0_target_0(g, asker, chosen) { return ability_0_mode_0(g, item); }
if ability_0_mode_1_target_0(g, asker, chosen) { return ability_0_mode_1(g, item); }
ability_0_mode_1(g, item) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Aether Gust", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0_implied), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
