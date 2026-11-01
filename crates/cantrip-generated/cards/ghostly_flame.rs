// Ghostly Flame
// status: planned
// requires: static change ColorlessDamageSource
// requires: static rule Unique
// Black and/or red permanents and spells are colorless sources of damage.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).colors.has(q.words(asker.source).color(Color::Black)) || (*so_far).colors.has(q.words(asker.source).color(Color::Red))) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).colors.has(q.words(asker.source).color(Color::Black)) || q.chars(id).colors.has(q.words(asker.source).color(Color::Red))) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(affected).became.is_none() && q.state().obj(affected).zone.kind() == ZoneKind::Stack) && ((*so_far).colors.has(q.words(asker.source).color(Color::Black)) || (*so_far).colors.has(q.words(asker.source).color(Color::Red))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); planned_value!("static change ColorlessDamageSource", () => chars.colorless_damage_source = true); }
planned_item!("static-rule-unique" => statics: object_rule(ability_0_0_rule, Rule::ColorlessSourceOfDamage));

pub static CARD: CardDef = CardDef { name: "Ghostly Flame", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Color, ability_1_affects, ability_1_apply_0, "Black and/or red permanents and spells are colorless sources of damage.").everywhere()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
