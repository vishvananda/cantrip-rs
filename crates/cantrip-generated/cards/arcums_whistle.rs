// Arcum's Whistle
// status: ready
// {3}, {T}: Choose target non-Wall creature the active player has controlled continuously since the beginning of the turn. That player may pay {X}, where X is that creature's mana value. If they don't pay, the creature attacks this turn if able, and at the beginning of the next end step, destroy it if it didn't attack this turn. Activate only before attackers are declared.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Wall))) && q.chars(id).is(CardType::Creature) && Some(q.state().turn.active) == Some(q.controller(id)) && q.state().obj(id).controlled_since < q.state().turn.number))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn delayed_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; about(q, item, 0).first().copied().is_some_and(|candidate: ObjectId| !(({ let mut attackers: Vec<ObjectId> = q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Attacks { attacker, defending, known, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(candidate) == Some(id) { return Some(Fired { subject, data }); } }  None }).filter_map(|f: Fired| f.subject.object()).collect(); attackers.sort(); attackers.dedup(); attackers.len() as i64 }) >= (1i64))) } {
let objects_4 = { let q: &dyn Query = g; about(q, item, 1) }; let destroyed_4 = g.destroy_all(&objects_4);
g.end_sentence();
hoisted_5.extend(destroyed_4.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let payer_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let owed_0: u8 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).clamp(0, 255) as u8 }; let base_0 = ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }; let price_0 = ManaCost { generic: base_0.generic.saturating_add(owed_0.saturating_mul(base_0.x)), x: 0, ..base_0 }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::mana(price_0), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_0 as i64) != 0)) } {
let objects_1 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::MustAttack };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE); about.extend({ let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
hoisted_7.extend(objects_1.clone());
}
g.end_sentence();
 Ok(()) }
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { (q.state().turn.step() as u8) < (Step::DeclareAttackers as u8) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Arcum's Whistle", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{3}, {T}: Choose target non-Wall creature the active player has controlled continuously since the beginning of the turn. That player may pay {X}, where X is that creature's mana value. If they don't pay, the creature attacks this turn if able, and at the beginning of the next end step, destroy it if it didn't attack this turn. Activate only before attackers are declared.").targets(ABILITY_0_TARGETS).condition(ability_0_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_3, "If they don't pay, the creature attacks this turn if able, and at the beginning of the next end step, destroy it if it didn't attack this turn.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
