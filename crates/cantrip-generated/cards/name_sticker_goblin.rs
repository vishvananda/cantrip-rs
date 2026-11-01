// "Name Sticker" Goblin
// status: ready
// When this creature enters from anywhere other than a graveyard or exile, if it's on the battlefield and you control 9 or fewer creatures named "Name Sticker" Goblin, roll a 20-sided die.
// 1-6 | Add {R}{R}{R}{R}.
// 7-14 | Add {R}{R}{R}{R}{R}.
// 15-20 | Add {R}{R}{R}{R}{R}{R}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) && !from.is(ZoneKind::Graveyard) && !from.is(ZoneKind::Exile) && (subject.object().is_some_and(|candidate: ObjectId| (q.state().obj(candidate).became.is_none() && q.state().obj(candidate).zone.kind() == ZoneKind::Battlefield)) && ((q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) && q.chars(id).name == "\"Name Sticker\" Goblin") && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) <= (9i64)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; (item.subject.object().is_some_and(|candidate: ObjectId| (q.state().obj(candidate).became.is_none() && q.state().obj(candidate).zone.kind() == ZoneKind::Battlefield)) && ((q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) && q.chars(id).name == "\"Name Sticker\" Goblin") && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) <= (9i64)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_0 = i64::from(g.roll_die(player_0, 20).result);
g.end_sentence();
if { let q: &dyn Query = g; ((value_0) >= (1i64) && (value_0) <= (6i64)) } {
let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (4i64).max(0) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::R, mana_2_0);}
g.end_sentence();
}
if { let q: &dyn Query = g; ((value_0) >= (7i64) && (value_0) <= (14i64)) } {
let mana_players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_4_0 = { let q: &dyn Query = g; (5i64).max(0) as u32 };
for player in mana_players_4 {
g.add_mana(player, Mana::R, mana_4_0);}
g.end_sentence();
}
if { let q: &dyn Query = g; ((value_0) >= (15i64) && (value_0) <= (20i64)) } {
let mana_players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_6_0 = { let q: &dyn Query = g; (6i64).max(0) as u32 };
for player in mana_players_6 {
g.add_mana(player, Mana::R, mana_6_0);}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "\"Name Sticker\" Goblin", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Goblin,Subtype::Guest]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters from anywhere other than a graveyard or exile, if it's on the battlefield and you control 9 or fewer creatures named \"Name Sticker\" Goblin, roll a 20-sided die.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
