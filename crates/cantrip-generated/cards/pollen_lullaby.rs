// Pollen Lullaby
// status: planned
// requires: duration DuringNextUntapStep
// requires: engine: no resolving rule on an open set: UntapStep
// requires: operation Clash
// requires: player reference ClashOpponent
// Prevent all combat damage that would be dealt this turn. Clash with an opponent. If you win, creatures that player controls don't untap during the player's next untap step. (Each clashing player reveals the top card of their library, then puts that card on their choice of the top or bottom. A player wins if their card had a greater mana value.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && true && true && true { return true; } }
 false }
planned_item!("engine-no-resolving-rule-on-an-open-set-untap-step" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && planned_value!("player reference ClashOpponent", None::<PlayerId> => q.clash_opponent(asker)) == Some(q.controller(id))) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let opponents_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let won_2: bool = planned_value!("operation Clash", false => cantrip_mechanics::clash::clash(g, player_2, asker.source, opponents_2)?); if won_2 {
if let Some(present_5_0) = { let q: &dyn Query = g; planned_value!("player reference ClashOpponent", None::<PlayerId> => q.clash_opponent(asker)) } { let about_0 = Target::None; let until_0 = planned_value!("duration DuringNextUntapStep", Duration::Indefinite => Duration::DuringNextUntapStepOf({ let q: &dyn Query = g; present_5_0 }));
planned!("engine-no-resolving-rule-on-an-open-set-untap-step" => g.add_lasting_rule(item, 0, about_0, until_0););
 }
g.end_sentence();
} else {
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-untap-step" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "If you win, creatures that player controls don't untap during the player's next untap step.", rule: LastingRuleKind::DoesntUntap, applies: lasting_rule_0 }];);

pub static CARD: CardDef = CardDef { name: "Pollen Lullaby", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Prevent all combat damage that would be dealt this turn. Clash with an opponent. If you win, creatures that player controls don't untap during the player's next untap step.", damage: "damage", of_it: None, applies: made_1_applies, then: None, text: "Prevent all combat damage that would be dealt this turn. Clash with an opponent. If you win, creatures that player controls don't untap during the player's next untap step." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
