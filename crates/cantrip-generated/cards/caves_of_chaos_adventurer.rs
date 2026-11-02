// Caves of Chaos Adventurer
// status: planned
// requires: a play for the turn read as a play now
// Trample
// When this creature enters, you take the initiative.
// Whenever this creature attacks, exile the top card of your library. If you've completed a dungeon, you may play that card this turn without paying its mana cost. Otherwise, you may play that card this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.take_initiative(player); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.controller).is_some_and(|player: PlayerId| q.history().any(EventQuery::kind(EventMask::DUNGEON_COMPLETED).player(player))) } {
planned!("a-play-for-the-turn-read-as-a-play-now", Optional, "body" = "{\"instructions\":[{\"args\":{\"operation\":{\"args\":{\"action\":\"Either\",\"actor\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"cards\":{\"args\":{\"args\":{\"index\":0,\"phase\":\"After\",\"result\":0},\"kind\":\"Result\"},\"kind\":\"One\"},\"cost\":\"Free\",\"count\":{\"Exactly\":{\"args\":1,\"kind\":\"Constant\"}},\"into_exile\":false},\"kind\":\"PlayCards\"},\"result\":1},\"kind\":\"Effect\"}]}", "decider" = { let q: &dyn Query = g; Some(asker.controller) });
g.end_sentence();
} else {
let cards_2_0 = { let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let players_2_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_2_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_2_0 { g.allow_play(card, player, PlayGrant { lands: true, expiry: Expiry::EndOfTurn, ..PlayGrant::CAST }); } } } });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Caves of Chaos Adventurer", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Barbarian]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, you take the initiative.").targets(&[]),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever this creature attacks, exile the top card of your library. If you've completed a dungeon, you may play that card this turn without paying its mana cost. Otherwise, you may play that card this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
