// Blood Scrivener
// status: ready
// If you would draw a card while you have no cards in hand, instead you draw two cards and you lose 1 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Draw(d) = proposal { let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; if Some(asker.controller) == Some(d.player) && ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) == (0i64) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Draw(seen) = &seen { let d = seen; let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; if let Proposal::Draw(d) = proposal { d.replaced = true; } } }
#[allow(unused_variables, unused_mut)] fn ability_0_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
 Ok(()) }
#[allow(unused_variables)] fn ability_0_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::Draw(d) = before { let q: &dyn Query = g; let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; let _ = ability_0_0_then_steps(g, asker, subject, data); } }

pub static CARD: CardDef = CardDef { name: "Blood Scrivener", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::Draw, ability_0_0_applies, ability_0_0_apply, "If you would draw a card while you have no cards in hand, instead you draw two cards and you lose 1 life.").then(ability_0_0_then)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
