// Legion's Chant
// status: planned
// requires: engine: no StartingIntensity
// requires: operation ChangeZone
// requires: subtype Chorus
// Starting intensity 3
// Return any number of creature cards with total mana value X or less from your graveyard to the battlefield, where X is this spell's intensity.
// All Chorus cards you own intensify.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let moved_0: Vec<ObjectId> = planned_value!("operation ChangeZone", Vec::<ObjectId>::new() => cantrip_mechanics::enter::change_zone(g, asker, ZoneChange { actor: { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, cards: { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }, count: Some({ let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }), random: false, group: Some(GroupFilter::TotalManaValueIs("Comparison(LessOrEqual, ObjectProperty { object: Source, property: Intensity, information: CurrentOrLastKnown })")), destination: Destination::OwnersBattlefield, entry: &[EnterFlag::EntersNormally], face_down: None })?);
g.end_sentence();
planned!("subtype-chorus" => cantrip_mechanics::intensity::intensify_all(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, |q: &dyn Query, asker: &Asker, id: ObjectId| -> bool { q.chars(id).has_subtype(Subtype::Chorus) }));
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-starting-intensity" => enters: cantrip_mechanics::intensity::starting::<3>);

pub static CARD: CardDef = CardDef { name: "Legion's Chant", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[planned_value!("subtype Chorus", Subtype::Plains => Subtype::Chorus)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
