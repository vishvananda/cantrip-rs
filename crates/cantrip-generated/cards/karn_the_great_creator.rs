// Karn, the Great Creator
// status: ready
// Activated abilities of artifacts your opponents control can't be activated.
// [+1]: Until your next turn, up to one target noncreature artifact becomes an artifact creature with power and toughness each equal to its mana value.
// [−2]: You may reveal an artifact card you own from outside the game or choose a face-up artifact card you own in exile. Put that card into your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Artifact) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(affected))) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, object: ObjectId, ability: &ActivatedDef) -> bool { !(q.state().obj(object).zone.is(ZoneKind::Battlefield) && (q.chars(object).is(CardType::Artifact) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(object)))) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).is(CardType::Creature)) && q.chars(id).is(CardType::Artifact)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 1)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") });
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::AddTypes(TypeSet::of(&[CardType::Artifact, CardType::Creature])) };
let kind_0_1_0 = { let q: &dyn Query = g; EffectKind::SetPt((t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).clamp(-32768, 32767) as i16, (t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).clamp(-32768, 32767) as i16) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in (q.outside_the_game(asker.controller)).into_iter().filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (!(q.state().obj(id).face_down) && q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::PutIntoHand, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
if !selected_2.is_empty() { 
let objects_3 = { let q: &dyn Query = g; selected_2.clone() }; let moved_3 = { let (outside, inside): (Vec<ObjectId>, Vec<ObjectId>) = objects_3.iter().partition(|&&o| g.state().obj(o).zone.is(ZoneKind::Sideboard)); let mut moved = g.bring_into_hand_revealed(&outside); moved.extend(g.return_to_hand_revealed(&inside)); moved };
g.end_sentence();
hoisted_4.extend(moved_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Karn, the Great Creator", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Karn]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(5), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_1, "+1: Until your next turn, up to one target noncreature artifact becomes an artifact creature with power and toughness each equal to its mana value.").targets(ABILITY_1_TARGETS).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-2)), ability_2, "−2: You may reveal an artifact card you own from outside the game or choose a face-up artifact card you own in exile. Put that card into your hand.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, activation_rules: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
