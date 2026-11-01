// Brutal Suppression
// status: planned
// requires: activation cost with another cost
// Activated abilities of nontoken Rebels cost an additional "Sacrifice a land" to activate.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_also_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Land) }
planned_item!("activation-cost-with-another-cost" => cost_changes: |q, source, player, costed, targets| { let asker = &Asker::new(source, q.controller(source)); let Costed::Ability(permanent, ability, _) = costed else { return CostDelta::NONE; }; let def = q.activated_ability(ability); if true && (def.is_some_and(|d| d.from == ZoneKind::Battlefield) && (!(q.state().obj(permanent).token) && q.chars(permanent).has_subtype(q.words(asker.source).subtype(Subtype::Rebel)))) { CostDelta { also: Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(1, ability_0_0_also_0)), ..CostDelta::NONE } } else { CostDelta::NONE } });

pub static CARD: CardDef = CardDef { name: "Brutal Suppression", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
