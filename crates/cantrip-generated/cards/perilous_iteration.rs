// Perilous Iteration
// status: planned
// requires: delayed trigger starting PlayersNextTurn
// requires: engine: a delayed ability on an event keeps no list
// Seek a card with mana value 2 or less and a card with mana value 3 or greater. Discard those cards at the beginning of your next turn's end step.
#[allow(unused_imports)] use cantrip_api::*;
fn delayed_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let cards_6 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ({ let mut all: Vec<ObjectId> = Vec::new(); for id in about(q, item, 0) { if !all.contains(&id) { all.push(id); } } for id in about(q, item, 1) { if !all.contains(&id) { all.push(id); } } all }).contains(&id)).collect::<Vec<ObjectId>>() }; let discarded_6 = g.discard_together(&cards_6);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_0 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (i64::from(q.mana_value(id))) <= (2i64)).collect::<Vec<ObjectId>>() }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_0: Vec<ObjectId> = Vec::new(); while picked_0.len() < count_0 && !pool_0.is_empty() { let at = g.choose_at_random(pool_0.len()); picked_0.push(pool_0.remove(at)); }
let objects_1 = { let q: &dyn Query = g; picked_0.clone() }; let moved_1 = g.return_to_hand(&objects_1);
g.end_sentence();
let mut pool_2 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (i64::from(q.mana_value(id))) >= (3i64)).collect::<Vec<ObjectId>>() }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_2: Vec<ObjectId> = Vec::new(); while picked_2.len() < count_2 && !pool_2.is_empty() { let at = g.choose_at_random(pool_2.len()); picked_2.push(pool_2.remove(at)); }
let objects_3 = { let q: &dyn Query = g; picked_2.clone() }; let moved_3 = g.return_to_hand(&objects_3);
g.end_sentence();
planned!("engine-a-delayed-ability-on-an-event-keeps-no-list" => { let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; moved_1.clone() }); about.push(ObjectId::NONE); about.extend({ let q: &dyn Query = g; moved_3.clone() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(planned_value!("delayed trigger starting PlayersNextTurn", DelayedWhen::Step { step: Step::End, on_turn_of: Some({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }) } => DelayedWhen::StepOfNextTurn { step: Step::End, of: Some({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }) }), item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } });
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }

pub static CARD: CardDef = CardDef { name: "Perilous Iteration", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_5, "Discard those cards at the beginning of your next turn's end step.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
