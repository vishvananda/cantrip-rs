// Druid's Deliverance
// status: ready
// Prevent all combat damage that would be dealt to you this turn. Populate. (Create a token that's a copy of a creature token you control.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && true && true && true { return true; } }
 false }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{  let to = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>()).first().copied() }; let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
let mut populated_2: Vec<ObjectId> = Vec::new();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let tokens: Vec<ObjectId> = g.permanents_of(player).into_iter().filter(|&o| g.obj(o).token && g.chars(o).is(CardType::Creature)).collect(); if let Some(token) = cantrip_mechanics::choose::card(g, player, asker.source, Purpose::CopyWhich, tokens)? { let values = g.copiable_values(token); populated_2.extend(g.create_token_copies_tapped(values, player, 1, false)?); }; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Druid's Deliverance", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Prevent all combat damage that would be dealt to you this turn. Populate.", damage: "damage", of_it: None, applies: made_1_applies, then: None, text: "Prevent all combat damage that would be dealt to you this turn. Populate." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
