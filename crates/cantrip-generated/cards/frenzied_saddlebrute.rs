// Frenzied Saddlebrute
// status: planned
// requires: static rule AttackException SummoningSickness
// Haste
// All creatures can attack your opponents and planeswalkers your opponents control as though those creatures had haste.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (*so_far).is(CardType::Creature) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { q.chars(attacker).is(CardType::Creature) && (match defender { Target::Player(p) => (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => true && q.chars(id).is(CardType::Planeswalker) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false }) }
planned_item!("static-rule-attack-exception-summoning-sickness" => attacks_as_though_haste: ability_1_0_rule);

pub static CARD: CardDef = CardDef { name: "Frenzied Saddlebrute", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Orc,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
