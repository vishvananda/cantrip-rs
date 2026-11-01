// Hatchery Sliver
// status: planned
// requires: engine: no cost copies a mana cost
// requires: player rule SpellsHaveAbilities
// Replicate {1}{G} (When you cast this spell, copy it for each time you paid its replicate cost.)
// Each Sliver spell you cast has replicate. The replicate cost is equal to its mana cost. (A copy of a permanent spell becomes a token.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-spells-have-abilities" => statics: player_rule(ability_1_0_applies, PlayerRule::SpellsHave(SpellGrant { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Sliver)), from: None, nth_each_turn: None, grants: &[SpellEffect::Replicate(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no cost copies a mana cost", Cost::NONE => Cost { own_mana_cost: true, ..Cost::NONE })))] })));

pub static CARD: CardDef = CardDef { name: "Hatchery Sliver", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Sliver]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::replicate::trigger("Replicate")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, kicker: multikicker(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
