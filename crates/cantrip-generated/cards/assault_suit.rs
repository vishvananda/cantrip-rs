// Assault Suit
// status: ready
// Equipped creature gets +2/+2, has haste, can't attack you or planeswalkers you control, and can't be sacrificed.
// At the beginning of each opponent's upkeep, you may have that player gain control of equipped creature until end of turn. If you do, untap it.
// Equip {3}
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Haste); }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((2i64) as i16, (2i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { !(q.state().obj(asker.source).attached_to == Some(attacker) && (match defender { Target::Player(p) => (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => true && q.chars(id).is(CardType::Planeswalker) && (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false })) }
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.state().obj(asker.source).attached_to == Some(id) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
let objects_1 = { let q: &dyn Query = g; (Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::Control(item.subject.player().expect("a reference that always names a player")) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
hoisted_2.extend(objects_1.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let objects_3 = { let q: &dyn Query = g; (Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_3: Vec<ObjectId> = objects_3.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_3 { g.untap(object); } });
g.end_sentence();
hoisted_4.extend(turned_3.clone());
hoisted_5.extend(objects_3.clone());
hoisted_6.extend({ let q: &dyn Query = g; turned_3.clone() });
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_3_target_0)];
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let attachments_7 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_7 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; if let Some(host) = host_7 { cantrip_api::instruction(g, |g| { for attachment in attachments_7 { g.attach(attachment, host); } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Assault Suit", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Equipment]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of each opponent's upkeep, you may have that player gain control of equipped creature until end of turn. If you do, untap it.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_3, "Equip {3}").targets(ABILITY_3_TARGETS).equip().sorcery_speed()], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_0_apply_1, "Equipped creature gets +2/+2, has haste, can't attack you or planeswalkers you control, and can't be sacrificed."),static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_0_apply_0, "Equipped creature gets +2/+2, has haste, can't attack you or planeswalkers you control, and can't be sacrificed.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[ability_1_0_rule], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, sacrifice_bans: &[ability_1_1_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
