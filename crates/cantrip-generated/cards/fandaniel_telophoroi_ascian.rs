// Fandaniel, Telophoroi Ascian
// status: ready
// Whenever you cast an instant or sorcery spell, surveil 1.
// At the beginning of your end step, each opponent may sacrifice a nontoken creature of their choice. Each opponent who doesn't loses 2 life for each instant and sorcery card in your graveyard.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::surveil(g, player_0, asker.source, amount_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut guard_1_0 = vec![false; players_1.len()];
let mut gathered_2: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { if true { guard_1_0[index_1] = true;
let objects_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let players_3 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_3 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; for player in players_3 { let eligible: Vec<ObjectId> = objects_3.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_3.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_3.clone(), &[]))? }; for object in taken { picks_3.push((player, object)); } }
gathered_2.extend(picks_3.iter().copied());
   }
 } 
let sacrificed_2 = g.sacrifice_each(&gathered_2);
g.end_sentence();
let amount_4 = { let q: &dyn Query = g; ((2i64 * (((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>()).len() as i64)).clamp(0, 2147483647) as i32 }; let players_4 = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| !(Some(bound_0).and_then(|p: PlayerId| players_1.iter().position(|&next| next == p)).is_some_and(|at| gathered_2.iter().any(|(who, _)| *who == players_1[at])))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } };
for player in players_4 { g.lose_life(player, amount_4); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Fandaniel, Telophoroi Ascian", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elder,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Whenever you cast an instant or sorcery spell, surveil 1.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your end step, each opponent may sacrifice a nontoken creature of their choice. Each opponent who doesn't loses 2 life for each instant and sorcery card in your graveyard.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
