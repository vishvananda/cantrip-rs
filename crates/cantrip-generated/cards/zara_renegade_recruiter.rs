// Zara, Renegade Recruiter
// status: ready
// Flying
// Whenever Zara attacks, look at defending player's hand. You may put a creature card from it onto the battlefield under your control tapped and attacking that player or a planeswalker they control. Return that creature to its owner's hand at the beginning of the next end step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_6(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_7 = { let q: &dyn Query = g; about(q, item, 0) }; let moved_7 = g.return_to_hand(&objects_7);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_0 = { let q: &dyn Query = g; ((item.subject.object().and_then(|o: ObjectId| q.defending_player(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; g.look_at(player_0, &objects_0);
g.end_sentence();
{
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((item.subject.object().and_then(|o: ObjectId| q.defending_player(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::PutOntoBattlefield, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
if !selected_1.is_empty() { 
g.end_sentence();
let (carried_4_0, carried_4_1,) = if let Some(present_4_0) = { let q: &dyn Query = g; item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) } { let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_3 = ({ let q: &dyn Query = g; (selected_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_3 { let controller = player_3; let defender = { let player = { let q: &dyn Query = g; present_4_0 }; let candidates: Vec<Target> = g.enter_attacking_targets(controller).into_iter().filter(|t| match *t { Target::Player(p) => p == player, Target::Object(o) => g.controller(o) == player && g.chars(o).is(CardType::Planeswalker), _ => false }).collect(); cantrip_mechanics::combat::choose_among(g, controller, asker.source, &candidates)?.unwrap_or(Target::None) }; let entered = g.put_onto_battlefield_attacking(o, controller, defender, &[])?; let came = g.is_on_battlefield(entered); if came { moved.push(entered); } } moved };
(moved_3.clone(), { let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() },) } else { Default::default() };
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; (carried_4_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE);  if about.iter().any(|&o| o != ObjectId::NONE) { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
hoisted_8.extend(carried_4_0.clone());
hoisted_9.extend(selected_1.clone());
hoisted_10.extend({ let q: &dyn Query = g; carried_4_1.clone() });
}
}
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }

pub static CARD: CardDef = CardDef { name: "Zara, Renegade Recruiter", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Pirate]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Zara attacks, look at defending player's hand. You may put a creature card from it onto the battlefield under your control tapped and attacking that player or a planeswalker they control. Return that creature to its owner's hand at the beginning of the next end step.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_6, "Return that creature to its owner's hand at the beginning of the next end step.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
