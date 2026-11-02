// Scheming Fence
// status: planned
// requires: static change GainAbilitiesOf
// As this creature enters, you may choose a nonland permanent.
// Activated abilities of the chosen permanent can't be activated.
// This creature has all activated abilities of the chosen permanent except for loyalty abilities. You may spend mana as though it were mana of any color to activate those abilities.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); {
let permanents = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; cantrip_mechanics::choose::object_as_enters(g, entering, permanents, cantrip_mechanics::select::Selection::of(count, &[]))?;

}
 Ok(()) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); q.state().chosen_object(q.linked_to(asker.source)) == Some(affected) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, object: ObjectId, ability: &ActivatedDef) -> bool { !(q.state().obj(object).zone.is(ZoneKind::Battlefield) && q.state().chosen_object(q.linked_to(asker)) == Some(object)) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); planned_value!("static change GainAbilitiesOf", () => chars.gain_abilities_of(AbilitiesOf { objects: { let ids: Vec<ObjectId> = q.state().chosen_object(q.linked_to(asker.source)).into_iter().collect::<Vec<ObjectId>>(); ids }, kinds: &[ActivatedAbilities::And(ActivatedAbilities::AbilityOfPermanent(q.state().chosen_object(q.linked_to(asker.source))), ActivatedAbilities::NonLoyaltyAbility)], once_each_turn: false, spend_as: Some(SpendAs::AnyColor) })); }

pub static CARD: CardDef = CardDef { name: "Scheming Fence", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Citizen]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_2_apply_0, "This creature has all activated abilities of the chosen permanent except for loyalty abilities. You may spend mana as though it were mana of any color to activate those abilities.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, activation_rules: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
