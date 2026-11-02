// Vesuvan Shapeshifter
// status: planned
// requires: ability AsEvent
// requires: engine: no effect turns a permanent face down
// As this creature enters or is turned face up, you may choose another creature on the battlefield. If you do, until this creature is turned face down, it becomes a copy of that creature, except it has "At the beginning of your upkeep, you may turn this creature face down."
// Morph {1}{U}
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn granted_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn granted_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
planned!("engine-no-effect-turns-a-permanent-face-down" => if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.turn_face_down(object); });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn lasting_0(q: &dyn Query, e: &ContinuousEffect) -> bool { let asker = &Asker::new(e.source, e.controller); is_live(q, e.source) && q.is_on_battlefield(e.source) && !(q.state().obj(e.source).face_down) }
planned_item!("ability-as-event" => #[allow(unused_variables)] fn ability_0_as_object(g: &mut dyn Game, asker: &Asker) -> Option<ObjectId> { { let q: &dyn Query = g; Some(asker.source) } });
planned_item!("ability-as-event" => #[allow(unused_variables)] fn ability_0_as_actions(g: &mut dyn Game, asker: &Asker) -> Result<(), Suspend> { cantrip_mechanics::as_events::perform(g, asker, &[EnterOrFaceUpAction::MayActions(&[EnterOrFaceUpAction::EnterAsACopyOfAPermanentUntil({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }, CopyEffects::CopyEffects(&[CopyEffect::AddAbility(&[GrantedAbility::Delayed(0)])]), { let q: &dyn Query = g; Duration::AsLongAs({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) })])]) });
planned_item!("ability-as-event" => as_events: AsEventDef { event: AsEvent::EntersOrTurnsFaceUp, object: ability_0_as_object, actions: ability_0_as_actions });

pub static CARD: CardDef = CardDef { name: "Vesuvan Shapeshifter", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), granted_0_event), granted_0, "At the beginning of your upkeep, you may turn this creature face down.").targets(&[])], enters: None, modal: None, enchant: None, lasting: &[Lasting { text: "its condition holds", holds: lasting_0 }], ways: &[cantrip_mechanics::morph::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), "Morph {1}{U}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
