// Argentum Masticore
// status: planned
// requires: engine: no protection from this quality
// First strike, protection from multicolored
// At the beginning of your upkeep, sacrifice this creature unless you discard a card. When you discard a card this way, destroy target nonland permanent an opponent controls with mana value less than or equal to the mana value of the discarded card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).is(CardType::Land)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)) && (i64::from(q.mana_value(id))) <= (q.event_subject().object().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o))))))) }
static REFLEXIVE_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_3_target_0)];
fn reflexive_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_4 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_4 = g.destroy_all(&objects_4);
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut paid_0 = false; {
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_1 = Vec::new(); for player in players_1 { cards_1.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_1, true)? }); } let discarded_1 = g.discard_together(&cards_1);
if !cards_1.is_empty() { paid_0 = true;
if !discarded_1.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; (discarded_1.clone()).get(0).copied() }.map_or(Target::None, Target::Object)); }
g.end_sentence();
hoisted_5.extend(discarded_1.clone());
hoisted_6.extend({ let q: &dyn Query = g; discarded_1.clone() });
}
}
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_0 as i64) != 0)) } {
let objects_7 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_7: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_7 { let controller = g.controller(object); if players_7.contains(&controller) && g.can_sacrifice(controller, object) { picks_7.push((controller, object)); } }
let sacrificed_7 = g.sacrifice_each(&picks_7);
g.end_sentence();
hoisted_8.extend(sacrificed_7.clone());
hoisted_9.extend({ let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-protection-from-this-quality" => statics: cantrip_mechanics::protection::from_matching(ObjectKind::Any, |q, asker, source| (q.chars(source).colors.iter().count() as i64) > (1i64), "Protection from a quality"));

pub static CARD: CardDef = CardDef { name: "Argentum Masticore", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Masticore]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of your upkeep, sacrifice this creature unless you discard a card. When you discard a card this way, destroy target nonland permanent an opponent controls with mana value less than or equal to the mana value of the discarded card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_3, "When you discard a card this way, destroy target nonland permanent an opponent controls with mana value less than or equal to the mana value of the discarded card.").targets(REFLEXIVE_3_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
