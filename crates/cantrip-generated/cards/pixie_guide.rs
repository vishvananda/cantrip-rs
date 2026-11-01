// Pixie Guide
// status: planned
// requires: replacement of RollDice
// Flying
// Grant an Advantage — If you would roll one or more dice, instead roll that many dice plus one and ignore the lowest roll.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("replacement-of-roll-dice" => callbacks: #[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::DieRoll(p) = proposal { let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if Some(asker.controller) == Some(p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::DieRoll(seen) = &seen { let p = seen; let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if let Proposal::DieRoll(p) = proposal { p.extra_dice += 1; p.ignore = Some(DiceIgnore::Lowest); } } }
);
planned_item!("replacement-of-roll-dice" => replacements: replacement(ReplacementKind::DieRoll, ability_1_applies, ability_1_apply, "Grant an Advantage — If you would roll one or more dice, instead roll that many dice plus one and ignore the lowest roll."));

pub static CARD: CardDef = CardDef { name: "Pixie Guide", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
