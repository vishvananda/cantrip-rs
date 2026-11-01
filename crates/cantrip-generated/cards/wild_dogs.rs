// Wild Dogs
// status: ready
// At the beginning of your upkeep, if a player has more life than each other player, the player with the most life gains control of this creature.
// Cycling {2} ({2}, Discard this card: Draw a card.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().any(|bound_0: PlayerId| (({ let mut all = Some(bound_0).into_iter().collect::<Vec<PlayerId>>(); let other = { let rows: Vec<(PlayerId, i64)> = (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| (p, i64::from(q.state().player(p).life))).collect(); let best = rows.iter().map(|r| r.1).max(); let mut top = rows.iter().filter(|r| Some(r.1) == best).map(|r| r.0); match (top.next(), top.next()) { (Some(p), None) => Some(p), _ => None } }.into_iter().collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }).len() as i64) > (0i64)) }
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let (carried_2_0,) = if let Some(present_2_0) = { let q: &dyn Query = g; { let rows: Vec<(PlayerId, i64)> = (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| (p, i64::from(q.state().player(p).life))).collect(); let best = rows.iter().map(|r| r.1).max(); let mut top = rows.iter().filter(|r| Some(r.1) == best).map(|r| r.0); match (top.next(), top.next()) { (Some(p), None) => Some(p), _ => None } } } { let objects_1 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::Indefinite;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::Control(present_2_0) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
(objects_1.clone(),) } else { Default::default() };
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Wild Dogs", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dog]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, if a player has more life than each other player, the player with the most life gains control of this creature.").targets(&[]).condition(ability_0_if),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(DISCARD_THIS), ability_1, "Cycling {2}").targets(&[]).cycling().from(ZoneKind::Hand)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
