// Hazezon, Shaper of Sand
// status: planned
// requires: landwalk of Subtype
// Desertwalk (This creature can't be blocked as long as defending player controls a Desert.)
// You may play Desert lands from your graveyard.
// Whenever a Desert you control enters, create two 1/1 red, green, and white Sand Warrior creature tokens.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_cost(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> Option<Cost> { (player == asker.controller && (q.chars(card).has_subtype(q.words(asker.source).subtype(Subtype::Desert)) && q.chars(card).is(CardType::Land)) && q.chars(card).is(CardType::Land)).then(|| Cost::mana(q.card(q.chars(card).card).cost)) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Desert)) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Sand Warrior).name, token_words: token!(Sand Warrior).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Sand,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(25)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Hazezon, Shaper of Sand", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[planned_value!("landwalk of Subtype", Keyword::Swampwalk => Keyword::Landwalk(LandFilter::Subtype(Subtype::Desert)))]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "Whenever a Desert you control enters, create two 1/1 red, green, and white Sand Warrior creature tokens.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_ways: &[GrantedWay { text: "You may play Desert lands from your graveyard.", permission: true, lands: true, cost: ability_1_0_cost, ..GrantedWay::CAST }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
