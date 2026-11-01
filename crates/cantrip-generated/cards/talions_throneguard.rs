// Talion's Throneguard
// status: planned
// requires: engine: no perpetual effects
// requires: modes implied by the target chosen
// Flash
// Bargain
// Flying
// When this creature enters, return up to one target spell or nonland permanent to its owner's hand. If this creature was bargained, that card perpetually gains "This spell costs {2} more to cast."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn ability_3_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (true)) }
static ABILITY_3_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_3_mode_0_target_0).between(0, 1)];
static PERPETUAL_3: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))), ..CardDef::BLANK };
fn ability_3_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).paid > 0) } {
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_3) };
for &object in &objects_2 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).is(CardType::Land)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker))))) }
static ABILITY_3_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_3_mode_1_target_0).between(0, 1)];
static PERPETUAL_7: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))), ..CardDef::BLANK };
fn ability_3_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_4 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_4 = g.return_to_hand(&objects_4);
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).paid > 0) } {
planned!("engine-no-perpetual-effects" => let objects_6: Vec<ObjectId> = { let q: &dyn Query = g; (moved_4.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
let kind_6_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_7) };
for &object in &objects_6 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_6_0); });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
static ABILITY_3_MODES: ModalDef = planned_value!("modes implied by the target chosen", modes(&[mode(ability_3_mode_0, "Return the chosen permanents to their owners' hands. ").targets(ABILITY_3_MODE_0_TARGETS),mode(ability_3_mode_1, "Return the chosen permanents to their owners' hands. ").targets(ABILITY_3_MODE_1_TARGETS)]).choose(1, 1) => modes(&[mode(ability_3_mode_0, "Return the chosen permanents to their owners' hands. ").targets(ABILITY_3_MODE_0_TARGETS),mode(ability_3_mode_1, "Return the chosen permanents to their owners' hands. ").targets(ABILITY_3_MODE_1_TARGETS)]).choose(1, 1).implied_by_target());
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Talion's Throneguard", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_3_event), &ABILITY_3_MODES, "When this creature enters, return up to one target spell or nonland permanent to its owner's hand. If this creature was bargained, that card perpetually gains \"This spell costs {2} more to cast.\"")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: cantrip_mechanics::bargain::LABELS, optional_cost: Some(cantrip_mechanics::bargain::OPTIONAL_COST), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
