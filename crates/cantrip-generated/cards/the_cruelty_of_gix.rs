// The Cruelty of Gix
// status: ready
// Read ahead (Choose a chapter and start with that many lore counters. Add one after your draw step. Skipped chapters don't trigger. Sacrifice after III.)
// I — Target opponent reveals their hand. You choose a creature or planeswalker card from it. That player discards that card.
// II — Search your library for a card, put that card into your hand, then shuffle. You lose 3 life.
// III — Put target creature card from a graveyard onto the battlefield under your control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_1_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_0_target_0)];
fn ability_1_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; ((t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let mut owners_0: Vec<PlayerId> = objects_0.iter().map(|o| g.state().obj(*o).owner).collect(); owners_0.dedup(); for owner in owners_0 { let cards: Vec<ObjectId> = objects_0.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Discard, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
let cards_2 = { let q: &dyn Query = g; selected_1.clone() }; let discarded_2 = g.discard_together(&cards_2);
g.end_sentence();
 Ok(()) }
fn ability_1_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_3 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let shown_3 = cantrip_mechanics::library::searched_cards(g, owner_3); let can_3: Vec<ObjectId> = { let q: &dyn Query = g; shown_3.iter().copied().filter(|&id| true).collect() }; let found_3 = cantrip_mechanics::library::search(g, searcher_3, owner_3, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_3, &shown_3, can_3))?;
let objects_4 = { let q: &dyn Query = g; found_3.clone() }; let moved_4 = g.return_to_hand(&objects_4);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.lose_life(player, amount_6); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_2_target_0)];
fn ability_1_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_7 = ({ let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let rows_7: Vec<(ObjectId, PlayerId, bool)> = objects_7.iter().map(|&o| (o, player_7, false)).collect(); let came_7 = g.put_all_onto_battlefield_as(&rows_7)?; came_7.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Cruelty of Gix", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_1_0, "I — Target opponent reveals their hand. You choose a creature or planeswalker card from it. That player discards that card.").targets(ABILITY_1_0_TARGETS),chapter(2, ability_1_1, "II — Search your library for a card, put that card into your hand, then shuffle. You lose 3 life.").targets(&[]),chapter(3, ability_1_2, "III — Put target creature card from a graveyard onto the battlefield under your control.").targets(ABILITY_1_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
