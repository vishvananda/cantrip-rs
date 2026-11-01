// Case of the Lost Witness
// status: planned
// requires: operation Conjure
// When this Case enters, conjure four cards named Fblthp, the Lost into your library, then shuffle. Draw a card.
// To solve — You control a legendary Homunculus.
// Solved — You may look at the top card of your library any time and you may play lands and cast spells from the top of your library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_0: u32 = { let q: &dyn Query = g; (4i64).max(0) as u32 }; let mut conjured_0: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_0.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Fblthp, the Lost"), count_0, ConjureTo::Library(ConjureSpot::Shuffled))?)); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }
fn ability_1_to_solve(q: &dyn Query, asker: &Asker) -> bool { q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).supertypes.legendary && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Homunculus)))).count() >= 1 }
#[allow(unused_variables)] fn ability_1_solved_0_0_0_while(q: &dyn Query, asker: &Asker) -> bool { q.is_solved(asker.source) }
#[allow(unused_variables)] fn ability_1_solved_0_0_1_while(q: &dyn Query, asker: &Asker) -> bool { q.is_solved(asker.source) }
#[allow(unused_variables)] fn ability_1_solved_0_0_2_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { true }
#[allow(unused_variables)] fn ability_1_solved_0_0_2_while(q: &dyn Query, asker: &Asker) -> bool { q.is_solved(asker.source) }

pub static CARD: CardDef = CardDef { name: "Case of the Lost Witness", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Case]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this Case enters, conjure four cards named Fblthp, the Lost into your library, then shuffle. Draw a card.").targets(&[]),cantrip_mechanics::solved::to_solve(ability_1_to_solve, "To solve (If unsolved, solve at the beginning of your end step.)")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::While(ability_1_solved_0_0_0_while, &[PlayRule::LookAtLibraryTop]),PlayRule::While(ability_1_solved_0_0_1_while, &[PlayRule::LibraryTopLands]),PlayRule::While(ability_1_solved_0_0_2_while, &[PlayRule::LibraryTopSpells(ability_1_solved_0_0_2_rule, None)])], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
