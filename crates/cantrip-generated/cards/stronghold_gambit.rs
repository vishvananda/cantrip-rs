// Stronghold Gambit
// status: planned
// requires: a value chosen in secret
// requires: condition HadExtreme
// requires: earlier objects Revealed
// requires: operation RevealChoices
// requires: per-player ChangeZone
// requires: per-player RevealChoices
// Each player chooses a card in their hand. Then each player reveals their chosen card. The owner of each creature card revealed this way with the lowest mana value puts it onto the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { planned!("a-value-chosen-in-secret" => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Object, ChoiceTerms { options: &[|q: &dyn Query, asker: &Asker, id: ObjectId| true, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }], except: &[], count: None, secret: true, unchosen: false, random: false, ordered: false, up_to: false })?);
    } 
g.end_sentence();
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
planned!("per-player-reveal-choices" => g.begin_batch(););
 for (index_2, _) in players_2.iter().copied().enumerate() { planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Object));
    } 
planned!("per-player-reveal-choices" => g.end_batch(););
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition HadExtreme", false => Some(bound_0).is_some_and(|p: PlayerId| q.had_extreme(item, p, EarlierObjects::Revealed, |id: ObjectId| q.chars(id).is(CardType::Creature), false)))).collect::<Vec<PlayerId>>() };
planned!("per-player-change-zone" => g.begin_batch(););
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
 for (index_4, _) in players_4.iter().copied().enumerate() { let objects_6 = ({ let q: &dyn Query = g; (planned_value!("earlier objects Revealed", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Revealed))).first().copied().into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_6 = { let rows_6: Vec<(ObjectId, PlayerId, bool)> = objects_6.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_6 = g.put_all_onto_battlefield_as(&rows_6)?; came_6.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
 each_7[index_4] = moved_6.clone(); all_7.extend(each_7[index_4].iter().copied());
each_8[index_4] = { let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_8.extend(each_8[index_4].iter().copied());
   } 
planned!("per-player-change-zone" => g.end_batch(););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Stronghold Gambit", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
