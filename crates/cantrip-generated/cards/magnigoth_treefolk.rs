// Magnigoth Treefolk
// status: planned
// requires: landwalk of SharesWithAny
// Domain — For each basic land type among lands you control, this creature has landwalk of that type. (It can't be blocked as long as defending player controls a land of that type.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("landwalk-of-shares-with-any" => statics: cantrip_mechanics::statics::grant_to(|q: &dyn Query, asker: &Asker| -> Vec<ObjectId> { Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }, Keyword::Landwalk(LandFilter::Sharing(Property::BasicLandTypes, |q: &dyn Query, asker: &Asker| -> Vec<ObjectId> { q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }))));

pub static CARD: CardDef = CardDef { name: "Magnigoth Treefolk", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Treefolk]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
