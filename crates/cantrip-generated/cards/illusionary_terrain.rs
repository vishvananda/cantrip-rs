// Illusionary Terrain
// status: planned
// requires: a choice of several values
// requires: a land type chosen second
// requires: object filter NthChosen
// Cumulative upkeep {2} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)
// As this enchantment enters, choose two basic land types.
// Basic lands of the first chosen type are the second chosen type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let chosen_0 = planned_value!("a choice of several values", Subtype::Plains => cantrip_mechanics::choose::values(g, { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::BasicLandType, 2)?);
 Ok(()) }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).supertypes.basic && (*so_far).is(CardType::Land) && planned_value!("object filter NthChosen", false => q.nth_chosen_value(asker, 0).matches(affected))) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.set_land_subtypes(SubtypeSet::NONE.with(planned_value!("a land type chosen second", Subtype::Plains => q.nth_chosen_value(asker, 1).subtype()))); }

pub static CARD: CardDef = CardDef { name: "Illusionary Terrain", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::cumulative_upkeep!(cantrip_mechanics::cumulative_upkeep::Payment::Mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), "Cumulative upkeep")], statics: &[sets_land_subtypes(ability_2_affects, ability_2_apply_0, "Basic lands of the first chosen type are the second chosen type.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
