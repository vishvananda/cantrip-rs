// Kemba's Legion
// status: planned
// requires: static rule BlockCapacity
// Vigilance
// This creature can block an additional creature each combat for each Equipment attached to this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> Option<u32> { (id == asker.source).then(|| ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment)) && q.state().obj(id).attached_to.is_some_and(|host| Some(asker.source) == Some(host)))).collect::<Vec<ObjectId>>()).len() as i64).max(0) as u32) }
planned_item!("static-rule-block-capacity" => additional_blocks: ability_1_0_rule);

pub static CARD: CardDef = CardDef { name: "Kemba's Legion", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
