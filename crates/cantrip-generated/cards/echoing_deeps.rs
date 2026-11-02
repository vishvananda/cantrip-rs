// Echoing Deeps
// status: ready
// You may have this land enter tapped as a copy of any land card in a graveyard, except it's a Cave in addition to its other types.
// {T}: Add {C}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); const ASK_0: Question = Question::new("CopyAsEnters", "enter as a copy?", "You choose what it copies next. If you don't, it enters as itself", "Choose a copy", "Enter as itself"); let candidates_0: Vec<ObjectId> = ({ let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| id != entering.object).collect(); if let Some(chosen_0) = cantrip_mechanics::copy::choose_copy_of_one_of(g, entering, &candidates_0, &ASK_0)? {
entering.copy = { let q: &dyn Query = g; Some(chosen_0).map(|original: ObjectId| q.copiable_values(original).with_subtype(q.words(asker.source).subtype(Subtype::Cave))) }; let picked_0 = vec![chosen_0];
entering.tapped = true;
}
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::C, mana_2_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Echoing Deeps", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[Subtype::Cave]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_1_mana) }, "{T}: Add {C}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, enters_as_copy: Some(ability_0_enters), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
