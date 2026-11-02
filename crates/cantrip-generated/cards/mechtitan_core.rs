// Mechtitan Core
// status: planned
// requires: engine: a delayed ability on an event keeps no list
// requires: engine: no cost moves these objects
// requires: event trigger carrying values
// {5}, Exile this Vehicle and four other artifact creatures and/or Vehicles you control: Create Mechtitan, a legendary 10/10 Construct artifact creature token with flying, vigilance, trample, lifelink, and haste that's all colors. When that token leaves the battlefield, return all cards exiled with this Vehicle except this card to the battlefield tapped under their owners' control.
// Crew 2
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Mechtitan", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Construct]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(31)), power: Some(10), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance,Keyword::Trample,Keyword::Lifelink,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
#[allow(unused_variables)] fn delayed_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && (planned_value!("event trigger carrying values", Vec::<ObjectId>::new() => q.trigger_about(asker, 0))).contains(&id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = ({ let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.exiled_with(q.linked_to(asker)).contains(&id) && !(Some(asker.source) == Some(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let rows_4: Vec<(ObjectId, PlayerId, bool)> = objects_4.iter().map(|&o| (o, g.state().obj(o).owner, true)).collect(); let came_4 = g.put_all_onto_battlefield_as(&rows_4)?; came_4.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
planned!("engine-a-delayed-ability-on-an-event-keeps-no-list" => if let Some(source) = Some(item.object) { let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_0.clone() }); about.push(ObjectId::NONE);  g.delayed_trigger_about(DelayedWhen::UntilEvent, source, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_5(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(Some(asker.source) == Some(id)) && (((q.chars(id).is(CardType::Artifact) && q.chars(id).is(CardType::Creature)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Vehicle))) && Some(asker.controller) == Some(q.controller(id)))) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }

pub static CARD: CardDef = CardDef { name: "Mechtitan Core", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(EXILE_THIS.and(planned_value!("engine: no cost moves these objects", Cost::NONE => Cost { moves: Some(MovesObjects { from: CostZone::Battlefield, to: CostDestination::Exile, count: CostCount::Exactly(4), filter: ability_0_5 }), ..Cost::NONE }))), ability_0, "{5}, Exile this Vehicle and four other artifact creatures and/or Vehicles you control: Create Mechtitan, a legendary 10/10 Construct artifact creature token with flying, vigilance, trample, lifelink, and haste that's all colors. When that token leaves the battlefield, return all cards exiled with this Vehicle except this card to the battlefield tapped under their owners' control.").targets(&[]),cantrip_mechanics::crew::crew(2, "Crew 2")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), delayed_3_event), delayed_3, "When that token leaves the battlefield, return all cards exiled with this Vehicle except this card to the battlefield tapped under their owners' control.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
