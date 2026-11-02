// Wurmcalling
// status: planned
// requires: engine: X in a mana cost beside a kicker
// Buyback {2}{G} (You may pay an additional {2}{G} as you cast this spell. If you do, put this card into your hand as it resolves.)
// Create an X/X green Wurm creature token.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Wurm).name, token_words: token!(Wurm).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Wurm]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let power_0 = { let q: &dyn Query = g; (i64::from(item.x)).min(i16::MAX.into()) as i16 }; let toughness_0 = { let q: &dyn Query = g; (i64::from(item.x)).min(i16::MAX.into()) as i16 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut values_0 = Copiable::of(g.card_id(&token_1::CARD)).sized(power_0, toughness_0); values_0.words = g.words(asker.source); let mut created_0: Vec<ObjectId> = Vec::new(); for player in players_0 { created_0.extend(g.create_token_copies_tapped(values_0, player, count_0, false)?); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-x-in-a-mana-cost-beside-a-kicker", "kicker(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })");

pub static CARD: CardDef = CardDef { name: "Wurmcalling", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[cantrip_mechanics::replacements::BUYBACK], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, kicker: kicker(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
