// Lion Umbra
// status: planned
// requires: engine: no UmbraArmor
// Enchant modified creature (Equipment, Auras its controller controls, and counters are modifications.)
// Enchanted creature gets +3/+3 and has reach and vigilance.
// Umbra armor (If enchanted creature would be destroyed, instead remove all damage from it and destroy this Aura.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((modified(q, id) && q.chars(id).is(CardType::Creature)))) }
#[allow(unused_variables)] fn ability_1_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Reach);
chars.keywords = chars.keywords.with(Keyword::Vigilance); }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((3i64) as i16, (3i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
fn modified(q: &dyn Query, id: ObjectId) -> bool { q.state().obj(id).has_counters() || q.state().battlefield().iter().any(|&a| q.state().obj(a).attached_to == Some(id) && { let chars = q.chars(a); chars.has_subtype(Subtype::Equipment) || chars.has_subtype(Subtype::Aura) && q.controller(a) == q.controller(id) }) }
planned_item!("engine-no-umbra-armor" => replacements: cantrip_mechanics::umbra_armor::REPLACEMENT);

pub static CARD: CardDef = CardDef { name: "Lion Umbra", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_1_apply_1, "Enchanted creature gets +3/+3 and has reach and vigilance."),static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_1_apply_0, "Enchanted creature gets +3/+3 and has reach and vigilance.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
