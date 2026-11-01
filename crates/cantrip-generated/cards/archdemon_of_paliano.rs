// Archdemon of Paliano
// status: planned
// requires: ability DraftRule
// Draft this card face up.
// As long as this card is face up during the draft, you can't look at booster packs and must draft cards at random. After you draft three cards this way, turn this card face down. (You may look at cards as you draft them.)
// Flying
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[DraftAction::DraftFaceUp]));
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[FaceUpDraftEffect::DraftCardsAtRandomUntilNumberCardsHaveBeenDrafted_TurnThisDraftCardFaceDown(3i64)]));

pub static CARD: CardDef = CardDef { name: "Archdemon of Paliano", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
