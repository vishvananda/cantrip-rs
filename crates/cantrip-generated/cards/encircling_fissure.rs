// Encircling Fissure
// status: planned
// requires: engine: no Awaken
// requires: target reference outside a resolving ability
// Prevent all combat damage that would be dealt this turn by creatures target opponent controls.
// Awaken 2—{4}{W} (If you cast this spell for {4}{W}, also put two +1/+1 counters on target land you control and it becomes a 0/0 Elemental creature with haste. It's still a land.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_awakened_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
#[allow(unused_variables)] fn ability_0_awakened_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_AWAKENED_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_awakened_target_0),target(ability_0_awakened_target_1)];
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).is(CardType::Creature) && planned_value!("target reference outside a resolving ability", None::<PlayerId> => q.effect_about(asker, 0).first().and_then(|t: &Target| t.player())) == Some(q.controller(source))) && true { return true; } }
 false }
fn ability_0_awakened(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
planned!("target-reference-outside-a-resolving-ability" => {  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); let mut carried: Vec<Target> = Vec::new(); carried.extend({ let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).map(Target::Player).into_iter().collect::<Vec<Target>>() }); carried.push(Target::None);  if let (Some(to), Some(about)) = (to, about) { g.add_effect_about(&carried, item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } });
g.end_sentence();
planned!("engine-no-awaken" => if let Some(object) = ({ let q: &dyn Query = g; (t_1_raw.get(0).copied().filter(|t| t_1.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { let amount = { let q: &dyn Query = g; (2i64).max(0) as i16 }; cantrip_mechanics::awaken::awaken(g, asker.source, object, amount)?; });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_unpaid_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_UNPAID_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_unpaid_target_0)];
#[allow(unused_variables)] fn made_4_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).is(CardType::Creature) && planned_value!("target reference outside a resolving ability", None::<PlayerId> => q.effect_about(asker, 0).first().and_then(|t: &Target| t.player())) == Some(q.controller(source))) && true { return true; } }
 false }
fn ability_0_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("target-reference-outside-a-resolving-ability" => {  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); let mut carried: Vec<Target> = Vec::new(); carried.extend({ let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).map(Target::Player).into_iter().collect::<Vec<Target>>() }); carried.push(Target::None);  if let (Some(to), Some(about)) = (to, about) { g.add_effect_about(&carried, item.object, about, EffectKind::Prevention { card: item.ability.card, index: 1, to, shield: Shield::All }, Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-awaken" => ways: cantrip_mechanics::awaken::way(ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, ABILITY_0_AWAKENED_TARGETS, ability_0_awakened, "Awaken"));

pub static CARD: CardDef = CardDef { name: "Encircling Fissure", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_UNPAID_TARGETS, ability_0_unpaid), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "", damage: "damage", of_it: None, applies: made_1_applies },GrantedPreventionDef { hint: "", damage: "damage", of_it: None, applies: made_4_applies }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
