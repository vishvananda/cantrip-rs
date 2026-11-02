// Smoke
// status: planned
// requires: a limit on what untaps in an untap step
// Players can't untap more than one creature during their untap steps.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => #[allow(unused_variables)] fn ability_0_0_untaps_0(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true });
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => #[allow(unused_variables)] fn ability_0_0_untapping_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) });
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => untap_limits: UntapLimit { applies: ability_0_0_untaps_0, permanents: ability_0_0_untapping_0, amount: 1 });

pub static CARD: CardDef = CardDef { name: "Smoke", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
