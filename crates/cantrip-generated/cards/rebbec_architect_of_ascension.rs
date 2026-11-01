// Rebbec, Architect of Ascension
// status: planned
// requires: comparison with any one value
// requires: engine: no protection from this quality
// Artifacts you control have protection from each mana value among artifacts you control.
// Partner (You can have two commanders if both have partner.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); planned_value!("engine: no protection from this quality", () => chars.protection_from.push(ProtectionQuality::Matching(|q, asker, id| (i64::from(q.mana_value(id, &q.chars(id)))) == (planned_value!("comparison with any one value", 0i64 => cantrip_mechanics::numbers::AnyOf(&(q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o, &q.chars(o)))).collect::<Vec<i64>>())))))); }

pub static CARD: CardDef = CardDef { name: "Rebbec, Architect of Ascension", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_0_affects, ability_0_apply_0, "Artifacts you control have protection from each mana value among artifacts you control.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
