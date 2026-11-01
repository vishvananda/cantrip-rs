// Umori, the Collector
// status: planned
// requires: a choice of CardType
// requires: a choice of CardType kept on a permanent
// Companion — Each nonland card in your starting deck shares a card type. (If this card is your chosen companion, you may put it into your hand from outside the game for {3} as a sorcery.)
// As Umori enters, choose a card type.
// Spells you cast of the chosen type cost {1} less to cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_companion(q: &dyn Query, deck: &StartingDeck) -> bool { let those: Vec<ObjectId> = deck.cards.iter().copied().filter(|&id| { let asker = &Asker::new(id, deck.player); !(q.chars(id).is(CardType::Land)) }).collect(); those.first().is_none_or(|&first| q.chars(first).types.iter().any(|t| those.iter().all(|&id| q.chars(id).is(t)))) }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let chosen_0: CardType = planned_value!("a choice of CardType", CardType::Creature => cantrip_mechanics::choose::card_type(g, entering.controller, entering.object, CardType::ALL.to_vec())?);
planned!("a-choice-of-card-type" => entering.chosen_card_type = Some(chosen_0););
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && planned_value!("a choice of CardType kept on a permanent", None::<CardType> => q.state().chosen_card_type(q.linked_to(asker))).is_some_and(|chosen| q.chars(spell).is(chosen)) { CostDelta::less((1i64) as u8) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Umori, the Collector", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::B, Mana::G)),Some((Mana::B, Mana::G)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ooze]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, cost_changes: &[ability_2_0_rule], companion: Some(ability_0_companion), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
