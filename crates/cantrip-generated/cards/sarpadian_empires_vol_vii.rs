// Sarpadian Empires, Vol. VII
// status: planned
// requires: a choice of Color outside its body
// requires: a choice of ColorAndCreatureType
// requires: a choice of CreatureType outside its body
// requires: condition ValueChosen
// requires: token of the chosen color
// As this artifact enters, choose white Citizen, blue Camarid, black Thrull, red Goblin, or green Saproling.
// {3}, {T}: Create a 1/1 creature token of the chosen color and type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); planned!("a-choice-of-color-and-creature-type" => cantrip_mechanics::choose::value(g, asker.source, { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() }, ChoiceKind::ColorAndCreatureType, 1, &[ColorAndCreatureType::ColorAndCreatureType(Color::White, CreatureType::Citizen), ColorAndCreatureType::ColorAndCreatureType(Color::Blue, CreatureType::Camarid), ColorAndCreatureType::ColorAndCreatureType(Color::Black, CreatureType::Thrull), ColorAndCreatureType::ColorAndCreatureType(Color::Red, CreatureType::Goblin), ColorAndCreatureType::ColorAndCreatureType(Color::Green, CreatureType::Saproling)])?);
 Ok(()) }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!().name, token_words: token!().token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (planned_value!("condition ValueChosen", false => q.state().chosen_value(q.linked(item), ChoiceKind::Color).is_some()) && planned_value!("condition ValueChosen", false => q.state().chosen_value(q.linked(item), ChoiceKind::CreatureType).is_some())) } {
let created_1: Vec<ObjectId> = planned_value!("token of the chosen color", Vec::<ObjectId>::new() => cantrip_mechanics::tokens::create(g, asker, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, &[({ let q: &dyn Query = g; (1i64).max(0) as u32 }, TokenRecipe::Filled(g.card_id(&token_2::CARD), TokenFill { color: Some({ let q: &dyn Query = g; planned_value!("a choice of Color outside its body", Color::White => q.state().chosen_color(q.linked(item)).expect("it chose one")) }), creature_type: Some({ let q: &dyn Query = g; planned_value!("a choice of CreatureType outside its body", Subtype::Plains => q.state().chosen_subtype(q.linked(item)).expect("it chose one")) }), power_toughness: None, abilities_of: None }))], Arrival { tapped: false, counters: &[], states: &[] })?);
g.end_sentence();
hoisted_3.extend(created_1.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sarpadian Empires, Vol. VII", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Book]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{3}, {T}: Create a 1/1 creature token of the chosen color and type.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
