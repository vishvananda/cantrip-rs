// River Song
// status: planned
// requires: engine: a two-word subtype in a card's type line
// requires: player rule Unique
// Meet in Reverse — You draw cards from the bottom of your library rather than the top.
// Spoilers — Whenever an opponent scries, surveils, or searches their library, put a +1/+1 counter on River Song. Then River Song deals damage to that player equal to its power.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Scried { player, looked, .. } = event { let subject = Target::Player(*player); let data = [*looked as u32, 0]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&*player) { return Some(Fired { subject, data }); } }
if let Event::Surveilled { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&*player) { return Some(Fired { subject, data }); } }
if let Event::Searched { player, library } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if player == library && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::PlusOne, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let amount_1 = { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as u16 }; let recipients_1 = { let q: &dyn Query = g; (item.subject.player().into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_1 { let batch: Vec<(ObjectId, Target, u16)> = recipients_1.iter().map(|t| (source, *t, amount_1)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
planned_item!("player-rule-unique" => statics: player_rule(ability_0_0_applies, PlayerRule::DrawsFromBottomOfLibrary));

pub static CARD: CardDef = CardDef { name: "River Song", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,planned_value!("engine: a two-word subtype in a card's type line", Subtype::TimeLord => Subtype::TimeLord),Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SCRIED.0 | EventMask::SURVEILLED.0 | EventMask::SEARCHED.0), ability_1_event), ability_1, "Spoilers — Whenever an opponent scries, surveils, or searches their library, put a +1/+1 counter on River Song. Then River Song deals damage to that player equal to its power.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
