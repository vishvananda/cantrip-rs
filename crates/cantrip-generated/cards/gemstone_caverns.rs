// Gemstone Caverns
// status: ready
// If this card is in your opening hand and you're not the starting player, you may begin the game with Gemstone Caverns on the battlefield with a luck counter on it. If you do, exile a card from your hand.
// {T}: Add {C}. If Gemstone Caverns has a luck counter on it, instead add one mana of any color.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_may_begin(q: &dyn Query, asker: &Asker) -> bool { !(Some(asker.controller).is_some_and(|player: PlayerId| q.state().turn.starting == player)) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Luck))) > (0i64)) } {
let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_1 {
if mana_1 > 0 { let colors = cantrip_mechanics::choose_colors(g, player, asker.source, ColorSet(31), 1)?;
g.add_mana(player, colors[0].mana(), mana_1);}
}
g.end_sentence();
} else {
let mana_players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_3_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_3 {
g.add_mana(player, Mana::C, mana_3_0);}
g.end_sentence();
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Gemstone Caverns", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_1_mana), "{T}: Add {C}. If Gemstone Caverns has a luck counter on it, instead add one mana of any color.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, opening_hand: Some(OpeningHandAction { condition: ability_0_may_begin, counter: Some((CounterKind::Luck, 1)), exile_from_hand: 1 }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
