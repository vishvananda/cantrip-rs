// Solid Footing
// status: ready
// Flash
// Enchant creature
// Enchanted creature gets +1/+1.
// As long as enchanted creature has vigilance, it assigns combat damage equal to its toughness rather than its power.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_3_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).is_some_and(|candidate: ObjectId| q.chars(candidate).keywords.has(Keyword::Vigilance))) && (q.state().obj(source).attached_to == Some(affected)) }
#[allow(unused_variables)] fn ability_3_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).is_some_and(|candidate: ObjectId| q.chars(candidate).keywords.has(Keyword::Vigilance))) && q.state().obj(asker.source).attached_to == Some(id) }

pub static CARD: CardDef = CardDef { name: "Solid Footing", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_2_apply_0, "Enchanted creature gets +1/+1.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_1_enchant)), damage_by_toughness: &[ability_3_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
