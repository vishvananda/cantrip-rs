// Psychic Battle
// status: planned
// requires: trigger event TargetsChosen
// Whenever a player chooses one or more targets, each player reveals the top card of their library. The player who reveals the card with the greatest mana value may change the target or targets. If two or more cards are tied for greatest, the target or targets remain unchanged. Changing targets this way doesn't trigger abilities of permanents named Psychic Battle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().filter_map(|p: PlayerId| q.state().zone(Zone::library(p)).first().copied()).collect::<Vec<ObjectId>>() }; let mut owners_0: Vec<PlayerId> = objects_0.iter().map(|o| g.state().obj(*o).owner).collect(); owners_0.dedup(); for owner in owners_0 { let cards: Vec<ObjectId> = objects_0.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
if { let q: &dyn Query = g; (((objects_0.clone()).into_iter().filter(|&id| (objects_0.clone()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).max().is_some_and(|best| i64::from(q.mana_value(id)) == best)).collect::<Vec<ObjectId>>()).len() as i64) == (1i64) } {
let among: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| { let those: Vec<ObjectId> = objects_0.clone(); let extreme = those.iter().map(|&o| q.mana_value(o)).max(); those.iter().any(|&o| Some(q.state().obj(o).owner) == Some(bound_0) && Some(q.mana_value(o)) == extreme) }).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if among.len() > 1 { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
let chooser_2 = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }; let retarget_2: Option<Target> = { let q: &dyn Query = g; item.subject.object().map(Target::Object) }; if let Some(t) = retarget_2 {
if let Target::Object(spell) = t { g.choose_new_targets(spell, chooser_2)?; }
}

g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-targets-chosen" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TargetsChosen { object, player } = event { let id = *object; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if true { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Psychic Battle", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event TargetsChosen", EventMask(0) => EventMask(EventMask::TARGETS_CHOSEN.0)), ability_0_event), ability_0, "Whenever a player chooses one or more targets, each player reveals the top card of their library. The player who reveals the card with the greatest mana value may change the target or targets. If two or more cards are tied for greatest, the target or targets remain unchanged. Changing targets this way doesn't trigger abilities of permanents named Psychic Battle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
