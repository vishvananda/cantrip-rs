// Redoubled Stormsinger
// status: planned
// requires: tapped token copies of several objects at once
// First strike
// Whenever this creature attacks, for each creature token you control that entered this turn, create a tapped and attacking token that's a copy of that token. At the beginning of the next end step, sacrifice those tokens.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; about(q, item, 0) }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_3 { let controller = g.controller(object); if players_3.contains(&controller) && g.can_sacrifice(controller, object) { picks_3.push((controller, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("tapped-token-copies-of-several-objects-at-once", CreateTokens, "entry" = "{\"counters\":[],\"states\":[{\"Attacking\":{\"or_planeswalker\":false,\"player\":()}}],\"tapped\":true}", "players" = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, "tokens" = "[{\"amount\":{\"args\":1,\"kind\":\"Constant\"},\"recipe\":{\"CopyEach\":{\"adjustments\":[{\"args\":{\"family\":\"TokenCopyEffects\",\"form\":\"NoTokenCopyEffects\",\"operands\":[]},\"kind\":\"Term\"}],\"objects\":{\"args\":{\"args\":[{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"kind\":\"Token\"},{\"args\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"kind\":\"ControlledBy\"},{\"args\":{\"args\":{\"comparison\":\"GreaterOrEqual\",\"left\":{\"args\":{\"event\":{\"ZoneChange\":{\"from\":(),\"objects\":{\"args\":{\"kind\":\"Candidate\"},\"kind\":\"SameObject\"},\"to\":\"Battlefield\"}},\"measure\":\"Occurrences\",\"window\":\"ThisTurn\"},\"kind\":\"EventHistory\"},\"right\":{\"args\":1,\"kind\":\"Constant\"}},\"kind\":\"Compare\"},\"kind\":\"Condition\"}],\"kind\":\"All\"},\"kind\":\"Battlefield\"}}},\"words\":\"CreatingEffect\"}]");
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_0) }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }

pub static CARD: CardDef = CardDef { name: "Redoubled Stormsinger", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Orc,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, for each creature token you control that entered this turn, create a tapped and attacking token that's a copy of that token. At the beginning of the next end step, sacrifice those tokens.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_2, "At the beginning of the next end step, sacrifice those tokens.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
