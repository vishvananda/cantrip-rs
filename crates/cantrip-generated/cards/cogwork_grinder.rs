// Cogwork Grinder
// status: planned
// requires: ability DraftRule
// requires: numeric expression DraftNoted
// Draft this card face up.
// As you draft a card, you may remove it from the draft face down. (Those cards aren't in your card pool.)
// This creature enters with X +1/+1 counters on it, where X is the number of cards you removed from the draft with cards named Cogwork Grinder.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_2_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (planned_value!("numeric expression DraftNoted", 0i64 => q.draft_notes(asker.controller, DraftNoted { note: DraftNote { kind: Removed, player: ControllerOf(EventSubject), card: "Cogwork Grinder" }, highest: false }))).clamp(0, 32767) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
 Ok(()) }
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[DraftAction::DraftFaceUp]));
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[FaceUpDraftEffect::MayRemoveCardsFromDraftFaceDown]));

pub static CARD: CardDef = CardDef { name: "Cogwork Grinder", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Construct]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_2_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
