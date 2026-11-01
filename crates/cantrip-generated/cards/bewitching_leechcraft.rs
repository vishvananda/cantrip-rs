// Bewitching Leechcraft
// status: planned
// requires: granted Replacement
// requires: replacement of UntapsInUntapStep
// Enchant creature
// When this Aura enters, tap enchanted creature.
// Enchanted creature has "If this creature would untap during your untap step, remove a +1/+1 counter from it instead. If you do, untap it." (Otherwise, it doesn't untap.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_0: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn granted_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let mut removed_2: i64 = 0;
let objects_2_0 = { let q: &dyn Query = g; subject.object().into_iter().collect::<Vec<ObjectId>>() };
let amount_2_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_2_0 { removed_2 += i64::from(g.remove_counters(object, CounterKind::PlusOne, amount_2_0)); } });
let paid_2 = removed_2 > 0;
if { let q: &dyn Query = g; ((paid_2 as i64) != 0) } {
let objects_4 = { let q: &dyn Query = g; (subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_4: Vec<ObjectId> = objects_4.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_4 { g.untap(object); } });
}
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(planned_value!("granted Replacement", AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) => AbilityRef::granted_static({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))); } }
planned_item!("replacement-of-untaps-in-untap-step" => callbacks: #[allow(unused_variables)] fn granted_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::StepUntap(p) = proposal { let id = p.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if Some(asker.source) == Some(id) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn granted_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::StepUntap(seen) = &seen { let p = seen; let id = p.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::StepUntap(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn granted_1_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::StepUntap(p) = before { let q: &dyn Query = g; let id = p.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; let _ = granted_1_then_steps(g, asker, subject, data); } }
);
planned_item!("granted-replacement" => granted_rules: CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::StepUntap, granted_1_applies, granted_1_apply, "If this creature would untap during your untap step, remove a +1/+1 counter from it instead. If you do, untap it.").then(granted_1_then)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, ..CardDef::BLANK });

pub static CARD: CardDef = CardDef { name: "Bewitching Leechcraft", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this Aura enters, tap enchanted creature.").targets(&[])], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_2_apply_0, "Enchanted creature has \"If this creature would untap during your untap step, remove a +1/+1 counter from it instead. If you do, untap it.\"")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
