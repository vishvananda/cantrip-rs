// Legion's Landing // Adanto, the First Fort
// status: ready
// (Transforms from Legion's Landing.)
// {T}: Add {W}.
// {2}{W}, {T}: Create a 1/1 white Vampire creature token with lifelink.
// When Legion's Landing enters, create a 1/1 white Vampire creature token with lifelink.
// When you attack with three or more creatures, transform Legion's Landing.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Vampire).name, token_words: token!(Vampire).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && true && (q.state().attackers.iter().filter(|a| { let (id, defender) = (a.id, &a.target); q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && true }).count() as i64) >= (3i64) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Legion's Landing", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Legion's Landing enters, create a 1/1 white Vampire creature token with lifelink.").targets(&[]),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "When you attack with three or more creatures, transform Legion's Landing.").targets(&[]).once_per_batch()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD,&back::token_2::CARD];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
g.add_mana(player, Mana::W, mana_0_0);}
 Ok(()) }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Vampire).name, token_words: token!(Vampire).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Adanto, the First Fort", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([1, 0, 0, 0, 0, 0]), ..add_by(ability_0_mana) }, "{T}: Add {W}."),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{2}{W}, {T}: Create a 1/1 white Vampire creature token with lifelink.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
