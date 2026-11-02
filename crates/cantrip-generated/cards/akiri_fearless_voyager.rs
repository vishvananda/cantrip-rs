// Akiri, Fearless Voyager
// status: ready
// Whenever you attack a player with one or more equipped creatures, draw a card.
// {W}: You may unattach an Equipment from a creature you control. If you do, tap that creature and it gains indestructible until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if (q.state().battlefield().iter().any(|&a| q.state().obj(a).attached_to == Some(id) && q.chars(a).has_subtype(Subtype::Equipment)) && q.chars(id).is(CardType::Creature)) && Some(asker.controller) == Some(q.controller(id)) && match *defender { Target::Player(p) => true, _ => false } { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let hosts_1: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let offered_1: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment))).collect::<Vec<ObjectId>>() }.into_iter().filter(|&e| g.state().obj(e).attached_to.is_some_and(|h| hosts_1.contains(&h))).collect(); let picked_1 = cantrip_mechanics::choose::cards(g, payer_1, asker.source, Purpose::Optional, offered_1, 1, true)?; let mut unattached_1: Vec<ObjectId> = Vec::new(); for e in picked_1 { if let Some(h) = g.state().obj(e).attached_to { g.unattach(e); unattached_1.push(h); } } let paid_1 = !unattached_1.is_empty();
g.end_sentence();
if { let q: &dyn Query = g; ((paid_1 as i64) != 0) } {
let objects_2 = { let q: &dyn Query = g; ((unattached_1.clone()).first().copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_2: Vec<ObjectId> = objects_2.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_2);
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; ((unattached_1.clone()).first().copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfTurn;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Indestructible])) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
hoisted_4.extend(turned_2.clone());
hoisted_5.extend(objects_2.clone());
hoisted_6.extend(objects_3.clone());
hoisted_7.extend({ let q: &dyn Query = g; turned_2.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Akiri, Fearless Voyager", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Kor,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever you attack a player with one or more equipped creatures, draw a card.").targets(&[]).once_per_batch().batch_by_data(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{W}: You may unattach an Equipment from a creature you control. If you do, tap that creature and it gains indestructible until end of turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
