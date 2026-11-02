// Gideon, Martial Paragon
// status: ready
// [+2]: Untap all creatures you control. Those creatures get +1/+1 until end of turn.
// [0]: Until end of turn, Gideon becomes a 5/5 Human Soldier creature with indestructible that's still a planeswalker. Prevent all damage that would be dealt to him this turn.
// [−10]: Creatures you control get +2/+2 until end of turn. Tap all creatures your opponents control.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_0: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_0 { g.untap(object); } });
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (objects_0.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::Pump((1i64) as i16, (1i64) as i16) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn made_4_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && true { return true; } }
 false }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::SetPt((5i64) as i16, (5i64) as i16) };
let kind_2_1_0 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Human)) };
let kind_2_2_0 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Soldier)) };
let kind_2_3_0 = { let q: &dyn Query = g; EffectKind::AddTypes(TypeSet::of(&[CardType::Creature])) };
let kind_2_4_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Indestructible])) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2); g.add_effect(source, object, kind_2_1_0, duration_2); g.add_effect(source, object, kind_2_2_0, duration_2); g.add_effect(source, object, kind_2_3_0, duration_2); g.add_effect(source, object, kind_2_4_0, duration_2);  } }); }
g.end_sentence();
{  let to = { let q: &dyn Query = g; ((Some(asker.source)).map(Target::Object).into_iter().collect::<Vec<Target>>()).first().copied() }; let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let duration_5 = Duration::EndOfTurn;
let kind_5_0_0 = { let q: &dyn Query = g; EffectKind::Pump((2i64) as i16, (2i64) as i16) };
if let Some(source) = source_5 { cantrip_api::instruction(g, |g| { for &object in &objects_5 { g.add_effect(source, object, kind_5_0_0, duration_5);  } }); }
g.end_sentence();
let objects_6 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_6: Vec<ObjectId> = objects_6.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_6);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Gideon, Martial Paragon", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Gideon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(5), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(2)), ability_0, "+2: Untap all creatures you control. Those creatures get +1/+1 until end of turn.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(0)), ability_1, "0: Until end of turn, Gideon becomes a 5/5 Human Soldier creature with indestructible that's still a planeswalker. Prevent all damage that would be dealt to him this turn.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-10)), ability_2, "−10: Creatures you control get +2/+2 until end of turn. Tap all creatures your opponents control.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "0: Until end of turn, Gideon becomes a 5/5 Human Soldier creature with indestructible that's still a planeswalker. Prevent all damage that would be dealt to him this turn.", damage: "damage", of_it: None, applies: made_4_applies, then: None, text: "Prevent all damage that would be dealt to him this turn." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
