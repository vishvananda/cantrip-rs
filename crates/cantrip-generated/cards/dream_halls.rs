// Dream Halls
// status: planned
// requires: alternate spell cost rule
// requires: object reference Scoped
// Rather than pay the mana cost for a spell, its controller may discard a card that shares a color with that spell.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn rule_cost_0_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { planned_value!("object reference Scoped", None::<ObjectId> => q.scoped_object(asker)).is_some_and(|o: ObjectId| q.chars(o).colors.intersects(q.chars(id).colors)) }
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
planned_item!("alternate-spell-cost-rule" => statics: player_rule(ability_0_0_applies, PlayerRule::SpellCost { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| true, change: CostModifier::Alternate { cost: PlayCost::Alternate(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(discard(1, rule_cost_0_1))), limit: None, during: None } }));

pub static CARD: CardDef = CardDef { name: "Dream Halls", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
