// Reckless Blaze
// status: ready
// Reckless Blaze deals 5 damage to each creature. Whenever a creature you control dealt damage this way dies this turn, add {R}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn watch_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && Some(asker.source) == Some(id)) { return Some(Fired { subject, data }); } }
 None }
fn watch_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_5_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_5 {
g.add_mana(player, Mana::R, mana_5_0);}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut done_0: i64 = 0; let mut dealt_0: Vec<ObjectId> = Vec::new(); let mut hurt_0: Vec<PlayerId> = Vec::new(); let before_0 = g.history().head() as usize; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let amount_0 = { let q: &dyn Query = g; (5i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); done_0 += g.damage_together(&batch).iter().map(|&n| i64::from(n)).sum::<i64>(); dealt_0.extend(cantrip_mechanics::history::permanents_dealt_damage_by(g.history(), source, before_0)); for r in &g.history().all()[before_0..] { if let Event::Damage { source: s, target: Target::Player(p), amount, .. } = r.event { if s == source && amount > 0 && !hurt_0.contains(&p) { hurt_0.push(p); } } } }
g.end_sentence();
for bound_1002 in dealt_0.clone() {
if let Some(source) = { let q: &dyn Query = g; Some(bound_1002) } { g.delayed_trigger(DelayedWhen::ThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Reckless Blaze", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[Subtype::Lesson]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), watch_3_event), watch_3, "Whenever a creature you control dealt damage this way dies this turn, add {R}.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
