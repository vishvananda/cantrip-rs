// Volrath's Laboratory
// status: ready
// As this artifact enters, choose a color and a creature type.
// {5}, {T}: Create a 2/2 creature token of the chosen color and type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let Answer::Colors(colors_0) = g.ask(player_0, RequestKind::ChooseColors { source: entering.object, choices: ColorSet(31), count: 1, purpose: ColorUse::Any })? else { return Ok(()) }; let chosen_0 = colors_0[0];
entering.chosen_color = Some(chosen_0);
let Some(chosen_1) = cantrip_mechanics::chosen_type::choose_creature_type(g, entering)? else { return Ok(()) };
 Ok(()) }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!().name, token_words: token!().token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (q.state().chosen_color(q.linked(item)).is_some() && q.state().chosen_subtype(q.linked(item)).is_some()) } {
let kind_2: Subtype = { let q: &dyn Query = g; q.state().chosen_subtype(q.linked(item)).expect("it chose one") };
let values_2 = Copiable::of(g.card_id(&token_3::CARD)).colored(ColorSet::of({ let q: &dyn Query = g; q.state().chosen_color(q.linked(item)).expect("it chose one") })).with_subtype(kind_2).named(kind_2.name()); let mut created_2: Vec<ObjectId> = Vec::new(); let count_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { created_2.extend(g.create_token_copies_of(values_2, player, count_2)?); }
g.end_sentence();
hoisted_4.extend(created_2.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Volrath's Laboratory", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{5}, {T}: Create a 2/2 creature token of the chosen color and type.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD];
