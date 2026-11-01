// Sin, Spira's Punishment
// status: ready
// Flying
// Whenever Sin enters or attacks, exile a permanent card from your graveyard at random, then create a tapped token that's a copy of that card. If the exiled card is a land card, repeat this process.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
loop { let mut again_0 = false;
let mut pool_1 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_1: Vec<ObjectId> = Vec::new(); while picked_1.len() < count_1 && !pool_1.is_empty() { let at = g.choose_at_random(pool_1.len()); picked_1.push(pool_1.remove(at)); }
let objects_2 = { let q: &dyn Query = g; picked_1.clone() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
let original_3 = { let q: &dyn Query = g; (moved_2.clone()).get(0 as usize).copied() }; let values_3 = { let q: &dyn Query = g; original_3.map(|original: ObjectId| q.copiable_values(original)) }; let count_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_3 = match values_3 { Some(values) => g.create_token_copies_tapped(values, player_3, count_3, true)?, None => Vec::new() };
g.end_sentence();
if { let q: &dyn Query = g; (((objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
again_0 = true;
g.end_sentence();
}
g.end_sentence();
hoisted_5.extend(moved_2.clone());
hoisted_6.extend(picked_1.clone());
hoisted_7.extend(created_3.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
if !again_0 { break; } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sin, Spira's Punishment", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Leviathan,Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Sin enters or attacks, exile a permanent card from your graveyard at random, then create a tapped token that's a copy of that card. If the exiled card is a land card, repeat this process.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
