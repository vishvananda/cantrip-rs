// Curse of Bounty
// status: planned
// requires: per-player Tap
// Enchant player
// Whenever enchanted player is attacked, untap all nonland permanents you control. Each opponent attacking that player untaps all nonland permanents they control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if true && match *defender { Target::Player(p) => Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_player) == Some(p), _ => false } { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_0: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_0 { g.untap(object); } });
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| { let (by, at) = (Some(bound_0), Some(PlayerId(item.data[1] as u8))); q.state().attackers.iter().any(|a| Some(q.controller(a.id)) == by && Some(a.defending) == at) }).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } };
planned!("per-player-tap" => g.begin_batch(););
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let objects_2 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_2: Vec<ObjectId> = objects_2.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_2 { g.untap(object); } });
 each_3[index_1] = objects_2.clone(); all_3.extend(each_3[index_1].iter().copied());
each_4[index_1] = turned_2.clone(); all_4.extend(each_4[index_1].iter().copied());
each_5[index_1] = { let q: &dyn Query = g; turned_2.clone() }; all_5.extend(each_5[index_1].iter().copied());
   } 
planned!("per-player-tap" => g.end_batch(););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Curse of Bounty", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura,Subtype::Curse]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever enchanted player is attacked, untap all nonland permanents you control. Each opponent attacking that player untaps all nonland permanents they control.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
