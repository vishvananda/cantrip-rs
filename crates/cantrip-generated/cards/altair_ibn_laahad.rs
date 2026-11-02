// Altaïr Ibn-La'Ahad
// status: planned
// requires: tapped token copies of several objects at once
// First strike
// Whenever Altaïr attacks, exile up to one target Assassin creature card from your graveyard with a memory counter on it. Then for each creature card you own in exile with a memory counter on it, create a tapped and attacking token that's a copy of it. Exile those tokens at end of combat.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Assassin)) && q.chars(id).is(CardType::Creature)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 1)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = { let q: &dyn Query = g; about(q, item, 0) }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = { let counters: Vec<(CounterKind, i16)> = vec![(CounterKind::Memory, { let q: &dyn Query = g; (1i64).max(0) as i16 })]; let exiled = g.exile_all(&objects_0); let puts: Vec<(ObjectId, CounterKind, i16)> = exiled.iter().flat_map(|&o| counters.iter().map(move |&(kind, n)| (o, kind, n))).collect(); g.add_counters_together(&puts); exiled };
g.end_sentence();
planned!("tapped-token-copies-of-several-objects-at-once", CreateTokens, "entry" = "{\"counters\":[],\"states\":[{\"Attacking\":{\"or_planeswalker\":false,\"player\":()}}],\"tapped\":true}", "players" = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, "tokens" = "[{\"amount\":{\"args\":1,\"kind\":\"Constant\"},\"recipe\":{\"CopyEach\":{\"adjustments\":[{\"args\":{\"family\":\"TokenCopyEffects\",\"form\":\"NoTokenCopyEffects\",\"operands\":[]},\"kind\":\"Term\"}],\"objects\":{\"args\":{\"filter\":{\"args\":[{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"args\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"kind\":\"OwnedBy\"},{\"args\":{\"comparison\":\"Greater\",\"kind\":{\"Named\":\"MemoryCounter\"},\"value\":{\"args\":0,\"kind\":\"Constant\"}},\"kind\":\"Counters\"}],\"kind\":\"All\"},\"kind\":\"Card\",\"scope\":{\"Shared\":\"Exile\"}},\"kind\":\"Zone\"}}},\"words\":\"CreatingEffect\"}]");
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_1) }); about.push(ObjectId::NONE);  if about.iter().any(|&o| o != ObjectId::NONE) { g.delayed_trigger_about(DelayedWhen::Step { step: Step::EndOfCombat, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Altaïr Ibn-La'Ahad", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Assassin]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Altaïr attacks, exile up to one target Assassin creature card from your graveyard with a memory counter on it. Then for each creature card you own in exile with a memory counter on it, create a tapped and attacking token that's a copy of it. Exile those tokens at end of combat.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_3, "Exile those tokens at end of combat.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
