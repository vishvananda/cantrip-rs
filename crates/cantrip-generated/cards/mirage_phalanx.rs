// Mirage Phalanx
// status: planned
// requires: copy exception LosesAbility
// requires: engine: no Soulbond
// requires: object reference PairedWith
// requires: object state Paired
// Soulbond (You may pair this creature with another unpaired creature when either enters. They remain paired for as long as you control both of them.)
// As long as Mirage Phalanx is paired with another creature, each of those creatures has "At the beginning of combat on your turn, create a token that's a copy of this creature, except it has haste and loses soulbond. Exile it at end of combat."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| planned_value!("object state Paired", false => q.state().obj(candidate).paired_with.is_some()))) && ((Some(asker.source) == Some(affected) || planned_value!("object reference PairedWith", None::<ObjectId> => Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).paired_with)) == Some(affected))) }
#[allow(unused_variables)] fn granted_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn delayed_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = { let q: &dyn Query = g; about(q, item, 0) }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
 Ok(()) }
fn granted_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let original_1 = { let q: &dyn Query = g; Some(asker.source) }; let values_1 = { let q: &dyn Query = g; original_1.map(|original: ObjectId| planned_value!("copy exception LosesAbility", q.copiable_values(original).with_keyword(Keyword::Haste) => q.copiable_values(original).with_keyword(Keyword::Haste).losing(Lost::Soulbond))) }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_1 = match values_1 { Some(values) => g.create_token_copies_tapped(values, player_1, count_1, false)?, None => Vec::new() };
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_1.clone() }); about.push(ObjectId::NONE);  if about.iter().any(|&o| o != ObjectId::NONE) { g.delayed_trigger_about(DelayedWhen::Step { step: Step::EndOfCombat, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 1)); } }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
planned_item!("engine-no-soulbond" => abilities: cantrip_mechanics::soulbond::TRIGGER);

pub static CARD: CardDef = CardDef { name: "Mirage Phalanx", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_1_0_affects, ability_1_0_apply_0, "As long as Mirage Phalanx is paired with another creature, each of those creatures has \"At the beginning of combat on your turn, create a token that's a copy of this creature, except it has haste and loses soulbond. Exile it at end of combat.\"")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_3, "Exile it at end of combat."),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), granted_0_event), granted_0, "At the beginning of combat on your turn, create a token that's a copy of this creature, except it has haste and loses soulbond. Exile it at end of combat.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
