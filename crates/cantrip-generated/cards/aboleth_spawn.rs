// Aboleth Spawn
// status: planned
// requires: trigger event CausesTrigger
// Flash
// Ward {2}
// Probing Telepathy — Whenever a creature entering under an opponent's control causes a triggered ability of that creature to trigger, you may copy that ability. You may choose new targets for the copy.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let copier_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let t = Target::Ability(item.data[0]); g.copy_ability(t, copier_0, true)?;
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-causes-trigger" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Triggered { source, ability, cause } = event { let id = *source; let subject = Target::Object(id); let on_stack = cantrip_mechanics::triggers::triggered_ability_on_stack(q, event); let data = [on_stack.map_or(0, |i: StackItem| i.serial), on_stack.map_or(0, |i: StackItem| u32::from(i.controller.0))]; let cause = &*cause; if (if let Event::ZoneChange { old, new, from, to, .. } = cause { let id = *new; let subject = Target::Object(id); let data = [new.0, u32::MAX]; to.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))) } else { false }) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Aboleth Spawn", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fish,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward::<2>("Ward {2}"),triggered(when_asked(planned_value!("trigger event CausesTrigger", EventMask(0) => EventMask(EventMask::TRIGGERED.0)), ability_2_event), ability_2, "Probing Telepathy — Whenever a creature entering under an opponent's control causes a triggered ability of that creature to trigger, you may copy that ability. You may choose new targets for the copy.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
