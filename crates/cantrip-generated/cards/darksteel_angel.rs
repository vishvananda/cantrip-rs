// Darksteel Angel
// status: ready
// Flying, indestructible
// You can't lose the game and your opponents can't win the game.
// Creatures you control can't have -1/-1 counters put on them.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, reason: LossReason) -> bool { player == asker.controller && true }
#[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player != asker.controller }
#[allow(unused_variables)] fn ability_4_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_4_0_rule(q: &dyn Query, asker: &Asker, object: ObjectId, controller: PlayerId, kind: CounterKind) -> bool { (q.chars(object).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(object))) && kind == CounterKind::MinusOne }

pub static CARD: CardDef = CardDef { name: "Darksteel Angel", token_words: &[], oracle: true, cost: ManaCost { generic: 9, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, counter_bans: &[ability_4_0_rule], loss_rules: &[ability_2_0_rule], win_rules: &[ability_3_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
