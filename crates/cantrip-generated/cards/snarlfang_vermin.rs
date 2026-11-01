// Snarlfang Vermin
// status: planned
// requires: engine: no perpetual effects
// Whenever this creature deals combat damage to a creature, if that creature is still on the battlefield, suspect that creature. It perpetually gains this ability.
// Whenever a suspected creature an opponent controls dies while this card is in your graveyard, that opponent loses 1 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Object(o) => o.0, _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Object(o) if q.chars(o).is(CardType::Creature)) && Some(ObjectId(data[1])).filter(|o| *o != ObjectId::NONE).is_some_and(|candidate: ObjectId| (q.state().obj(candidate).became.is_none() && q.state().obj(candidate).zone.kind() == ZoneKind::Battlefield)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; Some(ObjectId(item.data[1])).filter(|o| *o != ObjectId::NONE).is_some_and(|candidate: ObjectId| (q.state().obj(candidate).became.is_none() && q.state().obj(candidate).zone.kind() == ZoneKind::Battlefield)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
for bound_4294967295 in { let q: &dyn Query = g; Some(ObjectId(item.data[1])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>() } {
if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967295).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.suspect(object); }
}
g.end_sentence();
let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; Some(ObjectId(item.data[1])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>() };
planned!("engine-no-perpetual-effects" => for &object in &objects_2 { cantrip_mechanics::perpetual::apply(g, asker.source, object, PerpetualEffect::AddAbilityVariable(AbilityVariable::ThisAbility)); });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && ((q.state().effects.iter().any(|e| e.affected == id && e.kind == EffectKind::Suspected) && q.chars(id).is(CardType::Creature)) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))) && Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().obj(candidate).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(candidate).owner))) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_3 = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.lose_life(player, amount_3); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Snarlfang Vermin", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_0_event), ability_0, "Whenever this creature deals combat damage to a creature, if that creature is still on the battlefield, suspect that creature. It perpetually gains this ability.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever a suspected creature an opponent controls dies while this card is in your graveyard, that opponent loses 1 life.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Graveyard]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
