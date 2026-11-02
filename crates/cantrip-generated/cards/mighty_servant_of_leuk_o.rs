// Mighty Servant of Leuk-o
// status: planned
// requires: numeric object property CrewCount
// requires: trigger event Object Crewed
// Trample
// Ward—Discard a card.
// Whenever this Vehicle becomes crewed for the first time each turn, if it was crewed by exactly two creatures, it gains "Whenever this creature deals combat damage to a player, draw two cards" until end of turn.
// Crew 4
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
#[allow(unused_variables)] fn granted_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn granted_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_2 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| (planned_value!("numeric object property CrewCount", 0i64 => q.crewed_by(candidate).len() as i64)) == (2i64)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-object-crewed" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Crewed { vehicle, .. } = event { let id = *vehicle; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) && (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Crewed { vehicle, .. } = event { let id = *vehicle; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }  None }).count() as i64 + 1) == (1i64) && subject.object().is_some_and(|candidate: ObjectId| (planned_value!("numeric object property CrewCount", 0i64 => q.crewed_by(candidate).len() as i64)) == (2i64)) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Mighty Servant of Leuk-o", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward_discard("Ward—Discard a card."),triggered(when_asked(planned_value!("trigger event Object Crewed", EventMask(0) => EventMask(EventMask::CREWED.0)), ability_2_event), ability_2, "Whenever this Vehicle becomes crewed for the first time each turn, if it was crewed by exactly two creatures, it gains \"Whenever this creature deals combat damage to a player, draw two cards\" until end of turn.").targets(&[]).once_per_batch(),cantrip_mechanics::crew::crew(4, "Crew 4")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), granted_1_event), granted_1, "Whenever this creature deals combat damage to a player, draw two cards").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
