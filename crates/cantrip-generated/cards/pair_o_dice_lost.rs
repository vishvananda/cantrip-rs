// Pair o' Dice Lost
// status: planned
// requires: keyword action RollDie(6) with an amount
// requires: operation ChangeZone
// Roll two six-sided dice. Return any number of cards with total mana value X or less from your graveyard to your hand, where X is the total of those results. Exile Pair o' Dice Lost.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_0: i64 = planned_value!("keyword action RollDie(6) with an amount", 0i64 => cantrip_mechanics::dice::roll_several(g, player_0, 6, amount_0)?.total());
g.end_sentence();
let moved_1: Vec<ObjectId> = planned_value!("operation ChangeZone", Vec::<ObjectId>::new() => cantrip_mechanics::enter::change_zone(g, asker, ZoneChange { actor: { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, cards: { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }, count: Some({ let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }), random: false, group: Some(GroupFilter::TotalManaValueIs("Comparison(LessOrEqual, Dice(Total))")), destination: Destination::OwnersHand, entry: &[], face_down: None })?);
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Pair o' Dice Lost", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
