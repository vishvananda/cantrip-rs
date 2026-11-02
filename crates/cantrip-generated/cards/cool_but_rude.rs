// Cool but Rude
// status: ready
// (Gain the next level as a sorcery to add its ability.)
// Whenever you attack, you may discard a card. If you do, draw a card.
// {1}{R}: Level 2
// Whenever you discard a card, this Class deals 2 damage to each opponent.
// {1}{R}: Level 3
// When this Class becomes level 3, search your library for a card, put it into your hand, shuffle, then discard a card at random.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AttackersDeclared { player, count } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if *count > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_2 = Vec::new(); for player in players_2 { cards_2.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_2, true)? }); } let discarded_2 = g.discard_together(&cards_2);
let paid_1 = !discarded_2.clone().is_empty();
(paid_1, { let q: &dyn Query = g; (discarded_2).clone() }, { let q: &dyn Query = g; (discarded_2.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_level_2_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true && !*undefined && q.state().obj(asker.source).class_level >= 2 { return Some(Fired { subject, data }); } }
 None }
fn ability_1_level_2_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let amount_4 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_4 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_4 { let batch: Vec<(ObjectId, Target, u16)> = recipients_4.iter().map(|t| (source, *t, amount_4)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_level_3_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ClassLevel { object, level } = event { let id = *object; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) && *level == (3i64).max(0) as u8 && q.state().obj(asker.source).class_level >= 3 { return Some(Fired { subject, data }); } }
 None }
fn ability_1_level_3_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_5 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let shown_5 = cantrip_mechanics::library::searched_cards(g, owner_5); let can_5: Vec<ObjectId> = { let q: &dyn Query = g; shown_5.iter().copied().filter(|&id| true).collect() }; let found_5 = cantrip_mechanics::library::search(g, searcher_5, owner_5, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_5, &shown_5, can_5))?;
let objects_6 = { let q: &dyn Query = g; found_5.clone() }; let moved_6 = g.return_to_hand(&objects_6);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let amount_8 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
cantrip_mechanics::discard_at_random(g, player_8, amount_8 as usize);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Cool but Rude", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Class]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKERS_DECLARED.0), ability_0_event), ability_0, "Whenever you attack, you may discard a card. If you do, draw a card.").targets(&[]),cantrip_mechanics::class::level_two("{1}{R}", "{1}{R}: Level 2"),triggered(when_asked(EventMask(EventMask::DISCARDED.0), ability_1_level_2_0_event), ability_1_level_2_0, "Whenever a card is discarded, ").targets(&[]),cantrip_mechanics::class::level_three("{1}{R}", "{1}{R}: Level 3"),triggered(when_asked(EventMask(EventMask::CLASS_LEVEL.0), ability_1_level_3_0_event), ability_1_level_3_0, "When this Class becomes that level,  Return the chosen permanents to their owners' hands.  ").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
