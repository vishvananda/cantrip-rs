// Loot, the Nexus
// status: ready
// {T}: Choose a color. Add one mana of that color for each different power among creatures you control.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Some(chosen_0) = chosen else { return Ok(()) };
let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; ({ let mut seen = Vec::new(); for id in q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() { let value = q.chars(id).power; if !seen.contains(&value) { seen.push(value); } } seen.len() as i64 }) as u32 };
for player in mana_players_2 {
if let Some(color) = { let q: &dyn Query = g; Some(chosen_0) } { g.add_mana(player, color.mana(), mana_2_0); }
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Loot, the Nexus", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast,Subtype::Noble]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_any_color(1).by(ability_0_mana), "{T}: Choose a color. Add one mana of that color for each different power among creatures you control.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
