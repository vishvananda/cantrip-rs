// Arachnus Web
// status: ready
// Enchant creature
// Enchanted creature can't attack or block, and its activated abilities can't be activated.
// At the beginning of the end step, if enchanted creature's power is 4 or greater, destroy this Aura.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { let source = asker.source; let asker = &Asker::new(source, q.controller(source)); !(q.state().obj(asker.source).attached_to == Some(attacker)) }
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(q.state().obj(asker.source).attached_to == Some(blocker) && true) }
#[allow(unused_variables)] fn ability_1_2_rule(q: &dyn Query, asker: &Asker, object: ObjectId, ability: &ActivatedDef) -> bool { !(q.state().obj(object).zone.is(ZoneKind::Battlefield) && q.state().obj(asker.source).attached_to == Some(object)) }
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).is_some_and(|candidate: ObjectId| q.chars(candidate).power.is_some_and(|v| (i64::from(v)) >= (4i64))) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Arachnus Web", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of the end step, if enchanted creature's power is 4 or greater, destroy this Aura.").targets(&[]).condition(ability_2_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_1_1_rule], block_requirements: &[], attack_rules: &[ability_1_0_rule], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), activation_rules: &[ability_1_2_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
