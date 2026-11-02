// Central Elevator // Promising Stairs
// status: ready
// When you unlock this door, search your library for a Room card that doesn't have the same name as a Room you control, reveal it, put it into your hand, then shuffle.
// (You may cast either half. That door unlocks on the battlefield. As a sorcery, you may pay the mana cost of a locked door to unlock it.)
// At the beginning of your upkeep, surveil 1. You win the game if there are eight or more different names among unlocked doors of Rooms you control.
// (You may cast either half. That door unlocks on the battlefield. As a sorcery, you may pay the mana cost of a locked door to unlock it.)
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Central Elevator // Promising Stairs", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Room]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, split: Some((&left::CARD, &right::CARD)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&left::CARD,&right::CARD];
pub mod left {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::DoorUnlocked { permanent, door, player } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if *door == DOOR_LEFT && Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Room)) && !((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Room)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).iter().any(|&o| cantrip_mechanics::same_name(q, id, o))))).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_2 = { let q: &dyn Query = g; found_0.clone() }; let moved_2 = g.return_to_hand_revealed(&objects_2);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Central Elevator", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Room]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DOOR_UNLOCKED.0), ability_0_event), ability_0, "When you unlock this door, search your library for a Room card that doesn't have the same name as a Room you control, reveal it, put it into your hand, then shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
pub mod right {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::surveil(g, player_0, asker.source, amount_0)?;
g.end_sentence();
if { let q: &dyn Query = g; ({ let mut seen: Vec<&'static str> = Vec::new(); for id in q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Room)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() { for name in cantrip_mechanics::library::object_names(q, id) { if !seen.contains(&name) { seen.push(name); } } } seen.len() as i64 }) >= (8i64) } {
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.win_game(player); if g.state().is_over() { return Ok(()); }; }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Promising Stairs", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Room]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, surveil 1. You win the game if there are eight or more different names among unlocked doors of Rooms you control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
