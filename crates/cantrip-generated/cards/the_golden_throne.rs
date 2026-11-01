// The Golden Throne
// status: planned
// requires: replacement of LoseGame
// Arcane Life-support — If you would lose the game, instead exile The Golden Throne and your life total becomes 1.
// A Thousand Souls Die Every Day — {T}, Sacrifice a creature: Add three mana in any combination of colors.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for player in players_1 { g.set_life_total(player, amount_1); }
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2 = { let q: &dyn Query = g; (3i64).max(0) as u8 };
for player in mana_players_2 {
if mana_2 > 0 { let colors = cantrip_mechanics::choose_colors(g, player, asker.source, ColorSet(31), mana_2)?;
for color in colors { g.add_mana(player, color.mana(), 1); }
}
}
 Ok(()) }
#[allow(unused_variables)] fn ability_1_3(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) }
planned_item!("replacement-of-lose-game" => callbacks: #[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::LoseGame(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if Some(asker.controller) == Some(p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::LoseGame(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::LoseGame(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn ability_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::LoseGame(p) = before { let q: &dyn Query = g; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; let _ = ability_0_then_steps(g, asker, subject, data); } }
);
planned_item!("replacement-of-lose-game" => replacements: replacement(ReplacementKind::LoseGame, ability_0_applies, ability_0_apply, "Arcane Life-support — If you would lose the game, instead exile The Golden Throne and your life total becomes 1.").then(ability_0_then));

pub static CARD: CardDef = CardDef { name: "The Golden Throne", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(sacrifice(1, ability_1_3)), add_by(ability_1_mana).colors(ColorSet(31)).yields_any(3), "A Thousand Souls Die Every Day — {T}, Sacrifice a creature: Add three mana in any combination of colors.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
