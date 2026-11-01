// Rhystic Cave
// status: ready
// {T}: Choose a color. Add one mana of that color unless any player pays {1}. Activate only as an instant.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Some(chosen_0) = chosen else { return Ok(()) };
let mut paid_1 = false; for bound_1000002 in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } {
let payer_3 = { let q: &dyn Query = g; Some(bound_1000002).expect("a reference that always names a player") }; let paid_3 = g.may_pay(payer_3, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
if paid_3 { paid_1 = true; break; } }
if { let q: &dyn Query = g; !(((paid_1 as i64) != 0)) } {
let mana_players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_5_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_5 {
if let Some(color) = { let q: &dyn Query = g; Some(chosen_0) } { g.add_mana(player, color.mana(), mana_5_0); }
}
g.end_sentence();
}
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Rhystic Cave", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_any_color(1).by(ability_0_mana), "{T}: Choose a color. Add one mana of that color unless any player pays {1}. Activate only as an instant.").only_as_an_instant()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
