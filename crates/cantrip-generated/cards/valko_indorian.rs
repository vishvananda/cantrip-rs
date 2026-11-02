// Valko Indorian
// status: planned
// requires: a deck rule a commander sets
// Rulebreaker — A deck with this commander can have Phyrexian cards of any color identity and any basic land cards.
// Phyrexian creatures you control have menace and lifelink.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Phyrexian)) && (*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Menace);
chars.keywords = chars.keywords.with(Keyword::Lifelink); }
planned_item!("a-deck-rule-a-commander-sets" => deck_rules: DeckRule::CanHaveAnyCards);
planned_item!("a-deck-rule-a-commander-sets" => deck_rules: DeckRule::CanHaveAnyBasicLandCards);

pub static CARD: CardDef = CardDef { name: "Valko Indorian", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_2_affects, ability_2_apply_0, "Phyrexian creatures you control have menace and lifelink.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
