// Booby Trap
// status: planned
// requires: player rule RevealDraws
// As this artifact enters, choose an opponent and a card name other than a basic land card name.
// The chosen player reveals each card they draw.
// When the chosen player draws a card with the chosen name, sacrifice this artifact. If you do, this artifact deals 10 damage to that player.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let candidates_0 = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_0) = cantrip_mechanics::choose::player(g, player_0, entering.object, candidates_0, "Choose", 0)? else { return Ok(()) };
entering.chosen_player = Some(chosen_0);
let player_1 = entering.controller; let Answer::Name(Some(chosen_1)) = g.ask(player_1, RequestKind::ChooseName { source: entering.object, looked_at: Vec::new(), suggestion: None, kind: NameKind::OTHER_THAN_BASIC_LAND })? else { return Ok(()) };
entering.chosen_card = Some(chosen_1);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { (q.state().chosen_player(q.linked_to(asker)).into_iter().collect::<Vec<PlayerId>>()).contains(&player) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Drew { player, card } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if q.state().chosen_player(q.linked_to(asker)) == Some(*player) && q.state().chosen_card(q.linked_to(asker)).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, id, chosen)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_2: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_2 { let controller = g.controller(object); if players_2.contains(&controller) && g.can_sacrifice(controller, object) { picks_2.push((controller, object)); } }
let sacrificed_2 = g.sacrifice_each(&picks_2);
let paid_2 = !picks_2.is_empty();
g.end_sentence();
if { let q: &dyn Query = g; ((paid_2 as i64) != 0) } {
let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let amount_3 = { let q: &dyn Query = g; (10i64).max(0) as u16 }; let recipients_3 = { let q: &dyn Query = g; (Some(PlayerId(item.data[0] as u8)).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_3 { let batch: Vec<(ObjectId, Target, u16)> = recipients_3.iter().map(|t| (source, *t, amount_3)).collect(); g.damage_together(&batch); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("player-rule-reveal-draws" => statics: player_rule(ability_1_0_applies, PlayerRule::RevealDraws { first_during: None, optional: false }));

pub static CARD: CardDef = CardDef { name: "Booby Trap", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DREW.0), ability_2_event), ability_2, "When the chosen player draws a card with the chosen name, sacrifice this artifact. If you do, this artifact deals 10 damage to that player.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
