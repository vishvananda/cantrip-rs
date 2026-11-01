// Vizier of Many Faces
// status: planned
// requires: object state Embalmed
// You may have this creature enter as a copy of any creature on the battlefield, except if this creature was embalmed, the token has no mana cost, it's white, and it's a Zombie in addition to its other types.
// Embalm {3}{U}{U}
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; Target::Object(asker.source).object().is_some_and(|candidate: ObjectId| planned_value!("object state Embalmed", false => q.state().obj(candidate).marks.embalmed)) } {
const ASK_0: Question = Question::new("CopyAsEnters", "enter as a copy?", "You choose what it copies next. If you don't, it enters as itself", "Choose a copy", "Enter as itself"); let candidates_0: Vec<ObjectId> = ({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| id != entering.object).collect(); if let Some(chosen_0) = cantrip_mechanics::copy::choose_copy_of_one_of(g, entering, &candidates_0, &ASK_0)? {
entering.copy = { let q: &dyn Query = g; Some(chosen_0).map(|original: ObjectId| q.copiable_values(original).costless().colored(ColorSet::NONE.with(Color::White)).with_subtype(Subtype::Zombie)) }; let picked_0 = vec![chosen_0];
}
} else {
const ASK_1: Question = Question::new("CopyAsEnters", "enter as a copy?", "You choose what it copies next. If you don't, it enters as itself", "Choose a copy", "Enter as itself"); let candidates_1: Vec<ObjectId> = ({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| id != entering.object).collect(); if let Some(chosen_1) = cantrip_mechanics::copy::choose_copy_of_one_of(g, entering, &candidates_1, &ASK_1)? {
entering.copy = { let q: &dyn Query = g; Some(chosen_1).map(|original: ObjectId| q.copiable_values(original)) }; let picked_1 = vec![chosen_1];
}
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Vizier of Many Faces", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::embalm::ability(ManaCost { generic: 3, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Embalm {3}{U}{U}")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, enters_as_copy: Some(ability_0_enters), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
