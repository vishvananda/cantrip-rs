// Shadow of Mortality
// status: ready
// If your life total is less than your starting life total, this spell costs {X} less to cast, where X is the difference.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_cost(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, player); if costed == Costed::Spell(source) && true { CostDelta::less(((0i64).max((Some(asker.controller).map_or(0, |player: PlayerId| i64::from(q.state().player(player).starting_life)) - Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))))).clamp(0, 255) as u8) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Shadow of Mortality", token_words: &[], oracle: true, cost: ManaCost { generic: 13, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, own_cost_changes: &[ability_0_cost], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
