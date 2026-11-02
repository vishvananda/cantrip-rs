// Glacian, Powerstone Engineer
// status: planned
// requires: tapping X permanents as a cost
// {T}, Tap X untapped artifacts you control: Look at the top X cards of your library. Put one of those cards into your hand and the rest into your graveyard.
// Partner (You can have two commanders if both have partner.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_0 = { let q: &dyn Query = g; (i64::from(item.x)) as usize }; cantrip_mechanics::library::look_at_top_take_one_rest_to_graveyard(g, player_0, asker.source, depth_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(id))) }

pub static CARD: CardDef = CardDef { name: "Glacian, Powerstone Engineer", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(planned_value!("tapping X permanents as a cost", Cost::NONE => Cost { variable: Some(VariableCost::TapX(ability_0_1)), ..Cost::NONE })), ability_0, "{T}, Tap X untapped artifacts you control: Look at the top X cards of your library. Put one of those cards into your hand and the rest into your graveyard.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
