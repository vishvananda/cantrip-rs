// Corpse Cobble
// status: planned
// requires: engine: no record of what paid a cost
// As an additional cost to cast this spell, sacrifice any number of creatures.
// Create an X/X blue and black Zombie creature token with menace, where X is the total power of the sacrificed creatures.
// Flashback {3}{U}{B} (You may cast this card from your graveyard for its flashback cost and any additional costs. Then exile it.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Zombie).name, token_words: token!(Zombie).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(6)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let power_1 = { let q: &dyn Query = g; ((planned_value!("engine: no record of what paid a cost", Vec::<ObjectId>::new() => q.paid_objects(item, [0]).to_vec())).into_iter().map(|o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))).sum::<i64>()).clamp(0, 32767) as i16 }; let toughness_1 = { let q: &dyn Query = g; ((planned_value!("engine: no record of what paid a cost", Vec::<ObjectId>::new() => q.paid_objects(item, [0]).to_vec())).into_iter().map(|o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))).sum::<i64>()).clamp(0, 32767) as i16 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut values_1 = Copiable::of(g.card_id(&token_2::CARD)).sized(power_1, toughness_1); values_1.words = g.words(asker.source); let mut created_1: Vec<ObjectId> = Vec::new(); for player in players_1 { created_1.extend(g.create_token_copies_tapped(values_1, player, count_1, false)?); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_require(q: &dyn Query, asker: &Asker) -> bool { ((q.chars(asker.source).is(CardType::Instant) || q.chars(asker.source).is(CardType::Sorcery))) }

pub static CARD: CardDef = CardDef { name: "Corpse Cobble", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ZoneKind::Graveyard, "Flashback").only_from().exile_after().condition(ability_2_require)], additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { variable: Some(VariableCost::SacrificeAnyNumber { filter: ability_0_0, less_each: 0 }), ..Cost::NONE })), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
