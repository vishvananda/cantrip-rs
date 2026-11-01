// Cloud Key
// status: planned
// requires: a choice of CardType
// requires: a choice of CardType kept on a permanent
// As this artifact enters, choose artifact, creature, enchantment, instant, or sorcery.
// Spells you cast of the chosen type cost {1} less to cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let chosen_0: CardType = planned_value!("a choice of CardType", CardType::Creature => cantrip_mechanics::choose::card_type(g, entering.controller, entering.object, vec![CardType::Artifact, CardType::Creature, CardType::Enchantment, CardType::Instant, CardType::Sorcery])?);
planned!("a-choice-of-card-type" => entering.chosen_card_type = Some(chosen_0););
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && planned_value!("a choice of CardType kept on a permanent", None::<CardType> => q.state().chosen_card_type(q.linked_to(asker))).is_some_and(|chosen| q.chars(spell).is(chosen)) { CostDelta::less((1i64) as u8) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Cloud Key", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, cost_changes: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
