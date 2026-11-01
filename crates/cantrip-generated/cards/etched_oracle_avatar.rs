// Etched Oracle Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// You may pay {W}{U}{B}{R}{G} rather than pay the mana cost for spells that you cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId, player: PlayerId) -> bool { player == asker.controller && true }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 1, life: -4 }));

pub static CARD: CardDef = CardDef { name: "Etched Oracle Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_alternatives: &[SpellAlternative { text: "Pay {W}{U}{B}{R}{G}", applies: ability_0_0_rule, cost: InsteadOfManaCost::Mana(ManaCost { generic: 0, x: 0, symbols: [1, 1, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), flash: false }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
