// Exquisite Archangel
// status: planned
// requires: replacement of LoseGame
// Flying
// If you would lose the game, instead exile this creature and your life total becomes equal to your starting life total.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
let amount_1 = { let q: &dyn Query = g; (Some(asker.controller).map_or(0, |player: PlayerId| i64::from(q.state().player(player).starting_life))).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for player in players_1 { g.set_life_total(player, amount_1); }
 Ok(()) }
planned_item!("replacement-of-lose-game" => callbacks: #[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::LoseGame(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if Some(asker.controller) == Some(p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::LoseGame(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::LoseGame(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn ability_1_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::LoseGame(p) = before { let q: &dyn Query = g; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; let _ = ability_1_then_steps(g, asker, subject, data); } }
);
planned_item!("replacement-of-lose-game" => replacements: replacement(ReplacementKind::LoseGame, ability_1_applies, ability_1_apply, "If you would lose the game, instead exile this creature and your life total becomes equal to your starting life total.").then(ability_1_then));

pub static CARD: CardDef = CardDef { name: "Exquisite Archangel", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
