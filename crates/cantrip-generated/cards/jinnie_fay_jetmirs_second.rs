// Jinnie Fay, Jetmir's Second
// status: planned
// requires: event change in the program of another event
// requires: instruction ChooseOne
// requires: replacement of token creation with other tokens
// If you would create one or more tokens, you may instead create that many 2/2 green Cat creature tokens with haste or that many 3/1 green Dog creature tokens with vigilance.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::TokenCreation(t) = proposal { let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if t.count > 0 && true && Some(asker.controller) == Some(t.controller) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::TokenCreation(seen) = &seen { let t = seen; let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; if let Proposal::TokenCreation(t) = proposal {  } } }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Cat).name, token_words: token!(Cat).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_5 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Dog).name, token_words: token!(Dog).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dog]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(3), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
#[allow(unused_variables, unused_mut)] fn ability_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_1, asker.source, vec![Label::Card(card_1, 0), Label::Card(card_1, 1), Label::Card(card_1, 2)])?) {
0 => {
planned!("event-change-in-the-program-of-another-event" => g.change_event(|q, proposal| { if let Proposal::TokenCreation(t) = proposal { let each = t.count; t.count = t.count.saturating_mul(0); planned_value!("replacement of token creation with other tokens", () => t.also(q.card_id(&token_3::CARD), (i64::from(data[0])).max(0) as u32, q.words(asker.source))); } }););
}
1 => {
planned!("event-change-in-the-program-of-another-event" => g.change_event(|q, proposal| { if let Proposal::TokenCreation(t) = proposal { let each = t.count; t.count = t.count.saturating_mul(0); planned_value!("replacement of token creation with other tokens", () => t.also(q.card_id(&token_5::CARD), (i64::from(data[0])).max(0) as u32, q.words(asker.source))); } }););
}
_ => {
planned!("event-change-in-the-program-of-another-event" => g.change_event(|q, proposal| { if let Proposal::TokenCreation(t) = proposal { t.count = t.count.saturating_mul(1); } }););
}
}
 Ok(()) }
#[allow(unused_variables)] fn ability_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::TokenCreation(t) = before { let q: &dyn Query = g; let subject = Target::Player(t.controller); let data = [t.count, u32::MAX]; let _ = ability_0_then_steps(g, asker, subject, data); } }

pub static CARD: CardDef = CardDef { name: "Jinnie Fay, Jetmir's Second", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [Some((Mana::R, Mana::G)),Some((Mana::G, Mana::W)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elf,Subtype::Druid]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::TokenCreation, ability_0_applies, ability_0_apply, "If you would create one or more tokens, you may instead create that many 2/2 green Cat creature tokens with haste or that many 3/1 green Dog creature tokens with vigilance.").then(ability_0_then)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD,&token_5::CARD];
