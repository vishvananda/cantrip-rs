// False Dawn
// status: planned
// requires: player rule for a while SpendColorAs
// Until end of turn, spells and abilities you control that would add colored mana instead add that much white mana. Until end of turn, you may spend white mana as though it were mana of any color.
// Draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::ManaProduction(m) = proposal { let id = m.permanent; let amount = m.total(); let subject = Target::Object(id); let data = [amount, u32::MAX]; if Some(asker.controller) == Some(q.controller(id)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_1_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ManaProduction(seen) = &seen { let m = seen; let id = m.permanent; let amount = m.total(); let subject = Target::Object(id); let data = [amount, u32::MAX]; if let Proposal::ManaProduction(m) = proposal { let colored: u32 = m.mana.0[..5].iter().sum(); m.mana.0[..5].fill(0); m.mana.0[0] += colored; } } }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { let _ = (to, about); g.add_player_effect(item.controller, item.object, EffectKind::ReplacementRule { card: item.ability.card, index: 0 }, Duration::EndOfTurn); } }
g.end_sentence();
planned!("player-rule-for-a-while-spend-color-as" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::SpendColorAs { color: Mana::White, as_color: None, others_colorless: false, on: Payment::Anything }), Duration::EndOfTurn); } });
g.end_sentence();
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "False Dawn", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::ManaProduction, made_1_applies, made_1_apply, "Until end of turn, spells and abilities you control that would add colored mana instead add that much white mana. Until end of turn, you may spend white mana as though it were mana of any color.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
