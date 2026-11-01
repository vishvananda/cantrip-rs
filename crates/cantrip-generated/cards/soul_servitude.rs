// Soul Servitude
// status: planned
// requires: engine: no perpetual effects
// requires: operation Conjure
// requires: reflexive ability about a list of objects
// Target player sacrifices a nontoken creature of their choice. When they do, you may discard a card. If you do, conjure a duplicate of the sacrificed creature into your hand. It perpetually gains "Mana of any type can be spent to cast this spell."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn reflexive_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_3, after_3, before_3, picked_3): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_5 = Vec::new(); for player in players_5 { cards_5.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_5, true)? }); } let discarded_5 = g.discard_together(&cards_5);
let paid_4 = !discarded_5.clone().is_empty();
(paid_4, { let q: &dyn Query = g; (discarded_5).clone() }, { let q: &dyn Query = g; (discarded_5.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_3 as i64) != 0) } {
let duplicated_6 = { let q: &dyn Query = g; (planned_value!("reflexive ability about a list of objects", Vec::<ObjectId>::new() => about(q, item, objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()))).get(0).copied().into_iter().collect::<Vec<ObjectId>>() };
let count_6: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_6: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_6.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::DuplicateOf(&duplicated_6), count_6, ConjureTo::Hand)?)); }
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_8: Vec<ObjectId> = { let q: &dyn Query = g; (conjured_6.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
let kind_8_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::SpendAsThoughAnyType };
for &object in &objects_8 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_8_0); });
g.end_sentence();
hoisted_9.extend(conjured_6.clone());
hoisted_10.extend(conjured_6.clone());
}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let players_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_0: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_0 { let eligible: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_0.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_0.clone(), &[]))? }; for object in taken { picks_0.push((player, object)); } }
let sacrificed_0 = g.sacrifice_each(&picks_0);
if !sacrificed_0.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Soul Servitude", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_2, "Target player sacrifices a nontoken creature of their choice. When they do, you may discard a card. If you do, conjure a duplicate of the sacrificed creature into your hand. It perpetually gains \"Mana of any type can be spent to cast this spell.\"").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
