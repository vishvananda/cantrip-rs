// Melira, Sylvok Outcast
// status: ready
// You can't get poison counters.
// Creatures you control can't have -1/-1 counters put on them.
// Creatures your opponents control lose infect.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, kind: PlayerCounter) -> bool { player == asker.controller && matches!(kind, PlayerCounter::Poison) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, object: ObjectId, controller: PlayerId, kind: CounterKind) -> bool { (q.chars(object).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(object))) && kind == CounterKind::MinusOne }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(affected))) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = Keywords(chars.keywords.0 & !Keywords::of(&[Keyword::Infect]).0); }

pub static CARD: CardDef = CardDef { name: "Melira, Sylvok Outcast", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Scout]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_2_affects, ability_2_apply_0, "Creatures your opponents control lose infect.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, counter_bans: &[ability_1_0_rule], player_counter_bans: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
