// Serra's Emissary
// status: planned
// requires: a choice of CardType
// requires: a choice of CardType kept on a permanent
// requires: engine: no protection from this quality
// requires: player keyword Protection
// Flying
// As this creature enters, choose a card type.
// You and creatures you control have protection from the chosen card type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let chosen_0: CardType = planned_value!("a choice of CardType", CardType::Creature => cantrip_mechanics::choose::card_type(g, entering.controller, entering.object, CardType::ALL.to_vec())?);
planned!("a-choice-of-card-type" => entering.chosen_card_type = Some(chosen_0););
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_3_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_3_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); planned_value!("engine: no protection from this quality", () => chars.protection_from.push(ProtectionQuality::Matching(|q, asker, id| planned_value!("a choice of CardType kept on a permanent", None::<CardType> => q.state().chosen_card_type(q.linked_to(asker.source))).is_some_and(|chosen| q.chars(id).is(chosen))))); }
planned_item!("player-keyword-protection" => statics: player_rule(ability_2_0_applies, PlayerRule::Protection(Protection::Objects(|q: &dyn Query, asker: &Asker, id: ObjectId| planned_value!("a choice of CardType kept on a permanent", None::<CardType> => q.state().chosen_card_type(q.linked_to(asker))).is_some_and(|chosen| q.chars(id).is(chosen))))));

pub static CARD: CardDef = CardDef { name: "Serra's Emissary", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [3, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_3_affects, ability_3_apply_0, "You and creatures you control have protection from the chosen card type.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
