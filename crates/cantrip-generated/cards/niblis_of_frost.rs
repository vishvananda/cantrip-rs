// Niblis of Frost
// status: planned
// requires: continuous rule UntapStep
// requires: duration DuringControllersNextUntapStep
// Flying
// Prowess (Whenever you cast a noncreature spell, this creature gets +1/+1 until end of turn.)
// Whenever you cast an instant or sorcery spell, tap target creature an opponent controls. That creature doesn't untap during its controller's next untap step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_0: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_0);
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = planned_value!("duration DuringControllersNextUntapStep", Duration::Indefinite => Duration::DuringItsControllersNextUntapStep);
let kinds_1 = { let mut all = Vec::new(); for &object in &objects_1 { let kind = { let q: &dyn Query = g; planned_value!("continuous rule UntapStep", EffectKind::CantBlock => EffectKind::DoesntUntap) }; all.push((object, kind));  } all }; if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &(object, kind) in &kinds_1 { g.add_effect(source, object, kind, duration_1); } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Niblis of Frost", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::PROWESS,triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_event), ability_2, "Whenever you cast an instant or sorcery spell, tap target creature an opponent controls. That creature doesn't untap during its controller's next untap step.").targets(ABILITY_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
