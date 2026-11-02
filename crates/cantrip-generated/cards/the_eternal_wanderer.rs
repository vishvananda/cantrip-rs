// The Eternal Wanderer
// status: ready
// No more than one creature can attack The Eternal Wanderer each combat.
// [+1]: Exile up to one target artifact or creature. Return that card to the battlefield under its owner's control at the beginning of that player's next end step.
// [0]: Create a 2/2 white Samurai creature token with double strike.
// [−4]: For each player, choose a creature that player controls. Each player sacrifices all creatures they control not chosen this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_limit(q: &dyn Query, asker: &Asker, attacked: Target) -> Option<u32> { (attacked == Target::Object(asker.source)).then_some(1) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 1)];
fn delayed_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = ({ let q: &dyn Query = g; (about(q, item, 0)).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let rows_4: Vec<(ObjectId, PlayerId, bool)> = objects_4.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_4 = g.put_all_onto_battlefield_as(&rows_4)?; came_4.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
if let Some(present_3_0) = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().map(|o: ObjectId| q.state().obj(o).owner) } { { let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: Some({ let q: &dyn Query = g; present_3_0 }) }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
 }
g.end_sentence();
 Ok(()) }
pub mod token_7 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Samurai).name, token_words: token!(Samurai).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Samurai]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::DoubleStrike]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_6 = Vec::new();
let count_6_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_6_0 = g.card_id(&token_7::CARD);
for player in players_6 {
tokens_6.extend(std::iter::repeat((def_6_0,player)).take(count_6_0 as usize));
} let words_6 = g.words(asker.source);
let created_6 = g.create_tokens_worded(&tokens_6, false, None, words_6)?;
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_8: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_8.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
 for (index_8, bound_0) in players_8.iter().copied().enumerate() { let selected_9_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_9_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_9_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_9 = { cantrip_mechanics::choose::select(g, selected_9_player, asker.source, Purpose::Keep, selected_9_candidates, cantrip_mechanics::select::Selection::of(selected_9_count, &[]))? };
 each_10[index_8] = selected_9.clone(); all_10.extend(each_10[index_8].iter().copied());
   } 
g.end_sentence();
let objects_11 = { let q: &dyn Query = g; { let chosen: Vec<ObjectId> = all_10.clone(); (players_8.iter().copied().flat_map(|bound_0: PlayerId| q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).collect::<Vec<ObjectId>>()).into_iter().filter(|o| !chosen.contains(o)).collect::<Vec<ObjectId>>() } }; let players_11 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_11: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_11 { let controller = g.controller(object); if players_11.contains(&controller) && g.can_sacrifice(controller, object) { picks_11.push((controller, object)); } }
let sacrificed_11 = g.sacrifice_each(&picks_11);
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Eternal Wanderer", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(5), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_1, "+1: Exile up to one target artifact or creature. Return that card to the battlefield under its owner's control at the beginning of that player's next end step.").targets(ABILITY_1_TARGETS).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(0)), ability_2, "0: Create a 2/2 white Samurai creature token with double strike.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-4)), ability_3, "−4: For each player, choose a creature that player controls. Each player sacrifices all creatures they control not chosen this way.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_3, "Return that card to the battlefield under its owner's control at the beginning of that player's next end step.")], enters: None, modal: None, enchant: None, attack_limits: &[ability_0_limit], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_7::CARD];
