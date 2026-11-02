// Chorus of the Conclave
// status: planned
// requires: cost part PlayerAction
// requires: numeric expression Game
// requires: spell cost rule with terms
// Forestwalk (This creature can't be blocked as long as defending player controls a Forest.)
// As an additional cost to cast creature spells, you may pay any amount of mana. If you do, that creature enters with that many additional +1/+1 counters on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");
planned_item!("spell-cost-rule-with-terms" => statics: player_rule(ability_1_0_applies, PlayerRule::SpellCost { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| q.chars(spell).is(CardType::Creature), change: CostModifier::OptionalAdditional { cost: Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost part PlayerAction", Cost::NONE => Cost::performed(|q: &dyn Query, asker: &Asker| -> Option<PlayerId> { Some(asker.controller) }, |g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { let mut done_1: i64 = 0; for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { done_1 += { i64::from(g.may_pay_x(player, asker.source, asker.source, cost("{X}"), &PAY_QUESTION)?.unwrap_or(0)) }; }
g.end_sentence();
 Ok(()) }))), then: &[EnterFlag::EntersWithNumberCounters(planned_value!("numeric expression Game", 0i64 => q.game_value_of(asker, GameValue::ManaPaidThisWay)), CounterType::PTCounter(1, 1))] } }));

pub static CARD: CardDef = CardDef { name: "Chorus of the Conclave", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [2, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dryad]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(8), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Forestwalk]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
