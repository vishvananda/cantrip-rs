// Acererak the Archlich
// status: planned
// requires: condition CompletedDungeon
// requires: engine: venture into the dungeon reaches only Undercity
// When Acererak enters, if you haven't completed Tomb of Annihilation, return Acererak to its owner's hand and venture into the dungeon.
// Whenever Acererak attacks, for each opponent, you create a 2/2 black Zombie creature token unless that player sacrifices a creature of their choice.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { !(planned_value!("condition CompletedDungeon", false => Some(asker.controller).is_some_and(|p: PlayerId| q.history().completed_dungeon(p, "Tomb of Annihilation")))) }
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
planned!("engine-venture-into-the-dungeon-reaches-only-undercity" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.venture(player)?; });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_9 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Zombie).name, token_words: token!(Zombie).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_2.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_2.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_2.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
let mut each_7 = vec![Default::default(); players_2.len()];
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let payer_3 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let (paid_3, after_3, before_3, picked_3): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let objects_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_4 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_4: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_4 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; for player in players_4 { let eligible: Vec<ObjectId> = objects_4.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_4.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() < n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_4.clone(), &[]))? }; for object in taken { picks_4.push((player, object)); } }
let sacrificed_4 = g.sacrifice_each(&picks_4);
let paid_4 = !picks_4.is_empty();
(paid_4, { let q: &dyn Query = g; (sacrificed_4).clone() }, { let q: &dyn Query = g; (objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
 each_5[index_2] = after_3.clone(); all_5.extend(each_5[index_2].iter().copied());
each_6[index_2] = { let q: &dyn Query = g; before_3.clone() }; all_6.extend(each_6[index_2].iter().copied());
each_7[index_2] = paid_3;
   } 
let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_8 = Vec::new();
let count_8_0 = { let q: &dyn Query = g; (({ let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| !(Some(bound_0).and_then(|p: PlayerId| players_2.iter().position(|&next| next == p)).is_some_and(|at| (each_7[at] as i64) != 0))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }; all.retain(|p: &PlayerId| other.contains(p)); all }).len() as i64).max(0) as u32 }; let def_8_0 = g.card_id(&token_9::CARD);
for player in players_8 {
tokens_8.extend(std::iter::repeat((def_8_0,player)).take(count_8_0 as usize));
} let words_8 = g.words(asker.source);
let created_8 = g.create_tokens_worded(&tokens_8, false, None, words_8)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Acererak the Archlich", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Acererak enters, if you haven't completed Tomb of Annihilation, return Acererak to its owner's hand and venture into the dungeon.").targets(&[]).condition(ability_0_if),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Acererak attacks, for each opponent, you create a 2/2 black Zombie creature token unless that player sacrifices a creature of their choice.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_9::CARD];
