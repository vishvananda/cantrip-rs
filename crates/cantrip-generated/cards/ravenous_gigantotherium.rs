// Ravenous Gigantotherium
// status: planned
// requires: engine: no Devour
// requires: target with a divided amount read as chosen
// Devour 3 (As this creature enters, you may sacrifice any number of creatures. It enters with three times that many +1/+1 counters on it.)
// When this creature enters, it deals X damage divided as you choose among up to X target creatures, where X is its power. Each of those creatures deals damage equal to its power to this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_count(q: &dyn Query, asker: &Asker) -> u16 { (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as u16 }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, TargetSpec::COUNTED)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
cantrip_mechanics::damage_divided(g, item);
g.end_sentence();
let sources_1 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let mut batch_1: Vec<(ObjectId, Target, u16)> = Vec::new(); for &source in &sources_1 { let amount = { let q: &dyn Query = g; (Some(source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as u16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { batch_1.push((source, t, amount)); } } let dealt_1 = g.damage_together(&batch_1); let done_1: i64 = dealt_1.iter().map(|&n| i64::from(n)).sum();
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-devour" => enters: cantrip_mechanics::devour::enters::<3>);

pub static CARD: CardDef = CardDef { name: "Ravenous Gigantotherium", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, it deals X damage divided as you choose among up to X target creatures, where X is its power. Each of those creatures deals damage equal to its power to this creature.").targets(ABILITY_1_TARGETS).divide(Divide { total: planned_value!("target with a divided amount read as chosen", 1u32 => Divide::read(|q: &dyn Query, asker: &Asker| (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as u32)), of_x: None }).counted_by(ability_1_0_count)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
