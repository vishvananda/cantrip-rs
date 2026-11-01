// Brago's Favor
// status: planned
// requires: a choice of CardName outside its body
// requires: card type Conspiracy
// requires: engine: no HiddenAgenda
// requires: layout Conspiracy
// Hidden agenda (Start the game with this conspiracy face down in the command zone and secretly choose a card name. You may turn this conspiracy face up any time and reveal that name.)
// Spells with the chosen name you cast cost {1} less to cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && (planned_value!("a choice of CardName outside its body", None::<CardId> => q.state().chosen_card(q.linked_to(asker))).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, spell, chosen))) { CostDelta::less((1i64) as u8) } else { CostDelta::NONE } }
planned_item!("engine-no-hidden-agenda" => opening_hand: cantrip_mechanics::hidden_agenda::OPENING_HAND);
planned_item!("layout-conspiracy" => variant: Some(Variant::ConspiracyDraft));

pub static CARD: CardDef = CardDef { name: "Brago's Favor", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Conspiracy", CardType::Land => CardType::Conspiracy)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
