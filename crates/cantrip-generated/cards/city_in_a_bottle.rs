// City in a Bottle
// status: planned
// requires: object collection EventGroup
// requires: object filter NameFirstPrintedIn
// requires: trigger event ObjectBecomes
// Whenever one or more other nontoken permanents with a name originally printed in the Arabian Nights expansion are on the battlefield, their controllers sacrifice them.
// Players can't cast spells or play lands with a name originally printed in the Arabian Nights expansion.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; planned_value!("object collection EventGroup", Vec::<ObjectId>::new() => q.event_objects(item).to_vec()) }; let players_0 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_0: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_0 { let controller = g.controller(object); if players_0.contains(&controller) && g.can_sacrifice(controller, object) { picks_0.push((controller, object)); } }
let sacrificed_0 = g.sacrifice_each(&picks_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && planned_value!("object filter NameFirstPrintedIn", false => q.chars(spell).first_printing == Printing::ArabianNights)) }
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, player: PlayerId, land: ObjectId, face: CardId) -> bool { !(true && planned_value!("object filter NameFirstPrintedIn", false => q.chars(land).first_printing == Printing::ArabianNights)) }
planned_item!("trigger-event-object-becomes" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StateChecked { object } = event { let id = *object; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if (!(Some(asker.source) == Some(id)) && (!(q.state().obj(id).token) && planned_value!("object filter NameFirstPrintedIn", false => q.chars(id).first_printing == Printing::ArabianNights))) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "City in a Bottle", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event ObjectBecomes", EventMask(0) => EventMask(EventMask::OBJECT_STATE.0)), ability_0_event), ability_0, "Whenever one or more other nontoken permanents with a name originally printed in the Arabian Nights expansion are on the battlefield, their controllers sacrifice them.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, land_play_rules: &[ability_1_1_rule], spell_rules: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
