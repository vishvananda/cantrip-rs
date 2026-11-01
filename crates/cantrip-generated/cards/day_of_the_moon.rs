// Day of the Moon
// status: planned
// requires: object filter ChosenOn
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I, II, III — Choose a creature card name, then goad all creatures with a name chosen for this enchantment. (Until your next turn, they attack each combat if able and attack a player other than you if able.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Name(Some(chosen_0)) = g.ask(player_0, RequestKind::ChooseName { source: asker.source, looked_at: Vec::new(), suggestion: None, kind: NameKind { types: TypeSet::of(&[CardType::Creature]), ..NameKind::ANY } })? else { return Ok(()) };
g.end_sentence();
for bound_4294967294 in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && planned_value!("object filter ChosenOn", false => Some(asker.source).is_some_and(|o: ObjectId| q.chosen_on(o, ChoiceKind::CardName).matches(id))))).collect::<Vec<ObjectId>>() } {
if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967294).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { cantrip_mechanics::goad::goad(g, item, object); }
}
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Name(Some(chosen_3)) = g.ask(player_3, RequestKind::ChooseName { source: asker.source, looked_at: Vec::new(), suggestion: None, kind: NameKind { types: TypeSet::of(&[CardType::Creature]), ..NameKind::ANY } })? else { return Ok(()) };
g.end_sentence();
for bound_4294967291 in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && planned_value!("object filter ChosenOn", false => Some(asker.source).is_some_and(|o: ObjectId| q.chosen_on(o, ChoiceKind::CardName).matches(id))))).collect::<Vec<ObjectId>>() } {
if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967291).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { cantrip_mechanics::goad::goad(g, item, object); }
}
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Name(Some(chosen_6)) = g.ask(player_6, RequestKind::ChooseName { source: asker.source, looked_at: Vec::new(), suggestion: None, kind: NameKind { types: TypeSet::of(&[CardType::Creature]), ..NameKind::ANY } })? else { return Ok(()) };
g.end_sentence();
for bound_4294967288 in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && planned_value!("object filter ChosenOn", false => Some(asker.source).is_some_and(|o: ObjectId| q.chosen_on(o, ChoiceKind::CardName).matches(id))))).collect::<Vec<ObjectId>>() } {
if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967288).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { cantrip_mechanics::goad::goad(g, item, object); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Day of the Moon", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II, III — Choose a creature card name, then goad all creatures with a name chosen for this enchantment.").targets(&[]),chapter(2, ability_0_1, "I, II, III — Choose a creature card name, then goad all creatures with a name chosen for this enchantment.").targets(&[]),chapter(3, ability_0_2, "I, II, III — Choose a creature card name, then goad all creatures with a name chosen for this enchantment.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
