// Arcbond
// status: planned
// requires: event trigger carrying values
// Choose target creature. Whenever that creature is dealt damage this turn, it deals that much damage to each other creature and each player.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn watch_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = *target; let data = [u32::from(amount), match target { Target::Object(o) => o.0, _ => u32::MAX }]; if true && true && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) { return Some(Fired { subject, data }); } }
 None }
fn watch_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_2 = { let q: &dyn Query = g; about(q, item, 0).first().copied() }; let amount_2 = { let q: &dyn Query = g; (i64::from(item.data[0])).clamp(0, 65535) as u16 }; let recipients_2 = { let q: &dyn Query = g; { let mut all: Vec<Target> = Vec::new(); for t in (q.state().battlefield().iter().copied().filter(|&id| !(about(q, item, 0).first().copied() == Some(id))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() { if !all.contains(&t) { all.push(t); } } for t in (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() { if !all.contains(&t) { all.push(t); } } all } }; if let Some(source) = source_2 { let batch: Vec<(ObjectId, Target, u16)> = recipients_2.iter().map(|t| (source, *t, amount_2)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("event-trigger-carrying-values" => if let Some(source) = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) } { let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE);  g.delayed_trigger_about(DelayedWhen::ThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); });
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Arcbond", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), watch_1_event), watch_1, "Choose target creature. Whenever that creature is dealt damage this turn, it deals that much damage to each other creature and each player.").targets(&[]).sums_per_subject()], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
