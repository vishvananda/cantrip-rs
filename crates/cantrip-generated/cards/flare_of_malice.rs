// Flare of Malice
// status: ready
// You may sacrifice a nontoken black creature rather than pay this spell's mana cost.
// Each opponent sacrifices a creature or planeswalker with the greatest mana value among creatures and planeswalkers they control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(q.state().obj(id).token) && q.chars(id).colors.has(q.words(asker.source).color(Color::Black)) && q.chars(id).is(CardType::Creature)) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut gathered_2: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let objects_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)) && (i64::from(q.mana_value(id))) == ((q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).max().unwrap_or(0)))).collect::<Vec<ObjectId>>() }; let players_3 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_3 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_3 { let eligible: Vec<ObjectId> = objects_3.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_3.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_3.clone(), &[]))? }; for object in taken { picks_3.push((player, object)); } }
gathered_2.extend(picks_3.iter().copied());
    } 
let sacrificed_2 = g.sacrifice_each(&gathered_2);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Flare of Malice", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(1, ability_0_0)), ZoneKind::Hand, "You may sacrifice a nontoken black creature rather than pay this spell's mana cost.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
