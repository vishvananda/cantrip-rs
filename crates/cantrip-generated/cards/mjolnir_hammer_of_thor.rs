// Mjölnir, Hammer of Thor
// status: ready
// When Mjölnir enters, it deals 4 damage to up to one target creature.
// Double all damage equipped creature would deal.
// Equip worthy {1} (A creature is worthy if it's a legendary non-Villain that's red and/or white.)
// {2}{R}, Discard this card: It deals 2 damage to each creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, 1)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_0 = { let q: &dyn Query = g; item.subject.object() }; let amount_0 = { let q: &dyn Query = g; (4i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Damage(e) = proposal { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to) == Some(source) && true { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Damage(seen) = &seen { let e = seen; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if let Proposal::Damage(e) = proposal { e.amount = ((2i64 * i64::from(data[0]))).clamp(0, 65535) as u16; } } }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (((q.chars(id).is(CardType::Creature) && (q.chars(id).supertypes.legendary && !q.chars(id).has_subtype(Subtype::Villain) && (q.chars(id).colors.has(Color::Red) || q.chars(id).colors.has(Color::White)))) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let attachments_1 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; if let Some(host) = host_1 { cantrip_api::instruction(g, |g| { for attachment in attachments_1 { g.attach(attachment, host); } }); }
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let amount_2 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_2 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_2 { let batch: Vec<(ObjectId, Target, u16)> = recipients_2.iter().map(|t| (source, *t, amount_2)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Mjölnir, Hammer of Thor", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Equipment]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Mjölnir enters, it deals 4 damage to up to one target creature.").targets(ABILITY_0_TARGETS),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "Equip worthy {1}").targets(ABILITY_2_TARGETS).equip().sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(DISCARD_THIS), ability_3, "{2}{R}, Discard this card: It deals 2 damage to each creature.").targets(&[]).from(ZoneKind::Hand)], statics: &[], replacements: &[replacement(ReplacementKind::Damage, ability_1_applies, ability_1_apply, "Double all damage equipped creature would deal.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
