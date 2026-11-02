// Call to the Void
// status: planned
// requires: a value chosen in secret
// requires: earlier objects Selected
// requires: operation RevealChoices
// Each player secretly chooses a creature they control and a creature they don't control. Then those choices are revealed. Destroy each creature chosen this way.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { planned!("a-value-chosen-in-secret" => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Object, ChoiceTerms { options: &[{ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }], except: &[], count: None, secret: true, unchosen: false, random: false, ordered: false, up_to: false })?);
    } 
g.end_sentence();
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { planned!("a-value-chosen-in-secret" => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Object, ChoiceTerms { options: &[{ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && ({ let other = Some(bound_0); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id)))).collect::<Vec<ObjectId>>() }], except: &[], count: None, secret: true, unchosen: false, random: false, ordered: false, up_to: false })?);
    } 
g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Object));
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (planned_value!("earlier objects Selected", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Selected))).contains(&id)).collect::<Vec<ObjectId>>() }; let destroyed_5 = g.destroy_all(&objects_5);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Call to the Void", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
