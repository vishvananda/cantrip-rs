// Everybody Lives!
// status: planned
// requires: player rule for a while can't lose life
// requires: player rule for a while can't lose the game
// requires: player rule for a while can't win the game
// All creatures gain hexproof and indestructible until end of turn. Players gain hexproof until end of turn. Players can't lose life this turn and players can't lose the game or win the game this turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Hexproof])) };
let kind_0_1_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Indestructible])) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_0, duration_0);  } }); }
g.end_sentence();
let players_1_0 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_1_0 { g.add_player_effect(player, source, EffectKind::PlayerHexproof, Duration::EndOfTurn); } }
g.end_sentence();
planned!("player-rule-for-a-while-can-t-lose-life" => let players_2_0 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_2_0 { g.add_player_effect(asker.controller, source, EffectKind::CantLoseLife(player), Duration::EndOfTurn); } });
planned!("player-rule-for-a-while-can-t-lose-the-game" => let players_2_1 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_2_1 { g.add_player_effect(asker.controller, source, EffectKind::CantLoseGame(player), Duration::EndOfTurn); } });
planned!("player-rule-for-a-while-can-t-win-the-game" => let players_2_2 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_2_2 { g.add_player_effect(asker.controller, source, EffectKind::CantWinGame(player), Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Everybody Lives!", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
