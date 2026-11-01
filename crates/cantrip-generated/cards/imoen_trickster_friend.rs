// Imoen, Trickster Friend
// status: planned
// requires: engine: no Specialize
// requires: keyword modifier ReduceCostIf
// Specialize {5}. This ability costs {3} less to activate if there are two or more instant and/or sorcery cards in your graveyard.
// Imoen can't be blocked as long as it's attacking alone.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().attackers.len() == 1 && q.state().attackers.iter().any(|a| a.id == candidate)))) && (source == affected) }
#[allow(unused_variables)] fn ability_1_0_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !((Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().attackers.len() == 1 && q.state().attackers.iter().any(|a| a.id == candidate)))) && attacker == asker.source && true) }
planned_item!("engine-no-specialize" => abilities: cantrip_mechanics::specialize::ability(ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Specialize {5}. This ability costs {3} less to activate if there are two or more instant and/or sorcery cards in your graveyard."));
planned_item!("keyword-modifier-reduce-cost-if" => abilities: modified(, Modifier::ReduceCostIf));

pub static CARD: CardDef = CardDef { name: "Imoen, Trickster Friend", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Rogue,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_1_0_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
