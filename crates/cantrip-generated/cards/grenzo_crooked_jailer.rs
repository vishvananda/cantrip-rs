// Grenzo, Crooked Jailer
// status: planned
// requires: operation TowardPlayer
// When Grenzo enters and at the beginning of your upkeep, heist target opponent's library.
// Once each turn, you may pay {0} rather than pay the mana cost for a spell you cast that you don't own with mana value 3 or less.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("operation-toward-player" => cantrip_mechanics::heist::heist(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }, asker.source)?);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId, player: PlayerId) -> bool { player == asker.controller && (({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.state().obj(spell).owner) && (i64::from(q.mana_value(spell))) <= (3i64)) && !cantrip_mechanics::history::permission_used_this_turn(q, asker.source) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Grenzo, Crooked Jailer", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Goblin,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "When Grenzo enters and at the beginning of your upkeep, heist target opponent's library.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_alternatives: &[SpellAlternative { text: "Pay {0}", applies: ability_1_0_rule, cost: InsteadOfManaCost::Mana(ManaCost::ZERO), flash: false }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
