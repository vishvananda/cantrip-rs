// Selesnya Loft Gardens
// status: planned
// requires: card type Plane
// requires: layout Planar
// requires: subtype Ravnica
// requires: trigger event Player ChaosEnsues
// If an effect would create one or more tokens, it creates twice that many of those tokens instead.
// If an effect would put one or more counters on a permanent, it puts twice that many of those counters on that permanent instead.
// Whenever chaos ensues, until end of turn, whenever you tap a land for mana, add one mana of any type that land produced.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::TokenCreation(t) = proposal { let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if t.count > 0 && true { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::TokenCreation(seen) = &seen { let t = seen; let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if let Proposal::TokenCreation(t) = proposal { t.count = t.count.saturating_mul(2); } } }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Counters(c) = proposal { let id = c.object; let subject = Target::Object(id); let data = [c.counts.iter().map(|&n| n.max(0) as u32).sum::<u32>(), u32::MAX]; if c.counts.iter().any(|&n| n > 0) && (c.entering || q.is_on_battlefield(id)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && c.effect { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Counters(seen) = &seen { let c = seen; let id = c.object; let subject = Target::Object(id); let data = [c.counts.iter().map(|&n| n.max(0) as u32).sum::<u32>(), u32::MAX]; if let Proposal::Counters(c) = proposal { for i in 0..c.counts.len() { if c.counts[i] > 0 { let data = [c.counts[i] as u32, u32::MAX]; c.counts[i] = ((2i64 * i64::from(data[0]))).min(i16::MAX.into()) as i16; } } } } }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
#[allow(unused_variables)] fn watch_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TappedForMana { permanent, player, produced } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), Mana::ALL.iter().filter(|&&m| produced.get(m) > 0).fold(0u32, |types, &m| types | 1 << m as u32)]; if Some(asker.controller) == Some(*player) && q.chars(id).is(CardType::Land) { return Some(Fired { subject, data }); } }
 None }
fn watch_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if item.data[1] != 0 { cantrip_mechanics::mana::add_one_mana_of_any_type_that_land_produced(g, item)?; }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::ThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Selesnya Loft Gardens", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Ravnica", Subtype::Plains => Subtype::Ravnica)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_2_event), ability_2, "Whenever chaos ensues, until end of turn, whenever you tap a land for mana, add one mana of any type that land produced.").targets(&[])], statics: &[], replacements: &[replacement(ReplacementKind::TokenCreation, ability_0_applies, ability_0_apply, "If an effect would create one or more tokens, it creates twice that many of those tokens instead."),replacement(ReplacementKind::Counters, ability_1_applies, ability_1_apply, "If an effect would put one or more counters on a permanent, it puts twice that many of those counters on that permanent instead.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::TAPPED.0), watch_1_event), watch_1, "Whenever chaos ensues, until end of turn, whenever you tap a land for mana, add one mana of any type that land produced.").targets(&[]).mana_ability(TriggeredMana::OneOfProduced)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
