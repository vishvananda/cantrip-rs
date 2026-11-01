// Case of the Stashed Skeleton
// status: ready
// When this Case enters, create a 2/1 black Skeleton creature token and suspect it. (It has menace and can't block.)
// To solve — You control no suspected Skeletons. (If unsolved, solve at the beginning of your end step.)
// Solved — {1}{B}, Sacrifice this Case: Search your library for a card, put it into your hand, then shuffle. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Skeleton).name, token_words: token!(Skeleton).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Skeleton]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
for bound_4294967293 in { let q: &dyn Query = g; created_0.clone() } {
if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967293).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.suspect(object); }
}
g.end_sentence();
 Ok(()) }
fn ability_1_to_solve(q: &dyn Query, asker: &Asker) -> bool { !(q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && (q.state().effects.iter().any(|e| e.affected == id && e.kind == EffectKind::Suspected) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Skeleton)))).count() >= 1) }
fn ability_1_solved_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_4 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let shown_4 = cantrip_mechanics::library::searched_cards(g, owner_4); let can_4: Vec<ObjectId> = { let q: &dyn Query = g; shown_4.iter().copied().filter(|&id| true).collect() }; let found_4 = cantrip_mechanics::library::search(g, searcher_4, owner_4, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_4, &shown_4, can_4))?;
let objects_5 = { let q: &dyn Query = g; found_4.clone() }; let moved_5 = g.return_to_hand(&objects_5);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn ability_1_solved_0_if(q: &dyn Query, asker: &Asker) -> bool { q.is_solved(asker.source) }

pub static CARD: CardDef = CardDef { name: "Case of the Stashed Skeleton", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Case]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this Case enters, create a 2/1 black Skeleton creature token and suspect it.").targets(&[]),cantrip_mechanics::solved::to_solve(ability_1_to_solve, "To solve (If unsolved, solve at the beginning of your end step.)"),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS), ability_1_solved_0, "To solve — You control no suspected Skeletons.").targets(&[]).sorcery_speed().condition(ability_1_solved_0_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
