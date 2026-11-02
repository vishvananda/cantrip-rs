// Phyrexian Unlife
// status: ready
// You don't lose the game for having 0 or less life.
// As long as you have 0 or less life, all damage is dealt to you as though its source had infect. (Damage is dealt to you in the form of poison counters.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, reason: LossReason) -> bool { player == asker.controller && matches!(reason, LossReason::Life) }
#[allow(unused_variables)] fn ability_1_0_infect(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { let target = &Target::Player(player); let subject = *target; true && true && matches!(*target, Target::Player(p) if Some(asker.controller) == Some(p)) && (Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))) <= (0i64) }

pub static CARD: CardDef = CardDef { name: "Phyrexian Unlife", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, infect_damage_to: &[ability_1_0_infect], loss_rules: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
