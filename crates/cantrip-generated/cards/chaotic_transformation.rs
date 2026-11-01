// Chaotic Transformation
// status: ready
// Exile up to one target artifact, up to one target creature, up to one target enchantment, up to one target planeswalker, and/or up to one target land. For each permanent exiled this way, its controller reveals cards from the top of their library until they reveal a card that shares a card type with it, puts that card onto the battlefield, then shuffles.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Artifact))) }
#[allow(unused_variables)] fn ability_0_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_0_target_2(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Enchantment))) }
#[allow(unused_variables)] fn ability_0_target_3(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Planeswalker))) }
#[allow(unused_variables)] fn ability_0_target_4(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Land))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, 1),target(ability_0_target_1).between(0, 1),target(ability_0_target_2).between(0, 1),target(ability_0_target_3).between(0, 1),target(ability_0_target_4).between(0, 1)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let t_2_raw = announced(g, item, 2, false); let t_2 = announced(g, item, 2, true);
let t_3_raw = announced(g, item, 3, false); let t_3 = announced(g, item, 3, true);
let t_4_raw = announced(g, item, 4, false); let t_4 = announced(g, item, 4, true);
let objects_0 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() { if !all.contains(&id) { all.push(id); } } for id in t_1_raw.get(0).copied().filter(|t| t_1.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() { if !all.contains(&id) { all.push(id); } } for id in t_2_raw.get(0).copied().filter(|t| t_2.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() { if !all.contains(&id) { all.push(id); } } for id in t_3_raw.get(0).copied().filter(|t| t_3.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() { if !all.contains(&id) { all.push(id); } } for id in t_4_raw.get(0).copied().filter(|t| t_4.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() { if !all.contains(&id) { all.push(id); } } all } }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
for bound_0 in { let q: &dyn Query = g; moved_0.clone() } {
let owner_1 = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_1, rest_1) = cantrip_mechanics::library::reveal_until_n(g, owner_1, count_1, &|q: &dyn Query, id: ObjectId| Some(bound_0).is_some_and(|o: ObjectId| { let (a, b) = (&q.chars(id), &q.chars(o)); a.types.iter().any(|t| b.types.has(t)) })); let shown_1 = found_1.len() + rest_1.len(); let objects_1: Vec<ObjectId> = g.state().zone(Zone::library(owner_1)).iter().copied().take(shown_1).collect();
let player_2 = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") };
let objects_2 = ({ let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| Some(bound_0).is_some_and(|o: ObjectId| { let (a, b) = (&q.chars(id), &q.chars(o)); a.types.iter().any(|t| b.types.has(t)) })).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_2 = { let rows_2: Vec<(ObjectId, PlayerId, bool)> = objects_2.iter().map(|&o| (o, player_2, false)).collect(); let came_2 = g.put_all_onto_battlefield_as(&rows_2)?; came_2.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
for player in { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_4.extend(moved_2.clone());
hoisted_5.extend(objects_1.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Chaotic Transformation", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
