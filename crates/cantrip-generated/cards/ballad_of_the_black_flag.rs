// Ballad of the Black Flag
// status: planned
// requires: operation EffectOn
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after IV.)
// I, II, III — Mill three cards. You may put a historic card from among them into your hand. (Artifacts, legendaries, and Sagas are historic.)
// IV — Historic spells you cast this turn cost {2} less to cast.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
{
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; (milled_0.clone()).into_iter().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).supertypes.legendary || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga)))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::PutIntoHand, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
if !selected_1.is_empty() { 
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let moved_2 = g.return_to_hand(&objects_2);
g.end_sentence();
hoisted_3.extend(moved_2.clone());
hoisted_4.extend(selected_1.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_6 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_6: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_6)).collect(); let milled_6 = g.mill_together(&mills_6);
g.end_sentence();
{
let selected_7_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_7_candidates = { let q: &dyn Query = g; (milled_6.clone()).into_iter().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).supertypes.legendary || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga)))).collect::<Vec<ObjectId>>() }; let selected_7_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_7 = { cantrip_mechanics::choose::select(g, selected_7_player, asker.source, Purpose::PutIntoHand, selected_7_candidates, cantrip_mechanics::select::Selection::of(selected_7_count, &[]))? };
if !selected_7.is_empty() { 
let objects_8 = { let q: &dyn Query = g; selected_7.clone() }; let moved_8 = g.return_to_hand(&objects_8);
g.end_sentence();
hoisted_9.extend(moved_8.clone());
hoisted_10.extend(selected_7.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_17: Vec<ObjectId> = Vec::new();
let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_12 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_12: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_12)).collect(); let milled_12 = g.mill_together(&mills_12);
g.end_sentence();
{
let selected_13_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_13_candidates = { let q: &dyn Query = g; (milled_12.clone()).into_iter().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).supertypes.legendary || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga)))).collect::<Vec<ObjectId>>() }; let selected_13_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_13 = { cantrip_mechanics::choose::select(g, selected_13_player, asker.source, Purpose::PutIntoHand, selected_13_candidates, cantrip_mechanics::select::Selection::of(selected_13_count, &[]))? };
if !selected_13.is_empty() { 
let objects_14 = { let q: &dyn Query = g; selected_13.clone() }; let moved_14 = g.return_to_hand(&objects_14);
g.end_sentence();
hoisted_15.extend(moved_14.clone());
hoisted_16.extend(selected_13.clone());
hoisted_17.extend({ let q: &dyn Query = g; objects_14.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
fn ability_0_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-effect-on" => cantrip_mechanics::effects::effect_on(g, asker, EffectOn::EachSpell(|q: &dyn Query, asker: &Asker, id: ObjectId| ((q.chars(id).is(CardType::Artifact) || q.chars(id).supertypes.legendary || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && cast_of(q, id).is_some_and(|cast| Some(asker.controller).into_iter().collect::<Vec<PlayerId>>().contains(&cast.player)))), &[SpellEffect::DecreaseManaCost(CostReductionSymbol::CostReduceGeneric(2))], Some(Duration::EndOfTurn))?);
g.end_sentence();
 Ok(()) }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }

pub static CARD: CardDef = CardDef { name: "Ballad of the Black Flag", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II, III — Mill three cards. You may put a historic card from among them into your hand.").targets(&[]),chapter(2, ability_0_1, "I, II, III — Mill three cards. You may put a historic card from among them into your hand.").targets(&[]),chapter(3, ability_0_2, "I, II, III — Mill three cards. You may put a historic card from among them into your hand.").targets(&[]),chapter(4, ability_0_3, "IV — Historic spells you cast this turn cost {2} less to cast.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
