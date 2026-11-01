// Spire Phantasm
// status: planned
// requires: ability DraftRule
// requires: condition NotedForDraft
// Reveal this card as you draft it. The next time a player drafts a card from this booster pack, guess that card's name. Then that player reveals the drafted card.
// Flying
// When this creature enters, if you guessed correctly for a card named Spire Phantasm, draw a card.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { planned_value!("condition NotedForDraft", false => Some(asker.controller).is_some_and(|p: PlayerId| q.draft_notes(p, DraftNoteKind::Guessed, "Spire Phantasm"))) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[DraftAction::RevealThisDraftedCard, DraftAction::GuessNameOfNextCardAPlayerDraftsFromThisPackAndTheyRevealThatCard]));

pub static CARD: CardDef = CardDef { name: "Spire Phantasm", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Gargoyle,Subtype::Illusion]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this creature enters, if you guessed correctly for a card named Spire Phantasm, draw a card.").targets(&[]).condition(ability_2_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
