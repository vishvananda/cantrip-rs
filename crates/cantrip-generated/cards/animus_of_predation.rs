// Animus of Predation
// status: planned
// requires: ability DraftRule
// requires: object collection OutsideTheGame
// requires: object filter DraftNoted
// requires: the keywords of other objects
// Draft this card face up.
// As you draft a card, you may remove it from the draft face up. (It isn't in your card pool.)
// If you removed a creature card with flying from the draft with cards named Animus of Predation, this creature has flying. The same is true for first strike, double strike, deathtouch, haste, hexproof, indestructible, lifelink, menace, reach, and vigilance.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let ids = (planned_value!("object collection OutsideTheGame", Vec::<ObjectId>::new() => q.outside_the_game(asker.controller))).into_iter().filter(|&id| (q.chars(id).is(CardType::Creature) && planned_value!("object filter DraftNoted", false => q.draft_noted_card(id, "DraftNoted(DraftNote { kind: Removed, player: AbilityController, card: \"Animus of Predation\" })")))).collect::<Vec<ObjectId>>(); planned_value!("the keywords of other objects", () => cantrip_mechanics::statics::grant_keywords_of(q, &ids, &[Keyword::Flying, Keyword::FirstStrike, Keyword::DoubleStrike, Keyword::Deathtouch, Keyword::Haste, Keyword::AnyHexproof, Keyword::Indestructible, Keyword::Lifelink, Keyword::Menace, Keyword::Reach, Keyword::Vigilance], chars)); }
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[DraftAction::DraftFaceUp]));
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[FaceUpDraftEffect::MayRemoveCardsFromDraftFaceUp]));

pub static CARD: CardDef = CardDef { name: "Animus of Predation", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_2_apply_0, "If you removed a creature card with flying from the draft with cards named Animus of Predation, this creature has flying. The same is true for first strike, double strike, deathtouch, haste, hexproof, indestructible, lifelink, menace, reach, and vigilance.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
