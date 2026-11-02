// Ravenous Tyrannosaurus
// status: planned
// requires: engine: no Devour
// Devour 3 (As this creature enters, you may sacrifice any number of creatures. It enters with three times that many +1/+1 counters on it.)
// Whenever this creature attacks, it deals damage equal to its power to up to one other target creature. Excess damage is dealt to that creature's controller instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.event_subject().object() == Some(id)) && q.chars(id).is(CardType::Creature)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 1)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if let Some(present_2_0) = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)) } { let source_1 = { let q: &dyn Query = g; item.subject.object() }; let targets_1 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; let amount_1: u16 = { let q: &dyn Query = g; (item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as u16 }; let player_1 = { let q: &dyn Query = g; present_2_0 }; if let (Some(source), Some(&target)) = (source_1, targets_1.first()) { let lethal_1 = { let q: &dyn Query = g; target.object().map_or(amount_1, |o: ObjectId| { let chars = q.chars(o); let left = u16::try_from(chars.toughness.unwrap_or(0).max(0)).unwrap_or(0).saturating_sub(q.state().obj(o).damage); match q.chars(source).has(Keyword::Deathtouch) { true => left.min(1), false => left } }) }; let dealt_1 = amount_1.min(lethal_1); g.damage_together(&[(source, target, dealt_1), (source, Target::Player(player_1), amount_1 - dealt_1)]); }
 }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-devour" => enters: cantrip_mechanics::devour::enters::<3>);

pub static CARD: CardDef = CardDef { name: "Ravenous Tyrannosaurus", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, it deals damage equal to its power to up to one other target creature. Excess damage is dealt to that creature's controller instead.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
