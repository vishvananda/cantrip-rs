// Batwing Brume
// status: ready
// Prevent all combat damage that would be dealt this turn if {W} was spent to cast this spell. Each player loses 1 life for each attacking creature they control if {B} was spent to cast this spell. (Do both if {W}{B} was spent.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && true && true && true { return true; } }
 false }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| ((Some(candidate).map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::W].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })) >= (1i64))) } {
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| ((Some(candidate).map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::B].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })) >= (1i64))) } {
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let amount_3 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| ((q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 2147483647) as i32 }; let players_3 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.lose_life(player, amount_3); }
    } 
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Batwing Brume", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::W, Mana::B)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Prevent all combat damage that would be dealt this turn if {W} was spent to cast this spell. Each player loses 1 life for each attacking creature they control if {B} was spent to cast this spell.", damage: "damage", of_it: None, applies: made_1_applies, then: None, text: "Prevent all combat damage that would be dealt this turn if {W} was spent to cast this spell. Each player loses 1 life for each attacking creature they control if {B} was spent to cast this spell." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
