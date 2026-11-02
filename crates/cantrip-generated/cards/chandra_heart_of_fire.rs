// Chandra, Heart of Fire
// status: ready
// [+1]: Discard your hand, then exile the top three cards of your library. Until end of turn, you may play cards exiled this way.
// [+1]: Chandra deals 2 damage to any target.
// [−9]: Search your graveyard and library for any number of red instant and/or sorcery cards, exile them, then shuffle. You may cast them this turn. Add six {R}.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_0 = Vec::new(); for player in players_0 { cards_0.extend(g.state().zone(Zone::hand(player)).to_vec()); } let discarded_0 = g.discard_together(&cards_0);
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((3i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
cantrip_api::instruction(g, |g| { let cards_2_0 = { let q: &dyn Query = g; moved_1.clone() }; let players_2_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_2_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_2_0 { g.allow_play(card, player, PlayGrant { lands: true, expiry: Expiry::EndOfTurn, ..PlayGrant::CAST }); } } } });
 });
g.end_sentence();
 Ok(()) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let amount_3 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_3 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_3 { let batch: Vec<(ObjectId, Target, u16)> = recipients_3.iter().map(|t| (source, *t, amount_3)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let mut found_4: Vec<ObjectId> = Vec::new();
{ let shown = g.state().zone(Zone::graveyard(owner_4)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| (q.chars(id).colors.has(q.words(asker.source).color(Color::Red)) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)))).collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_4, asker.source, Purpose::Exile, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?;  found_4.extend(taken); } }
{ let shown = cantrip_mechanics::library::searched_cards(g, owner_4); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| (q.chars(id).colors.has(q.words(asker.source).color(Color::Red)) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)))).collect() }; let taken = cantrip_mechanics::library::search(g, searcher_4, owner_4, asker.source, SearchDestination::Exile, cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?; found_4.extend(taken); }
let objects_5 = { let q: &dyn Query = g; found_4.clone() }; let moved_5 = g.exile_all(&objects_5);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
cantrip_api::instruction(g, |g| { let cards_7_0 = { let q: &dyn Query = g; moved_5.clone() }; let players_7_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_7_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_7_0 { g.allow_play(card, player, PlayGrant { lands: false, expiry: Expiry::EndOfTurn, ..PlayGrant::CAST }); } } } });
 });
g.end_sentence();
let mana_players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_9_0 = { let q: &dyn Query = g; (6i64).max(0) as u32 };
for player in mana_players_9 {
g.add_mana(player, Mana::R, mana_9_0);}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Chandra, Heart of Fire", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Chandra]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(5), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_0, "+1: Discard your hand, then exile the top three cards of your library. Until end of turn, you may play cards exiled this way.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_1, "+1: Chandra deals 2 damage to any target.").targets(ABILITY_1_TARGETS).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-9)), ability_2, "−9: Search your graveyard and library for any number of red instant and/or sorcery cards, exile them, then shuffle. You may cast them this turn. Add six {R}.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
