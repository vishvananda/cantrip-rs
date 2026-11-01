// Iroas, God of Victory
// status: planned
// requires: numeric expression CountManaSymbols
// Indestructible
// As long as your devotion to red and white is less than seven, Iroas isn't a creature.
// Creatures you control have menace.
// Prevent all damage that would be dealt to attacking creatures you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((planned_value!("numeric expression CountManaSymbols", 0i64 => i64::from(cantrip_mechanics::devotion::symbols_among(q, &objects, 2)))) < (7i64)) && (source == affected) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.types = TypeSet(chars.types.0 & !(1 << CardType::Creature as u8)); }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Menace); }
#[allow(unused_variables)] fn ability_3_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Object(o) if ((q.state().attackers.iter().any(|a| a.id == o) || q.was_attacking(o)) && q.chars(o).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(o)))) { return true; } }
 false }

pub static CARD: CardDef = CardDef { name: "Iroas, God of Victory", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::God]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Type, ability_1_0_affects, ability_1_0_apply_0, "As long as your devotion to red and white is less than seven, Iroas isn't a creature."),static_ability(Layer::Ability, ability_2_affects, ability_2_apply_0, "Creatures you control have menace.")], replacements: &[], preventions: &[PreventionDef { hint: "Prevent all damage that would be dealt to attacking creatures you control.", applies: ability_3_applies, amount: None }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
