// Oketra the True
// status: ready
// Double strike, indestructible
// Oketra can't attack or block unless you control at least three other creatures.
// {3}{W}: Create a 1/1 white Warrior creature token with vigilance.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(((q.state().battlefield().iter().copied().filter(|&id| ((!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (3i64))) && (source == affected) }
#[allow(unused_variables)] fn ability_2_0_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { let source = asker.source; let asker = &Asker::new(source, q.controller(source)); !((!(((q.state().battlefield().iter().copied().filter(|&id| ((!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (3i64))) && attacker == asker.source) }
#[allow(unused_variables)] fn ability_2_0_1_keyword(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if !(((q.state().battlefield().iter().copied().filter(|&id| ((!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (3i64)) { chars.keywords = chars.keywords.with(Keyword::CantBlock); } }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Warrior).name, token_words: token!(Warrior).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Oketra the True", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::God]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::DoubleStrike,Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_3, "{3}{W}: Create a 1/1 white Warrior creature token with vigilance.").targets(&[])], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_2_0_1_keyword, "Oketra can't attack or block unless you control at least three other creatures.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[ability_2_0_0_rule], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
