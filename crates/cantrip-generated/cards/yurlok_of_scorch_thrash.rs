// Yurlok of Scorch Thrash
// status: planned
// requires: numeric expression Game
// requires: player reference Prior
// requires: player rule AsManaEmpties
// Vigilance
// A player losing unspent mana causes that player to lose that much life.
// {1}, {T}: Each player adds {B}{R}{G}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
fn ability_2_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let mana_players_4 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
let mana_4_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mana_4_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mana_4_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_4 {
g.add_mana(player, Mana::B, mana_4_0);g.add_mana(player, Mana::R, mana_4_1);g.add_mana(player, Mana::G, mana_4_2);}
    } 
 Ok(()) }
planned_item!("player-rule-as-mana-empties" => statics: player_rule(ability_1_0_applies, PlayerRule::AsManaEmpties { mana: UnspentMana::AnyUnspentMana, then: &[|g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { let mut players_0: Vec<PlayerId> = planned_value!("player reference Prior", Vec::<PlayerId>::new() => "(kind:One,args:(kind:Prior,args:That))");
 for (index_0, bound_9000) in players_0.iter().copied().enumerate() { let amount_1 = { let q: &dyn Query = g; (planned_value!("numeric expression Game", 0i64 => q.game_value(item, GameValue::ManaLostThisWay))).clamp(0, 2147483647) as i32 }; let players_1 = { let q: &dyn Query = g; Some(bound_9000).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
    } 
g.end_sentence();
 Ok(()) }] }));

pub static CARD: CardDef = CardDef { name: "Yurlok of Scorch Thrash", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Lizard,Subtype::Shaman]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_2_mana), "{1}, {T}: Each player adds {B}{R}{G}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
