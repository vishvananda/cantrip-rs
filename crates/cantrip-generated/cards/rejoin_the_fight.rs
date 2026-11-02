// Rejoin the Fight
// status: planned
// requires: operation ChooseValueWith
// Mill three cards. Then starting with the next opponent in turn order, each opponent chooses a creature card in your graveyard that hasn't been chosen. Return each card chosen this way to the battlefield under your control.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_1.sort_by_key(|p| order.iter().position(|o| o == p)); }
let first_1 = { let q: &dyn Query = g; Some(asker.controller).and_then(|p: PlayerId| q.state().turn_order_from(p).filter(|&next| next != p).find(|&next| q.state().opponents(p).any(|o| o == next))) }; if let Some(at) = players_1.iter().position(|&p| Some(p) == first_1) { players_1.rotate_left(at); }
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
let mut each_chosen_2 = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let chosen_2 = planned_value!("operation ChooseValueWith", Vec::<ObjectId>::new() => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Object, ChoiceTerms { options: &[|q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)))], except: &[], count: None, secret: false, unchosen: true, random: false, ordered: false, up_to: false })?);
 each_3[index_1] = chosen_2.clone(); all_3.extend(each_3[index_1].iter().copied());
each_chosen_2.push(chosen_2);
   } 
g.end_sentence();
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_4 = ({ let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (all_3.clone()).contains(&id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let rows_4: Vec<(ObjectId, PlayerId, bool)> = objects_4.iter().map(|&o| (o, player_4, false)).collect(); let came_4 = g.put_all_onto_battlefield_as(&rows_4)?; came_4.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Rejoin the Fight", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
