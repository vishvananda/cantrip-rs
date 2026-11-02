// Vito, Fanatic of Aclazotz
// status: ready
// Flying
// Whenever you sacrifice another permanent, you gain 2 life if this is the first time this ability has resolved this turn. If it's the second time, each opponent loses 2 life. If it's the third time, create a 4/3 white and black Vampire Demon creature token with flying.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, reason: ZoneReason::Sacrifice, .. } = event { let id = *old; let player = q.controller(id); let subject = Target::Object(id); let data = [new.0, u32::from(player.0)]; if from.is(ZoneKind::Battlefield) && Some(asker.controller) == Some(player) && (!(Some(asker.source) == Some(id)) && true) { return Some(Fired { subject, data }); } }
 None }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Vampire Demon).name, token_words: token!(Vampire Demon).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire,Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(5)), power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (q.history().resolutions(item.object, item.ability) as i64 + 1) == (1i64) } {
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; (q.history().resolutions(item.object, item.ability) as i64 + 1) == (2i64) } {
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; (q.history().resolutions(item.object, item.ability) as i64 + 1) == (3i64) } {
let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_2 = Vec::new();
let count_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_2_0 = g.card_id(&token_3::CARD);
for player in players_2 {
tokens_2.extend(std::iter::repeat((def_2_0,player)).take(count_2_0 as usize));
} let words_2 = g.words(asker.source);
let created_2 = g.create_tokens_worded(&tokens_2, false, None, words_2)?;
g.end_sentence();
hoisted_4.extend(created_2.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Vito, Fanatic of Aclazotz", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire,Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever you sacrifice another permanent, you gain 2 life if this is the first time this ability has resolved this turn. If it's the second time, each opponent loses 2 life. If it's the third time, create a 4/3 white and black Vampire Demon creature token with flying.").targets(&[]).on_sacrifice()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD];
