// Opal-Eye, Konda's Yojimbo
// status: planned
// requires: engine: no Bushido
// requires: object collection DamageSources
// requires: prevention limited by NextOccurrence
// Defender (This creature can't attack.)
// Bushido 1 (Whenever this creature blocks or becomes blocked, it gets +1/+1 until end of turn.)
// {T}: The next time a source of your choice would deal damage this turn, that damage is dealt to Opal-Eye instead.
// {1}{W}: Prevent the next 1 damage that would be dealt to Opal-Eye this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_2_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && Some(affected) == Some(source) && true { return true; } }
 false }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; (planned_value!("object collection DamageSources", Vec::<ObjectId>::new() => cantrip_mechanics::damage_sources(q))).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::ChooseAsEnters, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
g.end_sentence();
{  let to = Some(Target::None); let about = { let q: &dyn Query = g; (selected_0.clone()).get(0 as usize).copied() }; if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: planned_value!("prevention limited by NextOccurrence", Shield::All => Shield::NextTime) }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn made_4_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && true { return true; } }
 false }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let shield = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 1, to, shield: Shield::Next(shield) }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-bushido" => abilities: cantrip_mechanics::bushido::trigger::<1>("Bushido 1"));

pub static CARD: CardDef = CardDef { name: "Opal-Eye, Konda's Yojimbo", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fox,Subtype::Samurai]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_2, "{T}: The next time a source of your choice would deal damage this turn, that damage is dealt to Opal-Eye instead.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_3, "{1}{W}: Prevent the next 1 damage that would be dealt to Opal-Eye this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "{T}: The next time a source of your choice would deal damage this turn, that damage is dealt to Opal-Eye instead.", damage: "damage", of_it: None, applies: made_2_applies },GrantedPreventionDef { hint: "{1}{W}: Prevent the next 1 damage that would be dealt to Opal-Eye this turn.", damage: "damage", of_it: None, applies: made_4_applies }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
