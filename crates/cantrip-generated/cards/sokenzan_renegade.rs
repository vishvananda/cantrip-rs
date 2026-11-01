// Sokenzan Renegade
// status: planned
// requires: engine: no Bushido
// Bushido 1 (Whenever this creature blocks or becomes blocked, it gets +1/+1 until end of turn.)
// At the beginning of your upkeep, if a player has more cards in hand than each other player, the player who has the most cards in hand gains control of this creature.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().any(|bound_0: PlayerId| (Some(bound_0).map_or(0i64, |p: PlayerId| q.state().zone(Zone::hand(p)).len() as i64)) > ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| q.state().zone(Zone::hand(p)).len() as i64).max().unwrap_or(0))) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let (carried_2_0,) = if let Some(present_2_0) = { let q: &dyn Query = g; { let rows: Vec<(PlayerId, i64)> = (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| (p, q.state().zone(Zone::hand(p)).len() as i64)).collect(); let best = rows.iter().map(|r| r.1).max(); let mut top = rows.iter().filter(|r| Some(r.1) == best).map(|r| r.0); match (top.next(), top.next()) { (Some(p), None) => Some(p), _ => None } } } { let objects_1 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::Indefinite;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::Control(present_2_0) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
(objects_1.clone(),) } else { Default::default() };
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-bushido" => abilities: cantrip_mechanics::bushido::trigger::<1>("Bushido 1"));

pub static CARD: CardDef = CardDef { name: "Sokenzan Renegade", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ogre,Subtype::Samurai,Subtype::Mercenary]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, if a player has more cards in hand than each other player, the player who has the most cards in hand gains control of this creature.").targets(&[]).condition(ability_1_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
