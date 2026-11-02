// Territorial Hellkite
// status: ready
// Flying, haste
// At the beginning of combat on your turn, choose an opponent at random that this creature didn't attack during your last combat. This creature attacks that player this combat if able. If you can't choose an opponent this way, tap this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut chosen_player_kept = None;
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut paid_1 = false; 'paid_1: {
let pool_3: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| !({ let of = Some(asker.controller); let history = q.history(); let all = history.all(); let combat = |s: Step| matches!(s, Step::BeginCombat | Step::DeclareAttackers | Step::DeclareBlockers | Step::CombatDamage | Step::EndOfCombat); let fighting = Some(q.state().turn.active) == of && combat(q.state().turn.step()); let mut begins = history.select(EventQuery::kind(EventMask::STEP_BEGAN).step(Step::BeginCombat)).filter(|r| Some(r.active) == of).map(|r| r.serial as usize); let mut last = begins.next_back(); if fighting { last = begins.next_back(); } match last { Some(at) => { let end = all[at + 1..].iter().position(|r| !combat(r.step) || matches!(r.event, Event::StepBegan { step: Step::BeginCombat })).map_or(all.len(), |n| at + 1 + n); all[at..end].iter() } None => all[..0].iter() } }.filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if Some(asker.source) == Some(id) && match *defender { Target::Player(p) => Some(bound_0) == Some(p), _ => false } { return Some(Fired { subject, data }); } }  None }).next().is_some())).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } }; if pool_3.is_empty() { break 'paid_1; } let chosen_3 = pool_3[g.choose_at_random(pool_3.len())];
g.end_sentence();
chosen_player_kept = Some(chosen_3);
paid_1 = true;
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_1 as i64) != 0) } {
let objects_4 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::EndOfCombat;
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::MustAttackAt(Target::Player(chosen_player_kept.expect("a reference that always names a player"))) };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4);  } }); }
g.end_sentence();
hoisted_5.extend(objects_4.clone());
} else {
let objects_6 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_6: Vec<ObjectId> = objects_6.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_6);
g.end_sentence();
hoisted_7.extend(turned_6.clone());
hoisted_8.extend(objects_6.clone());
hoisted_9.extend({ let q: &dyn Query = g; turned_6.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Territorial Hellkite", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of combat on your turn, choose an opponent at random that this creature didn't attack during your last combat. This creature attacks that player this combat if able. If you can't choose an opponent this way, tap this creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
