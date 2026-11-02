// What Must Be Done
// status: ready
// Choose one —
// • Let the World Burn — Destroy all artifacts and creatures.
// • Release Juno — Return target historic permanent card from your graveyard to the battlefield. It enters with two additional +1/+1 counters on it if it's a creature. (Artifacts, legendaries, and Sagas are historic.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((((q.chars(id).is(CardType::Artifact) || q.chars(id).supertypes.legendary || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker))) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_0_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_1_target_0)];
#[allow(unused_variables)] fn arrival_2_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { matches!(proposal, Proposal::ZoneChange(z) if z.object == affected && z.to.is(ZoneKind::Battlefield)) && q.chars(affected).is(CardType::Creature) }
#[allow(unused_variables)] fn arrival_2_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { if let Proposal::ZoneChange(z) = proposal { z.counters[CounterKind::PlusOne as usize] += (2i64).max(0) as i16; } }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_1 { let controller = player_1; g.add_effect(item.object, o, EffectKind::Replacement { card: item.ability.card, index: 0 }, Duration::EndOfTurn); moved.extend(g.put_all_onto_battlefield_as(&[(o, controller, false)])?.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield))); } moved };
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Let the World Burn — Destroy all artifacts and creatures.").targets(&[]),mode(ability_0_mode_1, "Release Juno — Return target historic permanent card from your graveyard to the battlefield. It enters with two additional +1/+1 counters on it if it's a creature.").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1);
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "What Must Be Done", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::ZoneChange, arrival_2_applies, arrival_2_apply, "enters with counters").once().text("Release Juno — Return target historic permanent card from your graveyard to the battlefield. It enters with two additional +1/+1 counters on it if it's a creature.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
