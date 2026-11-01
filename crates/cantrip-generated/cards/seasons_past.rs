// Seasons Past
// status: planned
// requires: operation ChangeZone
// Return any number of cards with different mana values from your graveyard to your hand. Put Seasons Past on the bottom of its owner's library.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let moved_0: Vec<ObjectId> = planned_value!("operation ChangeZone", Vec::<ObjectId>::new() => cantrip_mechanics::enter::change_zone(g, asker, ZoneChange { actor: { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, cards: { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }, count: Some({ let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }), random: false, group: Some(GroupFilter::DifferentManaValues), destination: Destination::OwnersHand, entry: &[], face_down: None })?);
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_1 = g.put_into_library(&objects_1, LibraryPlace::Bottom);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Seasons Past", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
