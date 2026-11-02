// Titania's Command
// status: ready
// Choose two —
// • Exile target player's graveyard. You gain 1 life for each card exiled this way.
// • Search your library for up to two land cards, put them onto the battlefield tapped, then shuffle.
// • Create two 2/2 green Bear creature tokens.
// • Put two +1/+1 counters on each creature you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; ((objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).len() as i64).min(i32::MAX.into()) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_2 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((2i64).max(0) as u32) }; let shown_2 = cantrip_mechanics::library::searched_cards(g, owner_2); let can_2: Vec<ObjectId> = { let q: &dyn Query = g; shown_2.iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect() }; let found_2 = cantrip_mechanics::library::search(g, searcher_2, owner_2, asker.source, SearchDestination::BattlefieldTapped, cantrip_mechanics::select::Selection::listed(rule_count_2, &shown_2, can_2))?;
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_3 = ({ let q: &dyn Query = g; found_2.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows_3: Vec<(ObjectId, PlayerId, bool)> = objects_3.iter().map(|&o| (o, player_3, true)).collect(); let came_3 = g.put_all_onto_battlefield_as(&rows_3)?; came_3.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
pub mod token_6 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Bear).name, token_words: token!(Bear).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bear]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_5 = Vec::new();
let count_5_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let def_5_0 = g.card_id(&token_6::CARD);
for player in players_5 {
tokens_5.extend(std::iter::repeat((def_5_0,player)).take(count_5_0 as usize));
} let words_5 = g.words(asker.source);
let created_5 = g.create_tokens_worded(&tokens_5, false, None, words_5)?;
g.end_sentence();
 Ok(()) }
fn ability_0_mode_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_7: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (2i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_7.push((object, CounterKind::PlusOne, amount)); } }
let had_7: Vec<_> = counters_7.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_7); let applied_7: Vec<ObjectId> = counters_7.iter().zip(had_7).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Exile target player's graveyard. You gain 1 life for each card exiled this way.").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, "Search your library for up to two land cards, put them onto the battlefield tapped, then shuffle.").targets(&[]),mode(ability_0_mode_2, "Create two 2/2 green Bear creature tokens.").targets(&[]),mode(ability_0_mode_3, "Put two +1/+1 counters on each creature you control.").targets(&[])]).choose(2, 2);
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Titania's Command", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_6::CARD];
