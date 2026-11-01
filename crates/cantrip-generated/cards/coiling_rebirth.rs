// Coiling Rebirth
// status: ready
// Gift a card (You may promise an opponent a gift as you cast this spell. If you do, they draw a card before its other effects.)
// Return target creature card from your graveyard to the battlefield. Then if the gift was promised and that creature isn't legendary, create a token that's a copy of that creature, except it's 1/1.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_promised_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_0_PROMISED_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_promised_target_0)];
#[allow(unused_variables)] fn ability_0_unpromised_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_0_UNPROMISED_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_unpromised_target_0)];
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { if cantrip_mechanics::gift::promised(q, asker.source) { (ability_0_promised_target_0)(q, asker, t) } else { (ability_0_unpromised_target_0)(q, asker, t) } }
fn ability_0_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_0 = ({ let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_0 = { let rows_0: Vec<(ObjectId, PlayerId, bool)> = objects_0.iter().map(|&o| (o, player_0, false)).collect(); let came_0 = g.put_all_onto_battlefield_as(&rows_0)?; came_0.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
if { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().is_some_and(|candidate: ObjectId| !(q.chars(candidate).supertypes.legendary)) } {
let original_1 = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied() }; let values_1 = { let q: &dyn Query = g; original_1.map(|original: ObjectId| q.copiable_values(original).sized(1, 1)) }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_1 = match values_1 { Some(values) => g.create_token_copies_tapped(values, player_1, count_1, false)?, None => Vec::new() };
g.end_sentence();
hoisted_2.extend(created_1.clone());
}
g.end_sentence();
 Ok(()) }
fn ability_0_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_3 = ({ let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows_3: Vec<(ObjectId, PlayerId, bool)> = objects_3.iter().map(|&o| (o, player_3, false)).collect(); let came_3 = g.put_all_onto_battlefield_as(&rows_3)?; came_3.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { if item.gift.is_some() { ability_0_paid(g, item) } else { ability_0_unpaid(g, item) } }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[TargetSpec { min: 1, max: 1, filter: ability_0_target_0, label: "", if_kicked: None, together: None }];
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Coiling Rebirth", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, gift: Some(cantrip_mechanics::gift::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
