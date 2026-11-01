// Archon of Absolution
// status: ready
// Flying
// Protection from white (This creature can't be blocked, targeted, dealt damage, enchanted, or equipped by anything white.)
// Creatures can't attack you or planeswalkers you control unless their controller pays {1} for each of those creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (*so_far).is(CardType::Creature) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> Option<ManaCost> { (q.chars(attacker).is(CardType::Creature) && (match defender { Target::Player(p) => (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => true && q.chars(id).is(CardType::Planeswalker) && (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false })).then(|| ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }) }

pub static CARD: CardDef = CardDef { name: "Archon of Absolution", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Archon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(1), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, attack_costs: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
