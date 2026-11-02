// Fortunate Few
// status: planned
// requires: operation ChooseValueWith
// Choose a nonland permanent you don't control, then each other player chooses a nonland permanent they don't control that hasn't been chosen this way. Destroy all other nonland permanents.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_0 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_0.iter().position(|&p| Some(p) == first_0) { players_0.rotate_left(at); }
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
let mut each_chosen_1 = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let chosen_1 = planned_value!("operation ChooseValueWith", Vec::<ObjectId>::new() => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Object, ChoiceTerms { options: &[{ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && ({ let other = Some(bound_0); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id)))).collect::<Vec<ObjectId>>() }], except: &[], count: None, secret: false, unchosen: true, random: false, ordered: false, up_to: false })?);
 each_2[index_0] = chosen_1.clone(); all_2.extend(each_2[index_0].iter().copied());
each_chosen_1.push(chosen_1);
   } 
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!((all_2.clone()).contains(&id)) && !(q.chars(id).is(CardType::Land)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)))).collect::<Vec<ObjectId>>() }; let destroyed_3 = g.destroy_all(&objects_3);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Fortunate Few", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
