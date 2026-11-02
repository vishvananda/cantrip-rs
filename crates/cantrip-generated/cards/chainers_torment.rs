// Chainer's Torment
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I, II — This Saga deals 2 damage to each opponent and you gain 2 life.
// III — Create an X/X black Nightmare Horror creature token, where X is half your life total, rounded up. It deals X damage to you.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let amount_2 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_2 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_2 { let batch: Vec<(ObjectId, Target, u16)> = recipients_2.iter().map(|t| (source, *t, amount_2)).collect(); g.damage_together(&batch); }
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.gain_life(player, amount_3); }
g.end_sentence();
 Ok(()) }
pub mod token_6 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Nightmare Horror).name, token_words: token!(Nightmare Horror).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Nightmare,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let value_4: i64 = { let q: &dyn Query = g; (Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))).saturating_add(1).div_euclid(2) };
g.end_sentence();
let count_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let power_5 = { let q: &dyn Query = g; (value_4).clamp(0, 32767) as i16 }; let toughness_5 = { let q: &dyn Query = g; (value_4).clamp(0, 32767) as i16 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut values_5 = Copiable::of(g.card_id(&token_6::CARD)).sized(power_5, toughness_5); values_5.words = g.words(asker.source); let mut created_5: Vec<ObjectId> = Vec::new(); for player in players_5 { created_5.extend(g.create_token_copies_tapped(values_5, player, count_5, false)?); }
g.end_sentence();
let source_7 = { let q: &dyn Query = g; (created_5.clone()).get(0 as usize).copied() }; let amount_7 = { let q: &dyn Query = g; (value_4).clamp(0, 65535) as u16 }; let recipients_7 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_7 { let batch: Vec<(ObjectId, Target, u16)> = recipients_7.iter().map(|t| (source, *t, amount_7)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Chainer's Torment", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II — This Saga deals 2 damage to each opponent and you gain 2 life.").targets(&[]),chapter(2, ability_0_1, "I, II — This Saga deals 2 damage to each opponent and you gain 2 life.").targets(&[]),chapter(3, ability_0_2, "III — Create an X/X black Nightmare Horror creature token, where X is half your life total, rounded up. It deals X damage to you.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_6::CARD];
