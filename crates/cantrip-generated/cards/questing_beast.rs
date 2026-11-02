// Questing Beast
// status: ready
// Vigilance, deathtouch, haste
// Questing Beast can't be blocked by creatures with power 2 or less.
// Combat damage that would be dealt by creatures you control can't be prevented.
// Whenever Questing Beast deals combat damage to an opponent, it deals that much damage to target planeswalker that player controls.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(attacker == asker.source && (q.chars(blocker).is(CardType::Creature) && q.chars(blocker).power.is_some_and(|v| (i64::from(v)) <= (2i64)))) }
#[allow(unused_variables)] fn ability_4_unpreventable(q: &dyn Query, asker: &Asker, source: ObjectId, combat: bool) -> bool { combat && q.is_on_battlefield(source) && (q.chars(source).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(source))) }
#[allow(unused_variables)] fn ability_5_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Planeswalker) && q.event_subject().player() == Some(q.controller(id))))) }
static ABILITY_5_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_5_target_0)];
#[allow(unused_variables)] fn ability_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = *target; let data = [u32::from(amount), source.0]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) { return Some(Fired { subject, data }); } }
 None }
fn ability_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_0 = { let q: &dyn Query = g; Some(ObjectId(item.data[1])).filter(|o| *o != ObjectId::NONE) }; let amount_0 = { let q: &dyn Query = g; (i64::from(item.data[0])).min(u16::MAX.into()) as u16 }; let recipients_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Questing Beast", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance,Keyword::Deathtouch,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_5_event), ability_5, "Whenever Questing Beast deals combat damage to an opponent, it deals that much damage to target planeswalker that player controls.").targets(ABILITY_5_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_3_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, unpreventable_damage: &[ability_4_unpreventable], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
