// Predatory Impetus
// status: planned
// requires: static rule GoadedBy
// Enchant creature
// Enchanted creature gets +3/+3, must be blocked if able, and is goaded. (It attacks each combat if able and attacks a player other than you if able.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((3i64) as i16, (3i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, source: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); q.state().obj(asker.source).attached_to == Some(attacker) }
#[allow(unused_variables)] fn ability_2_1_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.state().obj(asker.source).attached_to == Some(id) }
planned_item!("static-rule-goaded-by" => goaded_rules: ability_2_1_rule);

pub static CARD: CardDef = CardDef { name: "Predatory Impetus", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_1_apply_0, "Enchanted creature gets +3/+3, must be blocked if able, and is goaded.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), must_be_blocked: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
