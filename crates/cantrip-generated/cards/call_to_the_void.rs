// Call to the Void
// status: planned
// requires: a value chosen in secret
// requires: operation RevealChoices
// Each player secretly chooses a creature they control and a creature they don't control. Then those choices are revealed. Destroy each creature chosen this way.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
let mut each_chosen_1 = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let chosen_1 = planned_value!("a value chosen in secret", Vec::<ObjectId>::new() => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Object, ChoiceTerms { options: &[{ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }], except: &[], count: None, secret: true, unchosen: false, random: false, ordered: false, up_to: false })?);
 each_2[index_0] = chosen_1.clone(); all_2.extend(each_2[index_0].iter().copied());
each_chosen_1.push(chosen_1);
   } 
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
let mut each_chosen_4 = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let chosen_4 = planned_value!("a value chosen in secret", Vec::<ObjectId>::new() => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Object, ChoiceTerms { options: &[{ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && ({ let other = Some(bound_0); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id)))).collect::<Vec<ObjectId>>() }], except: &[], count: None, secret: true, unchosen: false, random: false, ordered: false, up_to: false })?);
 each_5[index_3] = chosen_4.clone(); all_5.extend(each_5[index_3].iter().copied());
each_chosen_4.push(chosen_4);
   } 
g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Object));
g.end_sentence();
let objects_7 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (all_5.clone()).contains(&id)).collect::<Vec<ObjectId>>() }; let destroyed_7 = g.destroy_all(&objects_7);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Call to the Void", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
