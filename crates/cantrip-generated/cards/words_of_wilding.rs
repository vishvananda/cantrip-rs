// Words of Wilding
// status: ready
// {1}: The next time you would draw a card this turn, create a 2/2 green Bear creature token instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::Draw(d) = proposal { let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; if Some(asker.controller) == Some(d.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_1_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Draw(seen) = &seen { let d = seen; let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; if let Proposal::Draw(d) = proposal { d.replaced = true; } } }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Bear).name, token_words: token!(Bear).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bear]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
#[allow(unused_variables, unused_mut)] fn made_1_then_steps(g: &mut dyn Game, asker: &Asker, affected: ObjectId, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_2 = Vec::new();
let count_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_2_0 = g.card_id(&token_3::CARD);
for player in players_2 {
tokens_2.extend(std::iter::repeat((def_2_0,player)).take(count_2_0 as usize));
} let words_2 = g.words(asker.source);
let created_2 = g.create_tokens_worded(&tokens_2, false, None, words_2)?;
 Ok(()) }
#[allow(unused_variables)] fn made_1_then(g: &mut dyn Game, asker: &Asker, affected: ObjectId, before: &Proposal, _: &Proposal) { if let Proposal::Draw(d) = before { let q: &dyn Query = g; let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; let _ = made_1_then_steps(g, asker, affected, subject, data); } }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { let _ = (to, about); g.add_player_effect(item.controller, item.object, EffectKind::ReplacementRule { card: item.ability.card, index: 0 }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Words of Wilding", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{1}: The next time you would draw a card this turn, create a 2/2 green Bear creature token instead.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::Draw, made_1_applies, made_1_apply, "{1}: The next time you would draw a card this turn, create a 2/2 green Bear creature token instead.").once().then(made_1_then).text("The next time you would draw a card this turn, create a 2/2 green Bear creature token instead.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD];
