// Kentaro, the Smiling Cat
// status: planned
// requires: a cost reading the card it is paid for
// requires: alternate spell cost rule
// requires: cost part ManaWithX
// requires: engine: no Bushido
// Bushido 1 (Whenever this creature blocks or becomes blocked, it gets +1/+1 until end of turn.)
// You may pay {X} rather than pay the mana cost for Samurai spells you cast, where X is that spell's mana value.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("engine-no-bushido" => abilities: cantrip_mechanics::bushido::trigger::<1>("Bushido 1"));
planned_item!("alternate-spell-cost-rule" => statics: player_rule(ability_1_0_applies, PlayerRule::SpellCost { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Samurai)), change: CostModifier::Alternate { cost: PlayCost::Alternate(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost part ManaWithX", Cost::NONE => Cost::mana_with_x_of_card(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, |q: &dyn Query, asker: &Asker, card: ObjectId| (planned_value!("a cost reading the card it is paid for", Vec::<ObjectId>::new() => vec![card])).into_iter().next().map_or(0i64, |id: ObjectId| i64::from(q.mana_value(id))))))), limit: None, during: None } }));

pub static CARD: CardDef = CardDef { name: "Kentaro, the Smiling Cat", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Samurai]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
