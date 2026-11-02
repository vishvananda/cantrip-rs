// Soul Shatter
// status: ready
// Each opponent sacrifices a creature or planeswalker with the greatest mana value among creatures and planeswalkers they control.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut gathered_1: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)) && (i64::from(q.mana_value(id))) == ((q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).max().unwrap_or(0)))).collect::<Vec<ObjectId>>() }; let players_2 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_2: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_2 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_2 { let eligible: Vec<ObjectId> = objects_2.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_2.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_2.clone(), &[]))? }; for object in taken { picks_2.push((player, object)); } }
gathered_1.extend(picks_2.iter().copied());
    } 
let sacrificed_1 = g.sacrifice_each(&gathered_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Soul Shatter", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
