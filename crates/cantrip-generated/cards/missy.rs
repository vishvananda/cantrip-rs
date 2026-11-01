// Missy
// status: planned
// requires: instruction ChooseOne
// requires: keyword action ChaosEnsues
// requires: move arrival state
// requires: per-player instruction with several steps
// Whenever another nonartifact creature dies, return it to the battlefield under your control face down and tapped. It's a 2/2 Cyberman artifact creature.
// At the beginning of your end step, each opponent faces a villainous choice — Each artifact creature you control deals 1 damage to that opponent, or you draw a card and chaos ensues.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (!(Some(asker.source) == Some(id)) && (!(q.chars(id).is(CardType::Artifact)) && q.chars(id).is(CardType::Creature))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_0 = ({ let q: &dyn Query = g; ((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_0 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_0 { let controller = player_0; moved.extend(planned_value!("move arrival state", None::<ObjectId> => cantrip_mechanics::enter::in_state(g, o, controller, item, "[FaceDownAs(FaceDownAs { power_toughness: Term { family: \"PT\", form: \"PT\", operands: [Integer(2), Integer(2)] }, creature_type: Some(Vocabulary { family: \"CreatureType\", form: \"Cyberman\" }), artifact: true })]", &[], true)?)); } moved };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
planned!("per-player-instruction-with-several-steps" => g.begin_batch(););
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let decider_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_2, asker.source, vec![Label::Card(card_2, 0), Label::Card(card_2, 1)])?) {
0 => {
let sources_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let mut batch_3: Vec<(ObjectId, Target, u16)> = Vec::new(); for &source in &sources_3 { let amount = { let q: &dyn Query = g; (1i64).max(0) as u16 }; for t in { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() } { batch_3.push((source, t, amount)); } } let dealt_3 = g.damage_together(&batch_3); let done_3: i64 = dealt_3.iter().map(|&n| i64::from(n)).sum();
g.end_sentence();
}
_ => {
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
g.end_sentence();
planned!("keyword-action-chaos-ensues" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::planechase::chaos_ensues(g, player)?; });
g.end_sentence();
}
}
    } 
planned!("per-player-instruction-with-several-steps" => g.end_batch(););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Missy", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::TimeLord,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Whenever another nonartifact creature dies, return it to the battlefield under your control face down and tapped. It's a 2/2 Cyberman artifact creature.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your end step, each opponent faces a villainous choice — Each artifact creature you control deals 1 damage to that opponent, or you draw a card and chaos ensues.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
