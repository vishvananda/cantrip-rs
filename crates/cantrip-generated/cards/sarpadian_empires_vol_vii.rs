// Sarpadian Empires, Vol. VII
// status: ready
// As this artifact enters, choose white Citizen, blue Camarid, black Thrull, red Goblin, or green Saproling.
// {3}, {T}: Create a 1/1 creature token of the chosen color and type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..5)?; let chosen_0: (Color, Subtype) = [(Color::White, Subtype::Citizen), (Color::Blue, Subtype::Camarid), (Color::Black, Subtype::Thrull), (Color::Red, Subtype::Goblin), (Color::Green, Subtype::Saproling)][usize::from(entering.chosen_option.map_or(0, |chosen| chosen.index))]; entering.chosen_color = Some(chosen_0.0); entering.chosen_subtype = Some(chosen_0.1);
 Ok(()) }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!().name, token_words: token!().token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (q.state().chosen_color(q.linked(item)).is_some() && q.state().chosen_subtype(q.linked(item)).is_some()) } {
let kind_1: Subtype = { let q: &dyn Query = g; q.state().chosen_subtype(q.linked(item)).expect("it chose one") };
let values_1 = Copiable::of(g.card_id(&token_2::CARD)).colored(ColorSet::of({ let q: &dyn Query = g; q.state().chosen_color(q.linked(item)).expect("it chose one") })).with_subtype(kind_1).named(kind_1.name()); let mut created_1: Vec<ObjectId> = Vec::new(); let count_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { created_1.extend(g.create_token_copies_of(values_1, player, count_1)?); }
g.end_sentence();
hoisted_3.extend(created_1.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sarpadian Empires, Vol. VII", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Book]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{3}, {T}: Create a 1/1 creature token of the chosen color and type.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, labels: &["white Citizen", "blue Camarid", "black Thrull", "red Goblin", "green Saproling"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
