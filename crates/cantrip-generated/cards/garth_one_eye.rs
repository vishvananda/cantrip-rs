// Garth One-Eye
// status: planned
// requires: a choice of CardName outside its body
// requires: operation ChooseValueWith
// {T}: Choose a card name that hasn't been chosen from among Disenchant, Braingeyser, Terror, Shivan Dragon, Regrowth, and Black Lotus. Create a copy of the card with the chosen name. You may cast the copy. (You still pay its costs.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-choose-value-with" => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CardName, ChoiceTerms { options: &[CardsInOracle::Or(&[CardsInOracle::IsNamed(NameFilter::NamedCard("Disenchant")), CardsInOracle::IsNamed(NameFilter::NamedCard("Braingeyser")), CardsInOracle::IsNamed(NameFilter::NamedCard("Terror")), CardsInOracle::IsNamed(NameFilter::NamedCard("Shivan Dragon")), CardsInOracle::IsNamed(NameFilter::NamedCard("Regrowth")), CardsInOracle::IsNamed(NameFilter::NamedCard("Black Lotus"))])], except: &[], count: None, secret: false, unchosen: true, random: false, ordered: false, up_to: false })?);
g.end_sentence();
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_1 = { let q: &dyn Query = g; (q.outside_the_game(asker.controller)).into_iter().filter(|&id| planned_value!("a choice of CardName outside its body", None::<CardId> => q.state().chosen_card(q.linked(item))).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, id, chosen))).collect::<Vec<ObjectId>>() }; let times_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_1: Vec<ObjectId> = Vec::new(); for _ in 0..times_1 { for &card in &originals_1 { copies_1.push(g.create_copy_in_exile(card, player_1)); } }
g.end_sentence();
let caster_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_2: Vec<ObjectId> = { let q: &dyn Query = g; (copies_1.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_2: u32 = 1; let mut played_2: Vec<ObjectId> = Vec::new();
while left_2 > 0 {
let mut open_2: Vec<ObjectId> = Vec::new(); for &c in &pool_2 { let ways = g.cast_ways_during_resolution_by(c, caster_2); if ways.iter().any(|w| true) { open_2.push(c); } }
let picked_2 = if open_2.is_empty() { None } else if open_2.len() > 1 || (true && (!true || pool_2.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_2, asker.source, Purpose::Optional, pool_2.clone(), open_2.clone(), 1, true)?.first().copied() } else if cantrip_mechanics::effects::ask_may(g, caster_2, open_2[0], Purpose::Optional)? { Some(open_2[0]) } else { None };
let Some(card_2) = picked_2 else { break; };
let ways_2: Vec<CastWay> = g.cast_ways_during_resolution_by(card_2, caster_2).into_iter().filter(|w| true).collect();
let done_2 = { let way = cantrip_mechanics::choose::cast_way(g, caster_2, card_2, ways_2)?.map(|w| w.way); g.cast_during_resolution_as(card_2, CastTerms { instead: None, way, caster: Some(caster_2), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_2 { left_2 -= 1; played_2.push(g.state().obj(card_2).became.unwrap_or(card_2)); } else { break; } if done_2 || !true { pool_2.retain(|&c| c != card_2); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Garth One-Eye", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 1, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{T}: Choose a card name that hasn't been chosen from among Disenchant, Braingeyser, Terror, Shivan Dragon, Regrowth, and Black Lotus. Create a copy of the card with the chosen name. You may cast the copy.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
