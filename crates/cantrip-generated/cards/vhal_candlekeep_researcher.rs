// Vhal, Candlekeep Researcher
// status: ready
// Vigilance
// {T}: Add an amount of {C} equal to Vhal's toughness. This mana can't be spent to cast spells from your hand.
// Choose a Background (You can have a Background as a second commander.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).toughness.unwrap_or(0)))).max(0) as u32 };
for player in mana_players_0 {
g.add_restricted_mana(player, Mana::C, mana_0_0, ManaRestriction::NOT_SPELLS_FROM_HAND);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Vhal, Candlekeep Researcher", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_1_mana).only_for(ManaRestriction::NOT_SPELLS_FROM_HAND), "{T}: Add an amount of {C} equal to Vhal's toughness. This mana can't be spent to cast spells from your hand.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
