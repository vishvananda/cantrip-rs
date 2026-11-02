// Superior Spider-Man
// status: ready
// Mind Swap — You may have Superior Spider-Man enter as a copy of any creature card in a graveyard, except his name is Superior Spider-Man and he's a 4/4 Spider Human Hero in addition to his other types. When you do, exile that card.
#[allow(unused_imports)] use cantrip_api::*;
fn reflexive_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); const ASK_0: Question = Question::new("CopyAsEnters", "enter as a copy?", "You choose what it copies next. If you don't, it enters as itself", "Choose a copy", "Enter as itself"); let candidates_0: Vec<ObjectId> = ({ let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| id != entering.object).collect(); if let Some(chosen_0) = cantrip_mechanics::copy::choose_copy_of_one_of(g, entering, &candidates_0, &ASK_0)? {
entering.copy = { let q: &dyn Query = g; Some(chosen_0).map(|original: ObjectId| q.copiable_values(original).named("Superior Spider-Man").sized(4, 4).with_subtype(q.words(asker.source).subtype(Subtype::Spider)).with_subtype(q.words(asker.source).subtype(Subtype::Human)).with_subtype(q.words(asker.source).subtype(Subtype::Hero))) }; let picked_0 = vec![chosen_0];
if !picked_0.clone().is_empty() && true { g.reflexive_trigger(asker.source, asker.controller, AbilityRef::delayed({ let q: &dyn Query = g; { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) } }, 0), { let q: &dyn Query = g; (picked_0.clone()).get(0 as usize).copied() }.map_or(Target::None, Target::Object)); }
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Superior Spider-Man", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spider,Subtype::Human,Subtype::Hero]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_2, "When you do, exile that card.").targets(&[])], enters: None, modal: None, enchant: None, enters_as_copy: Some(ability_0_enters), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
