// Demonlord Belzenlok
// status: ready
// Flying, trample
// When Demonlord Belzenlok enters, exile cards from the top of your library until you exile a nonland card, then put that card into your hand. If the card's mana value is 4 or greater, repeat this process. Demonlord Belzenlok deals 1 damage to you for each card put into your hand this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
loop { let mut again_0 = false;
let owner_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_1, rest_1) = cantrip_mechanics::library::reveal_until_n(g, owner_1, count_1, &|q: &dyn Query, id: ObjectId| !(q.chars(id).is(CardType::Land))); let shown_1 = found_1.len() + rest_1.len(); let objects_1: Vec<ObjectId> = g.state().zone(Zone::library(owner_1)).iter().copied().take(shown_1).collect();
let objects_2 = { let q: &dyn Query = g; objects_1.clone() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (((moved_2.clone()).into_iter().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_3 = g.return_to_hand_revealed(&objects_3);
g.end_sentence();
if { let q: &dyn Query = g; (((moved_3.clone()).into_iter().filter(|&id| (i64::from(q.mana_value(id))) >= (4i64)).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
again_0 = true;
g.end_sentence();
}
g.end_sentence();
hoisted_5.extend(moved_2.clone());
hoisted_6.extend(moved_3.clone());
hoisted_7.extend(objects_1.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_9.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
if !again_0 { break; } }
g.end_sentence();
let source_10 = { let q: &dyn Query = g; Some(asker.source) }; let amount_10 = { let q: &dyn Query = g; ((1i64 * (hoisted_6.clone()).len() as i64)).clamp(0, 65535) as u16 }; let recipients_10 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_10 { let batch: Vec<(ObjectId, Target, u16)> = recipients_10.iter().map(|t| (source, *t, amount_10)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Demonlord Belzenlok", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elder,Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When Demonlord Belzenlok enters, exile cards from the top of your library until you exile a nonland card, then put that card into your hand. If the card's mana value is 4 or greater, repeat this process. Demonlord Belzenlok deals 1 damage to you for each card put into your hand this way.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
