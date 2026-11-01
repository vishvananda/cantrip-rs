// Triumph of Saint Katherine
// status: planned
// requires: cost part Move
// requires: objects that paid a cost at resolution
// requires: operation ChangeZone
// requires: resolution cost Move
// requires: static subjects Take
// Lifelink
// Praesidium Protectiva — When this creature is put into your graveyard from the battlefield, exile it and the top six cards of your library in a face-down pile. If you do, shuffle that pile and put it back on top of your library.
// Miracle {1}{W} (You may cast this card for its miracle cost when you draw it if it's the first card you drew this turn.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (Some(asker.source) == Some(id) && Some(asker.controller) == Some(q.state().obj(id).owner)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let paid_0 = planned_value!("resolution cost Move", false => cantrip_mechanics::effects::pay(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost part Move", Cost::NONE => Cost::moves(&[Selection { objects: Picks::These(|q: &dyn Query, asker: &Asker| ((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>()), count: Count::Each }, Selection { objects: Picks::Matching(|q: &dyn Query, asker: &Asker, id: ObjectId| planned_value!("static subjects Take", false => ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((6i64).max(0) as usize).collect::<Vec<ObjectId>>()).contains(&id))), count: Count::Exactly(6i64) }], Destination::Exile, Arrival { tapped: false, face_down: true, states: &[EntryState::ExilePile { shuffled: true }] }))))?);
let paid_objects_0: Vec<ObjectId> = planned_value!("objects that paid a cost at resolution", Vec::<ObjectId>::new() => g.objects_that_paid());
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let moved_1: Vec<ObjectId> = planned_value!("operation ChangeZone", Vec::<ObjectId>::new() => cantrip_mechanics::enter::change_zone(g, asker, ZoneChange { actor: { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, cards: { let q: &dyn Query = g; paid_objects_0.clone() }, count: None, random: true, group: None, destination: Destination::OwnersLibraryTop, entry: &[], face_down: None })?);
g.end_sentence();
hoisted_2.extend(moved_1.clone());
hoisted_3.extend(moved_1.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Triumph of Saint Katherine", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Praesidium Protectiva — When this creature is put into your graveyard from the battlefield, exile it and the top six cards of your library in a face-down pile. If you do, shuffle that pile and put it back on top of your library.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, miracle: Some(ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
