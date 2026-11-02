// Welcome the Darkness
// status: ready
// X can't be 0.
// Draw X cards. Create an X/X black Demon creature token with flying. Your life total becomes X.
// You can't gain life for the rest of the game.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Demon).name, token_words: token!(Demon).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
let count_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let power_1 = { let q: &dyn Query = g; (i64::from(item.x)).clamp(0, 32767) as i16 }; let toughness_1 = { let q: &dyn Query = g; (i64::from(item.x)).clamp(0, 32767) as i16 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut values_1 = Copiable::of(g.card_id(&token_2::CARD)).sized(power_1, toughness_1); values_1.words = g.words(asker.source); let mut created_1: Vec<ObjectId> = Vec::new(); for player in players_1 { created_1.extend(g.create_token_copies_tapped(values_1, player, count_1, false)?); }
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as i32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for player in players_3 { g.set_life_total(player, amount_3); }
g.end_sentence();
let players_4_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_4_0 { g.add_player_effect(asker.controller, source, EffectKind::CantGainLife(player), Duration::Indefinite); } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Welcome the Darkness", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, x_cant_be_zero: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
