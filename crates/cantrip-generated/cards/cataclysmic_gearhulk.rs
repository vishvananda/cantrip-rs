// Cataclysmic Gearhulk
// status: ready
// Vigilance
// When this creature enters, each player chooses an artifact, a creature, an enchantment, and a planeswalker from among the nonland permanents they control, then sacrifices the rest.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let pool_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let mut picked_1: Vec<ObjectId> = Vec::new();
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Enchantment)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Planeswalker)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
 each_2[index_0] = picked_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; { let chosen: Vec<ObjectId> = all_2.clone(); (players_0.iter().copied().flat_map(|bound_0: PlayerId| q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).collect::<Vec<ObjectId>>()).into_iter().filter(|o| !chosen.contains(o)).collect::<Vec<ObjectId>>() } }; let players_3 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_3 { let controller = g.controller(object); if players_3.contains(&controller) && g.can_sacrifice(controller, object) { picks_3.push((controller, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Cataclysmic Gearhulk", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Construct]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, each player chooses an artifact, a creature, an enchantment, and a planeswalker from among the nonland permanents they control, then sacrifices the rest.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
