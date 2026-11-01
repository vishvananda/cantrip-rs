// The Parting of the Ways
// status: planned
// requires: engine: no Time Travel
// requires: exiled card gains suspend
// requires: object filter HasKeywordFamily
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — Exile the top five cards of your library. For each nonland card exiled this way, put a number of time counters on that card equal to its mana value. If it doesn't have suspend, it gains suspend.
// II — Time travel, then time travel.
// III — For each opponent, destroy up to one target artifact that player controls.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((5i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (!(q.chars(id).is(CardType::Land)) && (moved_0.clone()).contains(&id))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new(); for &object in &objects_1 { let amount = { let q: &dyn Query = g; (Some(object).map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).clamp(0, 32767) as i16 }; counters_1.push((object, CounterKind::Time, amount)); } g.add_counters_together(&counters_1);
g.end_sentence();
let cards_2 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (!(q.chars(id).is(CardType::Land)) && (moved_0.clone()).contains(&id) && !(planned_value!("object filter HasKeywordFamily", false => q.has_keyword_ability(id, KeywordAbility::Suspend))))).collect::<Vec<ObjectId>>() };
planned!("exiled-card-gains-suspend" => for card in cards_2 { cantrip_mechanics::suspend::gains_suspend(g, card); });
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-time-travel" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::time_travel::time_travel(g, player, asker.source)?; });
g.end_sentence();
planned!("engine-no-time-travel" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::time_travel::time_travel(g, player, asker.source)?; });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::controlled_by_slot_opponent(q, id)) && (matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Artifact)))) }
static ABILITY_0_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_2_target_0).between(0, TargetSpec::EACH_OPPONENT)];
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_5 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let destroyed_5 = g.destroy_all(&objects_5);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Parting of the Ways", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Exile the top five cards of your library. For each nonland card exiled this way, put a number of time counters on that card equal to its mana value. If it doesn't have suspend, it gains suspend.").targets(&[]),chapter(2, ability_0_1, "II — Time travel, then time travel.").targets(&[]),chapter(3, ability_0_2, "III — For each opponent, destroy up to one target artifact that player controls.").targets(ABILITY_0_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
