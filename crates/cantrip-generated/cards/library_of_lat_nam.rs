// Library of Lat-Nam
// status: planned
// requires: modes chosen by another player
// An opponent chooses one —
// • You draw three cards at the beginning of the next turn's upkeep.
// • You search your library for a card, put that card into your hand, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
fn delayed_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_2 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
g.delayed_trigger(DelayedWhen::Step { step: Step::Upkeep, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None);
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_3 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let shown_3 = cantrip_mechanics::library::searched_cards(g, owner_3); let can_3: Vec<ObjectId> = { let q: &dyn Query = g; shown_3.iter().copied().filter(|&id| true).collect() }; let found_3 = cantrip_mechanics::library::search(g, searcher_3, owner_3, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_3, &shown_3, can_3))?;
let objects_4 = { let q: &dyn Query = g; found_3.clone() }; let moved_4 = g.return_to_hand(&objects_4);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = planned_value!("modes chosen by another player", modes(&[mode(ability_0_mode_0, "You draw three cards at the beginning of the next turn's upkeep.").targets(&[]),mode(ability_0_mode_1, "You search your library for a card, put that card into your hand, then shuffle.").targets(&[])]).choose(1, 1) => modes(&[mode(ability_0_mode_0, "You draw three cards at the beginning of the next turn's upkeep.").targets(&[]),mode(ability_0_mode_1, "You search your library for a card, put that card into your hand, then shuffle.").targets(&[])]).choose(1, 1).chosen_by(|q: &dyn Query, asker: &Asker| Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()));

pub static CARD: CardDef = CardDef { name: "Library of Lat-Nam", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_1, "An opponent chooses one — You draw three cards at the beginning of the next turn's upkeep; or You search your library for a card, put that card into your hand, then shuffle.")], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
