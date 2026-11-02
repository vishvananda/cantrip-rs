// The Skullspore Nexus
// status: ready
// This spell costs {X} less to cast, where X is the greatest power among creatures you control.
// Whenever one or more nontoken creatures you control die, create a green Fungus Dinosaur creature token with base power and toughness each equal to the total power of those creatures.
// {2}, {T}: Double target creature's power until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_cost(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, player); if costed == Costed::Spell(source) && true { CostDelta::less(((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))).max().unwrap_or(0)).clamp(0, 255) as u8) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Fungus Dinosaur).name, token_words: token!(Fungus Dinosaur).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fungus,Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let power_0 = { let q: &dyn Query = g; ((q.event_objects(item).to_vec()).into_iter().map(|o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))).sum::<i64>()).clamp(0, 32767) as i16 }; let toughness_0 = { let q: &dyn Query = g; ((q.event_objects(item).to_vec()).into_iter().map(|o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))).sum::<i64>()).clamp(0, 32767) as i16 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut values_0 = Copiable::of(g.card_id(&token_1::CARD)).sized(power_0, toughness_0); values_0.words = g.words(asker.source); let mut created_0: Vec<ObjectId> = Vec::new(); for player in players_0 { created_0.extend(g.create_token_copies_tapped(values_0, player, count_0, false)?); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_2 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kinds_2 = { let mut all = Vec::new(); for &object in &objects_2 { let kind = { let q: &dyn Query = g; EffectKind::Pump((Some(object).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))) as i16, (0i64) as i16) }; all.push((object, kind));  } all }; if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &(object, kind) in &kinds_2 { g.add_effect(source, object, kind, duration_2); } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Skullspore Nexus", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever one or more nontoken creatures you control die, create a green Fungus Dinosaur creature token with base power and toughness each equal to the total power of those creatures.").targets(&[]).once_per_batch(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_2, "{2}, {T}: Double target creature's power until end of turn.").targets(ABILITY_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, own_cost_changes: &[ability_0_cost], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
