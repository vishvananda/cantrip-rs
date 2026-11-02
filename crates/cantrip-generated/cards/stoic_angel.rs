// Stoic Angel
// status: planned
// requires: a limit on what untaps in an untap step
// Flying, vigilance
// Players can't untap more than one creature during their untap steps.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => #[allow(unused_variables)] fn ability_2_0_untaps_0(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true });
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => #[allow(unused_variables)] fn ability_2_0_untapping_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) });
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => untap_limits: UntapLimit { applies: ability_2_0_untaps_0, permanents: ability_2_0_untapping_0, amount: 1 });

pub static CARD: CardDef = CardDef { name: "Stoic Angel", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
