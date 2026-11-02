// Parker Luck
// status: ready
// At the beginning of your end step, two target players each reveal the top card of their library. They each lose life equal to the mana value of the card revealed by the other player. Then they each put the card they revealed into their hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(2, 2)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (q.state().apnap().filter(|&p| t_0.iter().any(|t| t.player() == Some(p))).collect::<Vec<PlayerId>>()).into_iter().filter_map(|p: PlayerId| q.state().zone(Zone::library(p)).first().copied()).collect::<Vec<ObjectId>>() }; let mut owners_0: Vec<PlayerId> = objects_0.iter().map(|o| g.state().obj(*o).owner).collect(); owners_0.dedup(); for owner in owners_0 { let cards: Vec<ObjectId> = objects_0.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&p| t_0.iter().any(|t| t.player() == Some(p))).collect::<Vec<PlayerId>>() };
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let amount_2 = { let q: &dyn Query = g; (((objects_0.clone()).into_iter().filter(|&id| ({ let other = Some(bound_0); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.state().obj(id).owner)).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).sum::<i64>()).clamp(0, 2147483647) as i32 }; let players_2 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.lose_life(player, amount_2); }
    } 
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&p| t_0.iter().any(|t| t.player() == Some(p))).collect::<Vec<PlayerId>>() };
let mut gathered_4: Vec<ObjectId> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { gathered_4.extend({ let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() });
    } 
let objects_6 = { let q: &dyn Query = g; gathered_4.clone() }; let moved_6 = g.return_to_hand_revealed(&objects_6);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Parker Luck", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your end step, two target players each reveal the top card of their library. They each lose life equal to the mana value of the card revealed by the other player. Then they each put the card they revealed into their hand.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
