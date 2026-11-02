// Tangleclaw Werewolf // Fibrous Entangler
// status: planned
// requires: static rule BlockCapacity
// Vigilance
// This creature must be blocked if able.
// This creature can block an additional creature each combat.
// This creature can block an additional creature each combat.
// {6}{G}: Transform this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> Option<u32> { (id == asker.source).then(|| (1i64).max(0) as u32) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }
planned_item!("static-rule-block-capacity" => additional_blocks: ability_0_0_rule);

pub static CARD: CardDef = CardDef { name: "Tangleclaw Werewolf", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Werewolf,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{6}{G}: Transform this creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); attacker == asker.source }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> Option<u32> { (id == asker.source).then(|| (1i64).max(0) as u32) }
planned_item!("static-rule-block-capacity" => additional_blocks: ability_2_0_rule);

pub static CARD: CardDef = CardDef { name: "Fibrous Entangler", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi,Subtype::Werewolf]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, must_be_blocked: &[ability_1_0_rule], ..CardDef::BLANK };
}
