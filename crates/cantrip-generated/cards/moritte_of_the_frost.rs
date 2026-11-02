// Moritte of the Frost
// status: planned
// requires: copy exception AddSupertypes
// Changeling (This card is every creature type.)
// You may have Moritte enter as a copy of a permanent you control, except it's legendary and snow in addition to its other types and, if it's a creature, it enters with two additional +1/+1 counters on it and has changeling.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); const ASK_0: Question = Question::new("CopyAsEnters", "enter as a copy?", "You choose what it copies next. If you don't, it enters as itself", "Choose a copy", "Enter as itself"); let candidates_0: Vec<ObjectId> = ({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)) == Some(q.controller(id))).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| id != entering.object).collect(); if let Some(chosen_0) = cantrip_mechanics::copy::choose_copy_of_one_of(g, entering, &candidates_0, &ASK_0)? {
entering.copy = { let q: &dyn Query = g; Some(chosen_0).map(|original: ObjectId| planned_value!("copy exception AddSupertypes", q.copiable_values(original) => q.copiable_values(original).with_supertypes(&[Supertype::Legendary, Supertype::Snow]))) }; let picked_0 = vec![chosen_0];
if { let q: &dyn Query = g; (((picked_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) } {
let more = { let q: &dyn Query = g; (2i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
entering.copy = entering.copy.map(|values| values.with_keyword(Keyword::Changeling));
}
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Moritte of the Frost", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter]), supertypes: Supertypes { basic: false, legendary: true, snow: true }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Changeling]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, enters_as_copy: Some(ability_1_enters), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
