// Lodestone Bauble
// status: ready
// {1}, {T}, Sacrifice this artifact: Put up to four target basic land cards from a player's graveyard on top of their library in any order. That player draws a card at the beginning of the next turn's upkeep.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, 4).together(cantrip_api::select::set_part(&(common_owner as SelectFn), &(largest_common_owner as cantrip_api::select::LargestFn), 0, "with the same owner"))];
fn delayed_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; about(q, item, 0).first().copied().map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_0 = { let order_0 = match objects_0.iter().all(|&o| g.state().obj(o).owner == asker.controller) { true => cantrip_mechanics::library::ask_library_order(g, asker.controller, asker.source, &objects_0, DestinationKind::LibraryTop)?, false => objects_0.clone() }; g.put_into_library(&order_0, LibraryPlace::Top) };
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::Upkeep, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn common_by(q: &dyn Query, choice: &Choice, key: fn(&dyn Query, ObjectId) -> PlayerId) -> Verdict { let first = choice.selected.first().map(|&c| key(q, c)); Verdict { complete: choice.selected.iter().all(|&c| Some(key(q, c)) == first), addable: Bits::of(choice.candidates.len(), |i| first.map_or(true, |k| key(q, choice.candidates[i]) == k)) } }
fn largest_by(q: &dyn Query, choice: &Choice, key: fn(&dyn Query, ObjectId) -> PlayerId) -> Vec<ObjectId> { choice.candidates.iter().map(|&a| choice.candidates.iter().copied().filter(|&b| key(q, b) == key(q, a)).collect::<Vec<_>>()).max_by_key(Vec::len).unwrap_or_default() }
fn owner_of(q: &dyn Query, id: ObjectId) -> PlayerId { q.state().obj(id).owner }
fn common_owner(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { common_by(q, choice, owner_of) }
fn largest_common_owner(q: &dyn Query, choice: &Choice, _: u32) -> Vec<ObjectId> { largest_by(q, choice, owner_of) }

pub static CARD: CardDef = CardDef { name: "Lodestone Bauble", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{1}, {T}, Sacrifice this artifact: Put up to four target basic land cards from a player's graveyard on top of their library in any order. That player draws a card at the beginning of the next turn's upkeep.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_2, "{1}, {T}, Sacrifice this artifact: Put up to four target basic land cards from a player's graveyard on top of their library in any order. That player draws a card at the beginning of the next turn's upkeep.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
