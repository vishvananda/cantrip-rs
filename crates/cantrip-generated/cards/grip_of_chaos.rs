// Grip of Chaos
// status: planned
// requires: new targets chosen at random
// requires: trigger event OntoStack
// Whenever a spell or ability is put onto the stack, if it has a single target, reselect its target at random. (Select from among all legal targets.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| q.state().stack.iter().find(|i| i.object == candidate).is_some_and(|i| q.state().targets(i).len() == 1)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
planned!("new-targets-chosen-at-random" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { cantrip_mechanics::copy::retarget(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, spell, NewTargets::Random)?; });
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-onto-stack" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::PutOnStack { object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if true && subject.object().is_some_and(|candidate: ObjectId| q.state().stack.iter().find(|i| i.object == candidate).is_some_and(|i| q.state().targets(i).len() == 1)) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Grip of Chaos", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event OntoStack", EventMask(0) => EventMask(EventMask::ONTO_STACK.0)), ability_0_event), ability_0, "Whenever a spell or ability is put onto the stack, if it has a single target, reselect its target at random.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
