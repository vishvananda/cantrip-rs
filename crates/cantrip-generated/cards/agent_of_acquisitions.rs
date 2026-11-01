// Agent of Acquisitions
// status: planned
// requires: ability DraftRule
// Draft this card face up.
// Instead of drafting a card from a booster pack, you may draft each card in that booster pack, one at a time. If you do, turn this card face down and you can't draft cards for the rest of this draft round. (You may look at booster packs passed to you.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[DraftAction::DraftFaceUp]));
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[FaceUpDraftEffect::MayTurnThisDraftCardFaceDown_DraftEachCardInCurrentBoosterPackInsteadOfDraftingCardsThisRound]));

pub static CARD: CardDef = CardDef { name: "Agent of Acquisitions", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Construct]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
