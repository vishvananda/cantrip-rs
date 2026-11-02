// Dauntless Escort Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// Creatures you control have exalted. (Whenever a creature you control attacks alone, it gets +1/+1 until end of turn for each instance of exalted among permanents you control.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)); } }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 1, life: 1 }));

pub static CARD: CardDef = CardDef { name: "Dauntless Escort Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_0_affects, ability_0_apply_0, "Creatures you control have exalted.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(cantrip_mechanics::triggers::CREATURE_YOU_CONTROL_ATTACKS_ALONE, cantrip_mechanics::triggers::subject_gets_plus_one, "Exalted")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
