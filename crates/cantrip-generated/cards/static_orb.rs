// Static Orb
// status: planned
// requires: a limit on what untaps in an untap step
// As long as this artifact is untapped, players can't untap more than two permanents during their untap steps.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => #[allow(unused_variables)] fn ability_0_0_0_untaps_0(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { (Some(asker.source).is_some_and(|candidate: ObjectId| !(q.state().obj(candidate).tapped))) && true });
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => #[allow(unused_variables)] fn ability_0_0_0_untapping_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { true });
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => untap_limits: UntapLimit { applies: ability_0_0_0_untaps_0, permanents: ability_0_0_0_untapping_0, amount: 2 });

pub static CARD: CardDef = CardDef { name: "Static Orb", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
