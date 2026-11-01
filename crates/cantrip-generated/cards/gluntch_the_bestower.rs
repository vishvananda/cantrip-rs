// Gluntch, the Bestower
// status: planned
// requires: player reference Named
// Flying
// At the beginning of your end step, choose a player. They put two +1/+1 counters on a creature they control. Choose a second player to draw a card. Then choose a third player to create two Treasure tokens.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
pub mod token_11 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Treasure).name, token_words: token!(Treasure).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Treasure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_0 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let Some(chosen_0) = cantrip_mechanics::choose::player(g, player_0, asker.source, candidates_0, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let (carried_4_0,) = if let Some(present_4_0) = { let q: &dyn Query = g; planned_value!("player reference Named", None::<PlayerId> => q.named_player(asker, 0, 0)) } { let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
{ let amount = { let q: &dyn Query = g; (2i64).max(0) as i16 }; let player = { let q: &dyn Query = g; present_4_0 }; let candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && planned_value!("player reference Named", None::<PlayerId> => q.named_player(asker, 0, 0)) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let chosen = cantrip_mechanics::choose::select(g, player, asker.source, Purpose::PutCounters(CounterKind::PlusOne, amount.clamp(0, 255) as u8), candidates, cantrip_mechanics::select::Selection::of(count, &[]))?; for object in chosen { counters_3.push((object, CounterKind::PlusOne, amount)); } }
let had_3: Vec<_> = counters_3.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_3); let applied_3: Vec<ObjectId> = counters_3.iter().zip(had_3).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
(applied_3.clone(),) } else { Default::default() };
g.end_sentence();
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_5 = { let q: &dyn Query = g; { let other = planned_value!("player reference Named", None::<PlayerId> => q.named_player(asker, 1, 0)); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() } }; let Some(chosen_5) = cantrip_mechanics::choose::player(g, player_5, asker.source, candidates_5, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let amount_7 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_7 = { let q: &dyn Query = g; planned_value!("player reference Named", None::<PlayerId> => q.named_player(asker, 2, 1)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_7 { g.draw(player, amount_7); }
g.end_sentence();
let player_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_8 = { let q: &dyn Query = g; { let mut all = { let other = planned_value!("player reference Named", None::<PlayerId> => q.named_player(asker, 2, 0)); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }; let other = { let other = planned_value!("player reference Named", None::<PlayerId> => q.named_player(asker, 2, 1)); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }; all.retain(|p: &PlayerId| other.contains(p)); all } }; let Some(chosen_8) = cantrip_mechanics::choose::player(g, player_8, asker.source, candidates_8, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let players_10 = { let q: &dyn Query = g; planned_value!("player reference Named", None::<PlayerId> => q.named_player(asker, 4, 2)).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_10 = Vec::new();
let count_10_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let def_10_0 = g.card_id(&token_11::CARD);
for player in players_10 {
tokens_10.extend(std::iter::repeat((def_10_0,player)).take(count_10_0 as usize));
} let words_10 = Words::NONE;
let created_10 = g.create_tokens_worded(&tokens_10, false, None, words_10)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Gluntch, the Bestower", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Jellyfish]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(0), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your end step, choose a player. They put two +1/+1 counters on a creature they control. Choose a second player to draw a card. Then choose a third player to create two Treasure tokens.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_11::CARD];
