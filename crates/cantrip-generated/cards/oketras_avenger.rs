// Oketra's Avenger
// status: ready
// You may exert this creature as it attacks. When you do, prevent all combat damage that would be dealt to it this turn. (An exerted creature won't untap during your next untap step.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && true && true && true { return true; } }
 false }
fn ability_0_0_rule(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{  let to = { let q: &dyn Query = g; ((Some(asker.source)).map(Target::Object).into_iter().collect::<Vec<Target>>()).first().copied() }; let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Oketra's Avenger", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::exert::as_it_attacks(ability_0_0_rule, "You may exert this creature as it attacks. When you do, prevent all combat damage that would be dealt to it this turn.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "You may exert this creature as it attacks. When you do, prevent all combat damage that would be dealt to it this turn.", damage: "damage", of_it: None, applies: made_1_applies, then: None, text: "When you do, prevent all combat damage that would be dealt to it this turn." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
