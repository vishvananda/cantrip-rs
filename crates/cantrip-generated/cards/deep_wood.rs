// Deep Wood
// status: ready
// Cast this spell only during the declare attackers step and only if you've been attacked this step.
// Prevent all damage that would be dealt to you this turn by attacking creatures.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { (q.state().turn.step() == Step::DeclareAttackers && Some(asker.controller).is_some_and(|player: PlayerId| q.state().attackers.iter().any(|a| a.target == Target::Player(player)))) }
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && ((q.state().attackers.iter().any(|a| a.id == source) || q.was_attacking(source)) && q.chars(source).is(CardType::Creature)) && true { return true; } }
 false }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{  let to = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>()).first().copied() }; let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Deep Wood", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Prevent all damage that would be dealt to you this turn by attacking creatures.", damage: "damage", of_it: None, applies: made_1_applies, then: None, text: "Prevent all damage that would be dealt to you this turn by attacking creatures." }], cast_only_if: Some(ability_0_if), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
