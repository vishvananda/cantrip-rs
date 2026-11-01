// Wall of Roots
// status: planned
// requires: power/toughness counter other than ±1/±1
// Defender
// Put a -0/-1 counter on this creature: Add {G}. Activate only once each turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
g.add_mana(player, Mana::G, mana_0_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Wall of Roots", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Plant,Subtype::Wall]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(put_counters(planned_value!("power/toughness counter other than ±1/±1", CounterKind::PlusOne => CounterKind::Plus0Minus1), 1)), Adds { pool: ManaPool([0, 0, 0, 0, 1, 0]), ..add_by(ability_1_mana) }, "Put a -0/-1 counter on this creature: Add {G}. Activate only once each turn.").once_each_turn()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
