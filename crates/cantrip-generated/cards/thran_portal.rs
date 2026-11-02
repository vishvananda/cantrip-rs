// Thran Portal
// status: planned
// requires: activation cost with another cost
// This land enters tapped unless you control two or fewer other lands.
// As this land enters, choose a basic land type.
// This land is the chosen type in addition to its other types.
// Mana abilities of this land cost an additional 1 life to activate.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; !(((q.state().battlefield().iter().copied().filter(|&id| ((!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Land)) && Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) <= (2i64)) } {
entering.tapped = true;
}
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let Some(chosen_1) = cantrip_mechanics::chosen_type::choose_type_among(g, entering, vec![Subtype::Plains, Subtype::Island, Subtype::Swamp, Subtype::Mountain, Subtype::Forest])? else { return Ok(()) };
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if let Some(chosen) = q.state().chosen_subtype(q.linked_to(asker.source)) { chars.subtypes = chars.subtypes.with(chosen); } }
planned_item!("activation-cost-with-another-cost" => #[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Ability(permanent, ability, _) = costed else { return CostDelta::NONE; }; let def = q.activated_ability(ability); if true && (def.is_some_and(|d| d.is_mana_ability()) && (def.is_some_and(|d| d.from == ZoneKind::Battlefield) && Some(asker.source) == Some(permanent))) { CostDelta { also: Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(life(1)), ..CostDelta::NONE } } else { CostDelta::NONE } });
fn entering_all(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { ability_0_enters(g, entering)?; ability_1_enters(g, entering)?;  Ok(()) }
planned_item!("activation-cost-with-another-cost" => cost_changes: ability_3_0_rule);

pub static CARD: CardDef = CardDef { name: "Thran Portal", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[Subtype::Gate]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[sets_land_subtypes(cantrip_mechanics::statics::itself, ability_2_apply_0, "This land is the chosen type in addition to its other types.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(entering_all), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
