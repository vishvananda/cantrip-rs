// Maelstrom Archangel Avatar
// status: planned
// requires: card type Vanguard
// requires: keyword action ChooseCopy
// requires: layout Vanguard
// requires: sharing ManaCosts
// At end of combat, for each creature you controlled at the time it dealt combat damage to a player this turn, copy a random card with the same mana cost as that creature. You may pay {3}. If you do, choose one of those copies. If a copy of a permanent card is chosen, you may create a token that's a copy of that card. If a copy of an instant or sorcery card is chosen, you may cast the copy without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::EndOfCombat) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
for bound_0 in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Damage { source, target, amount, kind, controller, known, .. } = event { let (source, amount, combat, controller) = (*source, *amount, kind.is_combat(), *controller); let dealt = DamageDealt { source, target: *target, amount, combat, controller, known: *known }; let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && q.state().obj(source).zone.is(ZoneKind::Battlefield) && Some(id) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64))).collect::<Vec<ObjectId>>() } {
let mut pool_0: Vec<ObjectId> = { let q: &dyn Query = g; (q.outside_the_game(asker.controller)).into_iter().filter(|&id| Some(bound_0).is_some_and(|o: ObjectId| { let (a, b) = (&q.chars(id), &q.chars(o)); planned_value!("sharing ManaCosts", false => a.mana_cost() == b.mana_cost()) })).collect::<Vec<ObjectId>>() };
let count_0 = { let q: &dyn Query = g; 1i64 };
let mut random_0: Vec<ObjectId> = Vec::new(); for _ in 0..count_0 { let Some(one) = cantrip_mechanics::library::random_of(g, &pool_0) else { break }; pool_0.retain(|&o| o != one); random_0.push(one); }
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_1 = { let q: &dyn Query = g; random_0.clone() }; let times_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_1: Vec<ObjectId> = Vec::new(); for _ in 0..times_1 { for &card in &originals_1 { copies_1.push(g.create_copy_in_exile(card, player_1)); } }
g.end_sentence();
hoisted_2.extend(copies_1.clone());
hoisted_3.extend(random_0.clone());
}
g.end_sentence();
let payer_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_4 = g.may_pay(payer_4, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_4 as i64) != 0) } {
planned!("keyword-action-choose-copy" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::copy::choose_copy(g, player, asker.source)?; });
g.end_sentence();
if { let q: &dyn Query = g; q.state().chosen_object(q.linked(item)).is_some_and(|candidate: ObjectId| (q.chars(candidate).is(CardType::Artifact) || q.chars(candidate).is(CardType::Battle) || q.chars(candidate).is(CardType::Creature) || q.chars(candidate).is(CardType::Enchantment) || q.chars(candidate).is(CardType::Land) || q.chars(candidate).is(CardType::Planeswalker))) } {
let decider_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_6, asker.source, Purpose::Optional)? {
let original_7 = { let q: &dyn Query = g; (hoisted_2.clone()).first().copied() }; let count_7 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_7 = match original_7 { Some(original) => g.create_token_copies(original, player_7, count_7)?, None => Vec::new() };
g.end_sentence();
hoisted_8.extend(created_7.clone());
}
g.end_sentence();
hoisted_9.extend(hoisted_8.clone());
} else {
let caster_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_10: Vec<ObjectId> = { let q: &dyn Query = g; (hoisted_2.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_10: u32 = 1; let mut played_10: Vec<ObjectId> = Vec::new();
while left_10 > 0 {
let mut open_10: Vec<ObjectId> = Vec::new(); for &c in &pool_10 { let ways = g.cast_ways_during_resolution_by(c, caster_10); if ways.iter().any(|w| true) { open_10.push(c); } }
let picked_10 = if open_10.is_empty() { None } else if open_10.len() > 1 || (true && (!false || pool_10.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_10, asker.source, Purpose::CastFree, pool_10.clone(), open_10.clone(), 1, true)?.first().copied() } else if cantrip_mechanics::effects::ask_may(g, caster_10, open_10[0], Purpose::CastFree)? { Some(open_10[0]) } else { None };
let Some(card_10) = picked_10 else { break; };
let ways_10: Vec<CastWay> = g.cast_ways_during_resolution_by(card_10, caster_10).into_iter().filter(|w| true).collect();
let done_10 = { let way = cantrip_mechanics::choose::cast_way(g, caster_10, card_10, ways_10)?.map(|w| w.way); g.cast_during_resolution_as(card_10, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_10), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_10 { left_10 -= 1; played_10.push(g.state().obj(card_10).became.unwrap_or(card_10)); } else { break; } if done_10 || !true { pool_10.retain(|&c| c != card_10); }
}
g.end_sentence();
}
g.end_sentence();
hoisted_11.extend(hoisted_9.clone());
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 0, life: 5 }));

pub static CARD: CardDef = CardDef { name: "Maelstrom Archangel Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At end of combat, for each creature you controlled at the time it dealt combat damage to a player this turn, copy a random card with the same mana cost as that creature. You may pay {3}. If you do, choose one of those copies. If a copy of a permanent card is chosen, you may create a token that's a copy of that card. If a copy of an instant or sorcery card is chosen, you may cast the copy without paying its mana cost.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
