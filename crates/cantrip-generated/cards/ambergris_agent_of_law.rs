// Ambergris, Agent of Law
// status: ready
// Haste
// Whenever Ambergris attacks, you may discard your hand and draw two cards. If you do, other creatures you control get +X/+X until end of turn, where X is the number of cards you've discarded this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_1 = Vec::new(); for player in players_1 { cards_1.extend(g.state().zone(Zone::hand(player)).to_vec()); } let discarded_1 = g.discard_together(&cards_1);
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (!(item.subject.object() == Some(id)) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfTurn;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::Pump((q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true && !*undefined { return Some(Fired { subject, data }); } }  None }).count() as i64).clamp(-32768, 32767) as i16, (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true && !*undefined { return Some(Fired { subject, data }); } }  None }).count() as i64).clamp(-32768, 32767) as i16) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
hoisted_4.extend(discarded_1.clone());
hoisted_5.extend(objects_3.clone());
hoisted_6.extend({ let q: &dyn Query = g; discarded_1.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ambergris, Agent of Law", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dwarf,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Ambergris attacks, you may discard your hand and draw two cards. If you do, other creatures you control get +X/+X until end of turn, where X is the number of cards you've discarded this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
