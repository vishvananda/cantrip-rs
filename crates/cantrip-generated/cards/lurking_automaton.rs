// Lurking Automaton
// status: planned
// requires: ability DraftRule
// requires: numeric expression DraftNoted
// Reveal this card as you draft it and note how many cards you've drafted this draft round, including this card.
// This creature enters with X +1/+1 counters on it, where X is the highest number you noted for cards named Lurking Automaton.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (planned_value!("numeric expression DraftNoted", 0i64 => q.draft_notes(asker.controller, DraftNoted { note: DraftNote { kind: Number, player: ControllerOf(EventSubject), card: "Lurking Automaton" }, highest: true }))).min(i16::MAX.into()) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
 Ok(()) }
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[DraftAction::RevealThisDraftedCard, DraftAction::NoteNumberOfCardsDraftedThisRound]));

pub static CARD: CardDef = CardDef { name: "Lurking Automaton", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Construct]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
