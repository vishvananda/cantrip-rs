// Intruder's Inquisition
// status: ready
// Target creature you control deals damage equal to its power to target creature an opponent controls. If excess damage was dealt to a creature this way, its controller discards a card with the greatest mana value among cards in their hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
#[allow(unused_variables)] fn ability_0_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0),target(ability_0_target_1)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let mut done_0: i64 = 0; let mut excess_0: Vec<(Target, u32)> = Vec::new(); let since_0 = g.history().head() as usize; let source_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let amount_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; (t_1_raw.get(0).copied().filter(|t| t_1.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); done_0 += g.damage_together(&batch).iter().map(|&n| i64::from(n)).sum::<i64>(); excess_0.extend(g.history().all()[since_0..].iter().filter_map(|r| match r.event { Event::Damage { source: s, target, recipient_excess, .. } if s == source => Some((target, recipient_excess)), _ => None })); }
g.end_sentence();
for bound_0 in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && excess_0.iter().any(|&(t, n)| t == Target::Object(id) && n > 0))).collect::<Vec<ObjectId>>() } {
let selected_1_player = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(bound_0).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (i64::from(q.mana_value(id))) == ((((Some(bound_0).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).max().unwrap_or(0))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Discard, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
let cards_2 = { let q: &dyn Query = g; selected_1.clone() }; let discarded_2 = g.discard_together(&cards_2);
g.end_sentence();
hoisted_3.extend(discarded_2.clone());
hoisted_4.extend(selected_1.clone());
hoisted_5.extend({ let q: &dyn Query = g; discarded_2.clone() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Intruder's Inquisition", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
