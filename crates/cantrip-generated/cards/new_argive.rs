// New Argive
// status: planned
// requires: card type Plane
// requires: layout Planar
// requires: subtype Dominaria
// requires: trigger event Player ChaosEnsues
// Whenever a historic creature you control attacks, it gets +2/+2 until end of turn. (Artifacts, legendaries, and Sagas are historic.)
// Whenever chaos ensues, reveal cards from the top of your library until you reveal a historic card. Put that card into your hand and the rest on the bottom of your library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if ((q.chars(id).is(CardType::Artifact) || q.chars(id).supertypes.legendary || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::Pump((2i64) as i16, (2i64) as i16) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let owner_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_1, rest_1) = cantrip_mechanics::library::reveal_until_n(g, owner_1, count_1, &|q: &dyn Query, id: ObjectId| (q.chars(id).is(CardType::Artifact) || q.chars(id).supertypes.legendary || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga)))); let shown_1 = found_1.len() + rest_1.len(); let objects_1: Vec<ObjectId> = g.state().zone(Zone::library(owner_1)).iter().copied().take(shown_1).collect();
let objects_2 = { let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).supertypes.legendary || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga)))).collect::<Vec<ObjectId>>() }; let moved_2 = g.return_to_hand_revealed(&objects_2);
let library_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_3: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (objects_1.clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_3)).collect(); let placed_3 = cards_3.clone();
g.bottom_in_random_order(library_3, &cards_3);
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "New Argive", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Dominaria", Subtype::Plains => Subtype::Dominaria)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever a historic creature you control attacks, it gets +2/+2 until end of turn.").targets(&[]),triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_1_event), ability_1, "Whenever chaos ensues, reveal cards from the top of your library until you reveal a historic card. Put that card into your hand and the rest on the bottom of your library in a random order.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
