// Stand or Fall
// status: planned
// requires: operation ChangeCombat
// At the beginning of combat on your turn, for each defending player, separate all creatures that player controls into two piles and that player chooses one. Only creatures in the chosen piles can block this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|player: PlayerId| q.state().opponents(q.state().turn.active).any(|o| o == player))).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut piles_0: Vec<(PlayerId, Vec<Vec<ObjectId>>, Option<usize>, Vec<PlayerId>)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let separator_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut rest_1: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let mut piles_1: Vec<Vec<ObjectId>> = Vec::new(); for _ in 1..2 { let (first, second) = cantrip_mechanics::choose::separate(g, Some(separator_1), asker.source, rest_1)?; piles_1.push(first); rest_1 = second; } piles_1.push(rest_1); let choosers_1: Vec<PlayerId> = Vec::<PlayerId>::new();
piles_0.push((separator_1, piles_1, None, choosers_1));
    } 
g.end_sentence();
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let chooser_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let from_2 = None::<Vec<PlayerId>>; let mut chosen_2: Vec<ObjectId> = Vec::new(); for row in piles_0.iter_mut() { if from_2.as_ref().is_some_and(|from| !from.contains(&row.0)) { continue; } let at = { let (chosen, _) = cantrip_mechanics::choose::pile(g, chooser_2, asker.source, row.1[0].clone(), row.1[1].clone())?; usize::from(chosen != row.1[0]) }; row.2 = Some(at); chosen_2.extend(row.1[at].iter().copied()); }
 each_3[index_0] = chosen_2.clone(); all_3.extend(each_3[index_0].iter().copied());
   } 
g.end_sentence();
planned!("operation-change-combat" => cantrip_mechanics::combat::change(g, asker, CombatChange::OnlyBlockers({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| piles_0.iter().any(|row| row.2.is_some_and(|at| row.1[at].contains(&id)))).collect::<Vec<ObjectId>>() }))?);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Stand or Fall", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of combat on your turn, for each defending player, separate all creatures that player controls into two piles and that player chooses one. Only creatures in the chosen piles can block this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
