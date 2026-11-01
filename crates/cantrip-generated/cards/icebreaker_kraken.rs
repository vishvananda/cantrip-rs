// Icebreaker Kraken
// status: planned
// requires: continuous rule UntapStep
// requires: duration DuringControllersNextUntapStep
// requires: engine: no resolving rule on an open set: UntapStep
// Affinity for snow lands (This spell costs {1} less to cast for each snow land you control.)
// When this creature enters, artifacts and creatures target opponent controls don't untap during that player's next untap step.
// Return three snow lands you control to their owner's hand: Return this creature to its owner's hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affinity(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { if costed != Costed::Spell(source) { return CostDelta::NONE; } let asker = &Asker::new(source, player); let count = q.permanents_of(player).into_iter().filter(|&id| (q.chars(id).supertypes.snow && q.chars(id).is(CardType::Land))).count(); CostDelta::less(count.min(255) as u8) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("engine-no-resolving-rule-on-an-open-set-untap-step" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { g.add_open_effect(source, move |q: &dyn Query, asker: &Asker, id: ObjectId| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)) && t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) == Some(q.controller(id))), planned_value!("continuous rule UntapStep", EffectKind::CantBlock => EffectKind::DoesntUntap), planned_value!("duration DuringControllersNextUntapStep", Duration::Indefinite => Duration::DuringItsControllersNextUntapStep)); });
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_1 = g.return_to_hand(&objects_1);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_2(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).supertypes.snow && q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id))) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Icebreaker Kraken", token_words: &[], oracle: true, cost: ManaCost { generic: 10, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Kraken]), supertypes: Supertypes { basic: false, legendary: false, snow: true }, colors: None, power: Some(8), toughness: Some(8), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, artifacts and creatures target opponent controls don't untap during that player's next untap step.").targets(ABILITY_1_TARGETS),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(return_to_hand(3, ability_2_2)), ability_2, "Return three snow lands you control to their owner's hand: Return this creature to its owner's hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, own_cost_changes: &[ability_0_affinity], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
