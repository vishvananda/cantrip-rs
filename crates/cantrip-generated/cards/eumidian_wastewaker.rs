// Eumidian Wastewaker
// status: planned
// requires: engine: no Encore
// requires: per-player instruction with several steps
// Whenever this creature attacks, you and defending player each discard a card or sacrifice a permanent. You draw a card for each land card put into a graveyard this way.
// Encore {6}{B}{B} ({6}{B}{B}, Exile this card from your graveyard: For each opponent, create a token copy that attacks that opponent this turn if able. They gain haste. Sacrifice them at the beginning of the next end step. Activate only as a sorcery.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all: Vec<PlayerId> = Vec::new(); for p in Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } for p in item.subject.object().and_then(|o: ObjectId| q.defending_player(o)).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } all } };
planned!("per-player-instruction-with-several-steps" => g.begin_batch(););
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
let mut each_9: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_9: Vec<ObjectId> = Vec::new();
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
let mut each_11: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_11: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let decider_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let offered_1: Vec<u8> = [(0u8, { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().any(|p: PlayerId| !q.state().zone(Zone::hand(p)).is_empty()) }), (1u8, { let q: &dyn Query = g; { let players: Vec<PlayerId> = Some(bound_0).into_iter().collect::<Vec<PlayerId>>(); (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker))).collect::<Vec<ObjectId>>()).into_iter().any(|o: ObjectId| players.contains(&q.obj(o).controller)) } })].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_1 = item.ability.card; let picked_1 = if offered_1.len() > 1 { offered_1[cantrip_mechanics::choose::one_of(g, decider_1, asker.source, offered_1.iter().map(|&option| Label::Card(card_1, option)).collect())? as usize] } else { offered_1.first().copied().unwrap_or(u8::MAX) }; match picked_1 {
0 => {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_2 = Vec::new(); for player in players_2 { cards_2.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_2)?); } let discarded_2 = g.discard_together(&cards_2);
g.end_sentence();
hoisted_3.extend(discarded_2.clone());
hoisted_4.extend({ let q: &dyn Query = g; discarded_2.clone() });
}
1 => {
let objects_5 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker))).collect::<Vec<ObjectId>>() }; let players_5 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_5 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_5 { let eligible: Vec<ObjectId> = objects_5.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_5.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_5.clone(), &[]))? }; for object in taken { picks_5.push((player, object)); } }
let sacrificed_5 = g.sacrifice_each(&picks_5);
g.end_sentence();
hoisted_6.extend(sacrificed_5.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
_ => {}
}
 each_8[index_0] = hoisted_3.clone(); all_8.extend(each_8[index_0].iter().copied());
each_9[index_0] = hoisted_6.clone(); all_9.extend(each_9[index_0].iter().copied());
each_10[index_0] = { let q: &dyn Query = g; hoisted_4.clone() }; all_10.extend(each_10[index_0].iter().copied());
each_11[index_0] = { let q: &dyn Query = g; hoisted_7.clone() }; all_11.extend(each_11[index_0].iter().copied());
   } 
planned!("per-player-instruction-with-several-steps" => g.end_batch(););
g.end_sentence();
let amount_12 = { let q: &dyn Query = g; ((all_9.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Land)).count() as i64).max(0) as u32 }; let players_12 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_12 { g.draw(player, amount_12); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-encore" => abilities: cantrip_mechanics::encore::ability(ManaCost { generic: 6, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Encore {6}{B}{B}"));

pub static CARD: CardDef = CardDef { name: "Eumidian Wastewaker", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Insect,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever this creature attacks, you and defending player each discard a card or sacrifice a permanent. You draw a card for each land card put into a graveyard this way.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["You and defending player each discard a card", "Sacrifice a permanent"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
