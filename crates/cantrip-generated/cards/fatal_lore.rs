// Fatal Lore
// status: planned
// requires: modes chosen by another player
// requires: player reference ModeChooser
// An opponent chooses one —
// • You draw three cards.
// • You destroy up to two target creatures that player controls. They can't be regenerated. That player draws up to three cards.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && planned_value!("player reference ModeChooser", None::<PlayerId> => q.mode_chooser(asker)) == Some(q.controller(id))))) }
static ABILITY_0_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_1_target_0).between(0, 2)];
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_1 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let destroyed_1 = g.destroy_all_no_regen(&objects_1);
g.end_sentence();
let most_2 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mut done_2: i64 = 0; for player in { let q: &dyn Query = g; planned_value!("player reference ModeChooser", None::<PlayerId> => q.mode_chooser(asker)).into_iter().collect::<Vec<PlayerId>>() } { let n = cantrip_mechanics::choose::number(g, player, asker.source, 0, most_2, "Draw")?; let before = g.history().drawn(player) as i64; g.draw(player, n); done_2 += g.history().drawn(player) as i64 - before; }
g.end_sentence();
 Ok(()) }
planned_item!("modes-chosen-by-another-player" => #[allow(unused_variables)] fn mode_choosers_0(q: &dyn Query, asker: &Asker) -> Vec<PlayerId> { Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() });
static ABILITY_0_MODES: ModalDef = planned_value!("modes chosen by another player", modes(&[mode(ability_0_mode_0, "You draw three cards.").targets(&[]),mode(ability_0_mode_1, "You destroy up to two target creatures that player controls. They can't be regenerated. That player draws up to three cards.").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1) => modes(&[mode(ability_0_mode_0, "You draw three cards.").targets(&[]),mode(ability_0_mode_1, "You destroy up to two target creatures that player controls. They can't be regenerated. That player draws up to three cards.").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1).chosen_by(mode_choosers_0));
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Fatal Lore", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
