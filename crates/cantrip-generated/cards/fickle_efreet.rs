// Fickle Efreet
// status: ready
// Whenever this creature attacks or blocks, flip a coin at end of combat. If you lose the flip, an opponent gains control of this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let flip_2 = g.flip_coin(player_2);
if { let q: &dyn Query = g; !flip_2 } {
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_3) = cantrip_mechanics::choose::player(g, player_3, asker.source, candidates_3, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::Indefinite;
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(chosen_3).expect("a reference that always names a player")) };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4);  } }); }
g.end_sentence();
hoisted_5.extend(objects_4.clone());
}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
g.delayed_trigger(DelayedWhen::Step { step: Step::EndOfCombat, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Blocks { blocker, attacker, declared: true } = event { let id = *blocker; let subject = Target::Object(id); let data = [attacker.0, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_7(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let flip_8 = g.flip_coin(player_8);
if { let q: &dyn Query = g; !flip_8 } {
let player_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_9) = cantrip_mechanics::choose::player(g, player_9, asker.source, candidates_9, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let objects_10 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_10 = { let q: &dyn Query = g; Some(asker.source) }; let duration_10 = Duration::Indefinite;
let kind_10_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(chosen_9).expect("a reference that always names a player")) };
if let Some(source) = source_10 { cantrip_api::instruction(g, |g| { for &object in &objects_10 { g.add_effect(source, object, kind_10_0_0, duration_10);  } }); }
g.end_sentence();
hoisted_11.extend(objects_10.clone());
}
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
g.delayed_trigger(DelayedWhen::Step { step: Step::EndOfCombat, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 1), Target::None);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Fickle Efreet", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Efreet]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever this creature attacks or blocks, flip a coin at end of combat. If you lose the flip, an opponent gains control of this creature.").targets(&[]),triggered(when_asked(EventMask(EventMask::BLOCKS.0), ability_1_event), ability_1, "Whenever this creature attacks or blocks, flip a coin at end of combat. If you lose the flip, an opponent gains control of this creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_1, "Whenever this creature attacks or blocks, flip a coin at end of combat."),delayed_trigger(delayed_7, "Whenever this creature attacks or blocks, flip a coin at end of combat.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
