// Aurelia's Vindicator
// status: ready
// Flying, lifelink, ward {2}
// Disguise {X}{3}{W}
// When this creature is turned face up, exile up to X other target creatures from the battlefield and/or creature cards from graveyards.
// When this creature leaves the battlefield, return the exiled cards to their owners' hands.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_4_0_count(q: &dyn Query, asker: &Asker) -> u16 { (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) as u16 }
#[allow(unused_variables)] fn ability_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { (matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.event_subject().object() == Some(id)) && q.chars(id).is(CardType::Creature))))) || (matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (q.chars(id).is(CardType::Creature)))) }
static ABILITY_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_4_target_0).between(0, TargetSpec::COUNTED)];
#[allow(unused_variables)] fn ability_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TurnedFaceUp { permanent, .. } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let moved_0 = { let by = g.linked(item); g.exile_with(&objects_0, by, false) };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.exiled_with(q.linked_to(asker)).contains(&id)).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_1 = g.return_to_hand(&objects_1);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Aurelia's Vindicator", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward::<2>("Ward {2}"),triggered(when_asked(EventMask(EventMask::TURNED_FACE_UP.0), ability_4_event), ability_4, "When this creature is turned face up, exile up to X other target creatures from the battlefield and/or creature cards from graveyards.").targets(ABILITY_4_TARGETS).counted_by(ability_4_0_count),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_5_event), ability_5, "When this creature leaves the battlefield, return the exiled cards to their owners' hands.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::disguise::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 1, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), "Disguise {X}{3}{W}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
