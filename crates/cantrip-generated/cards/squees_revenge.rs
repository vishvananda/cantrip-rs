// Squee's Revenge
// status: ready
// Choose a number. Flip a coin that many times or until you lose a flip, whichever comes first. If you win all the flips, draw two cards for each flip.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let least_0 = 0u32; let most_0 = i32::MAX as u32; let chosen_0: u32 = cantrip_mechanics::choose::number(g, player_0, asker.source, least_0, most_0, "Choose a number")?;
g.end_sentence();
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times_1: Option<u32> = Some({ let q: &dyn Query = g; (i64::from(chosen_0)) as u32 }); let mut done_1: i64 = 0; let mut flips_1: u32 = 0; while times_1.map_or(true, |n| flips_1 < n) { flips_1 += 1; if g.flip_coin(player_1) { done_1 += 1; } else if true { break; } }
g.end_sentence();
if { let q: &dyn Query = g; ((i64::from(flips_1) - done_1)) == (0i64) } {
let amount_2 = { let q: &dyn Query = g; ((2i64 * done_1)).min(u32::MAX.into()) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Squee's Revenge", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
