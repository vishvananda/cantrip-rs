// Adipose Offspring
// status: ready
// Emerge {5}{W} (You may cast this spell by sacrificing a creature and paying the emerge cost reduced by that creature's mana value.)
// When this creature enters, create a 2/2 white Alien creature token. If this creature's emerge cost was paid, instead create X of those tokens, where X is the sacrificed creature's toughness.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Alien).name, token_words: token!(Alien).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Alien]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_4 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Alien).name, token_words: token!(Alien).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Alien]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| { let o = q.state().obj(candidate); o.cast_way > 0 && q.card(o.card).ways.get(o.cast_way as usize - 1).is_some_and(|w| w.cost.emerge) }) } {
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (({ let records = q.history().all(); let spell = q.history().original_spell(records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == item.object => Some(old), _ => None }).unwrap_or(item.object)); records.iter().rposition(|r| matches!(r.event, Event::Cast(c) if c.spell == spell)).map_or_else(Vec::new, |cast| records[..cast].iter().rev().filter_map(|r| match r.event { Event::ZoneChange { old, from, reason: ZoneReason::Sacrifice, .. } if from.is(ZoneKind::Battlefield) => Some(old), _ => None }).take(1).collect::<Vec<ObjectId>>()) }).into_iter().next().map_or(0i64, |id: ObjectId| i64::from(q.chars(id).toughness.unwrap_or(0)))) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
hoisted_2.extend(created_0.clone());
} else {
let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_3 = Vec::new();
let count_3_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_3_0 = g.card_id(&token_4::CARD);
for player in players_3 {
tokens_3.extend(std::iter::repeat((def_3_0,player)).take(count_3_0 as usize));
} let words_3 = g.words(asker.source);
let created_3 = g.create_tokens_worded(&tokens_3, false, None, words_3)?;
g.end_sentence();
hoisted_5.extend(created_3.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Adipose Offspring", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Alien]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, create a 2/2 white Alien creature token. If this creature's emerge cost was paid, instead create X of those tokens, where X is the sacrificed creature's toughness.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[way(Cost { mana: ManaCost { generic: 5, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, permanents: Some(PermanentCost { count: 1, filter: cantrip_mechanics::targets::filters::creature, action: PermanentAction::Sacrifice, tapped: false }), emerge: true, ..Cost::NONE }, ZoneKind::Hand, "Emerge")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD,&token_4::CARD];
