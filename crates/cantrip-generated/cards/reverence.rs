// Reverence
// status: ready
// Creatures with power 2 or less can't attack you.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && (*so_far).power.is_some_and(|v| (i64::from(v)) <= (2i64))) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { !((q.chars(attacker).is(CardType::Creature) && q.chars(attacker).power.is_some_and(|v| (i64::from(v)) <= (2i64))) && (match defender { Target::Player(p) => (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => false && q.chars(id).is(CardType::Planeswalker) && (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false })) }

pub static CARD: CardDef = CardDef { name: "Reverence", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[ability_0_0_rule], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
