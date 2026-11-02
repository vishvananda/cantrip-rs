// Stunning Reversal
// status: planned
// requires: replacement of LoseGame
// The next time you would lose the game this turn, instead draw seven cards and your life total becomes 1.
// Exile Stunning Reversal.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn made_1_then_steps(g: &mut dyn Game, asker: &Asker, affected: ObjectId, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let amount_2 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for player in players_3 { g.set_life_total(player, amount_3); }
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { let _ = (to, about); g.add_player_effect(item.controller, item.object, EffectKind::ReplacementRule { card: item.ability.card, index: 0 }, Duration::EndOfTurn); } }
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
 Ok(()) }
planned_item!("replacement-of-lose-game" => callbacks: #[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::LoseGame(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if Some(asker.controller) == Some(p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_1_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::LoseGame(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::LoseGame(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn made_1_then(g: &mut dyn Game, asker: &Asker, affected: ObjectId, before: &Proposal, _: &Proposal) { if let Proposal::LoseGame(p) = before { let q: &dyn Query = g; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; let _ = made_1_then_steps(g, asker, affected, subject, data); } }
);

pub static CARD: CardDef = CardDef { name: "Stunning Reversal", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[planned_value!("replacement of LoseGame", granted_replacement(ReplacementKind::Damage, |_, _, _, _| false, |_, _, _, _| (), "").text("The next time you would lose the game this turn, instead draw seven cards and your life total becomes 1.") => granted_replacement(ReplacementKind::LoseGame, made_1_applies, made_1_apply, "The next time you would lose the game this turn, instead draw seven cards and your life total becomes 1.").once().then(made_1_then).text("The next time you would lose the game this turn, instead draw seven cards and your life total becomes 1."))], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
