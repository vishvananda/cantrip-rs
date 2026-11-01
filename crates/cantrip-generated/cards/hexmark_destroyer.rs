// Hexmark Destroyer
// status: planned
// requires: static rule CombatGroup BlockedAttacker
// Multi-threat Eliminator — This creature can't be blocked except by six or more creatures.
// Unearth {4}{B}{B} ({4}{B}{B}: Return this card from your graveyard to the battlefield. It gains haste. Exile it at the beginning of the next end step or if it would leave the battlefield. Unearth only as a sorcery.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(attacker == asker.source && !(q.chars(blocker).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_0_1_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
#[allow(unused_variables)] fn ability_0_1_rule_counted(q: &dyn Query, asker: &Asker, id: ObjectId, other: ObjectId) -> bool { q.chars(other).is(CardType::Creature) }
planned_item!("static-rule-combat-group-blocked-attacker" => combat_groups: CombatGroupRule { role: CombatRole::BlockedAttacker, applies: ability_0_1_rule, counted: ability_0_1_rule_counted, others_only: false, count: 6..=u32::MAX });

pub static CARD: CardDef = CardDef { name: "Hexmark Destroyer", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Necron]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::unearth::ability(ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Unearth")], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_0_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[cantrip_mechanics::unearth::EXILE], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
