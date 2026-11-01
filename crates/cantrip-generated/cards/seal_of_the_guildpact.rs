// Seal of the Guildpact
// status: planned
// requires: a choice of several values
// requires: numeric expression ChosenColorsOf
// As this artifact enters, choose two colors.
// Each spell you cast costs {1} less to cast for each of the chosen colors it is.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let chosen_0 = planned_value!("a choice of several values", Color::White => cantrip_mechanics::choose::values(g, { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Color, 2)?);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && true { CostDelta::less((planned_value!("numeric expression ChosenColorsOf", 0i64 => Some(asker.source).map_or(0i64, |o: ObjectId| q.state().chosen_colors(o).iter().count() as i64))).clamp(0, 255) as u8) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Seal of the Guildpact", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, cost_changes: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
