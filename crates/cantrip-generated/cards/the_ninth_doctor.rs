// The Ninth Doctor
// status: planned
// requires: engine: no untap event
// requires: operation AddPhases
// requires: turn window UntapStep
// Haste
// Into the TARDIS — Whenever The Ninth Doctor becomes untapped during your untap step, you get an additional upkeep step after this step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-add-phases" => g.add_phases({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, AddedPhases::Upkeep, { let q: &dyn Query = g; (1i64).max(0) as u32 }, None));
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-untap-event" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Untapped { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) && (planned_value!("turn window UntapStep", false => q.state().turn.is_in(TurnWindow::UntapStep)) && Some(asker.controller) == Some(q.state().turn.active)) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "The Ninth Doctor", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::TimeLord,Subtype::Doctor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("engine: no untap event", EventMask(0) => EventMask(EventMask::UNTAPPED.0)), ability_1_event), ability_1, "Into the TARDIS — Whenever The Ninth Doctor becomes untapped during your untap step, you get an additional upkeep step after this step.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
