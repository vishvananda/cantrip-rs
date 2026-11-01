// Torrent of Lava
// status: planned
// requires: object reference Granter
// requires: source-zone scope on Static
// Torrent of Lava deals X damage to each creature without flying.
// As long as Torrent of Lava is on the stack, each creature has "{T}: Prevent the next 1 damage that would be dealt to this creature by Torrent of Lava this turn."
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let amount_0 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && !(q.chars(id).keywords.has(Keyword::Flying)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_stated_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (*so_far).is(CardType::Creature) }
#[allow(unused_variables)] fn made_3_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if q.state().obj(source).zone.is(ZoneKind::Stack) && Some(affected) == Some(source) && true { return true; } }
 false }
fn granted_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let shield = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let to = { let q: &dyn Query = g; ((Some(asker.source)).map(Target::Object).into_iter().collect::<Vec<Target>>()).first().copied() }; let about = { let q: &dyn Query = g; planned_value!("object reference Granter", None::<ObjectId> => q.granted_by(asker.source)) }; if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::Next(shield) }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_stated_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)); } }
planned_item!("source-zone-scope-on-static" => statics: cantrip_mechanics::zones::scoped(&[EventZone::Stack], None, static_ability(Layer::Ability, ability_1_stated_affects, ability_1_stated_apply_0, "As long as Torrent of Lava is on the stack, each creature has \"{T}: Prevent the next 1 damage that would be dealt to this creature by Torrent of Lava this turn.\"")));

pub static CARD: CardDef = CardDef { name: "Torrent of Lava", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), granted_1, "{T}: Prevent the next 1 damage that would be dealt to this creature by Torrent of Lava this turn.").targets(&[])], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "", damage: "damage", of_it: None, applies: made_3_applies }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
