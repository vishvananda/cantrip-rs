// Nicol Bolas, Dragon-God
// status: planned
// requires: per-player instruction with several steps
// requires: the loyalty abilities of other permanents
// Nicol Bolas has all loyalty abilities of all other planeswalkers on the battlefield.
// [+1]: You draw a card. Each opponent exiles a card from their hand or a permanent they control.
// [−3]: Destroy target creature or planeswalker.
// [−8]: Each opponent who doesn't control a legendary creature or planeswalker loses the game.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let ids = q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Planeswalker)).collect::<Vec<ObjectId>>(); planned_value!("the loyalty abilities of other permanents", () => cantrip_mechanics::statics::grant_loyalty_abilities_of(q, &ids, chars)); }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
planned!("per-player-instruction-with-several-steps" => g.begin_batch(););
let mut each_13: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_13: Vec<ObjectId> = Vec::new();
let mut each_14: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_14: Vec<ObjectId> = Vec::new();
let mut each_15: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_15: Vec<ObjectId> = Vec::new();
let mut each_16: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_16: Vec<ObjectId> = Vec::new();
let mut each_17: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_17: Vec<ObjectId> = Vec::new();
let mut each_18: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_18: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let decider_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let offered_2: Vec<u8> = [(0u8, !{ let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }.is_empty()), (1u8, !{ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (true && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }.is_empty())].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_2 = item.ability.card; let picked_2 = if offered_2.len() > 1 { offered_2[cantrip_mechanics::choose::one_of(g, decider_2, asker.source, offered_2.iter().map(|&option| Label::Card(card_2, option)).collect())? as usize] } else { offered_2.first().copied().unwrap_or(u8::MAX) }; match picked_2 {
0 => {
let selected_3_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::Exile, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
let objects_4 = { let q: &dyn Query = g; selected_3.clone() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
hoisted_5.extend(moved_4.clone());
hoisted_6.extend(selected_3.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
1 => {
let selected_8_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_8_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (true && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_8_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_8 = { cantrip_mechanics::choose::select(g, selected_8_player, asker.source, Purpose::Exile, selected_8_candidates, cantrip_mechanics::select::Selection::of(selected_8_count, &[]))? };
let objects_9 = { let q: &dyn Query = g; selected_8.clone() }; let moved_9 = g.exile_all(&objects_9);
g.end_sentence();
hoisted_10.extend(moved_9.clone());
hoisted_11.extend(selected_8.clone());
hoisted_12.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
_ => {}
}
 each_13[index_1] = hoisted_6.clone(); all_13.extend(each_13[index_1].iter().copied());
each_14[index_1] = hoisted_11.clone(); all_14.extend(each_14[index_1].iter().copied());
each_15[index_1] = hoisted_5.clone(); all_15.extend(each_15[index_1].iter().copied());
each_16[index_1] = hoisted_10.clone(); all_16.extend(each_16[index_1].iter().copied());
each_17[index_1] = { let q: &dyn Query = g; hoisted_7.clone() }; all_17.extend(each_17[index_1].iter().copied());
each_18[index_1] = { let q: &dyn Query = g; hoisted_12.clone() }; all_18.extend(each_18[index_1].iter().copied());
   } 
planned!("per-player-instruction-with-several-steps" => g.end_batch(););
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_19 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_19 = g.destroy_all(&objects_19);
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_20: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| !(q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(bound_0).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).supertypes.legendary && (q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)))).count() >= 1)).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } };
 for (index_20, bound_0) in players_20.iter().copied().enumerate() { { let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
    } 
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Nicol Bolas, Dragon-God", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 3, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Bolas]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(4), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_1, "+1: You draw a card. Each opponent exiles a card from their hand or a permanent they control.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-3)), ability_2, "−3: Destroy target creature or planeswalker.").targets(ABILITY_2_TARGETS).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-8)), ability_3, "−8: Each opponent who doesn't control a legendary creature or planeswalker loses the game.").targets(&[]).sorcery_speed()], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_0_apply_0, "Nicol Bolas has all loyalty abilities of all other planeswalkers on the battlefield.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["A card from their hand", "A permanent they control"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
