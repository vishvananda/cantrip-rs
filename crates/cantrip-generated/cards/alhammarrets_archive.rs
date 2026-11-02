// Alhammarret's Archive
// status: ready
// If you would gain life, you gain twice that much life instead.
// If you would draw a card except the first one you draw in each of your draw steps, draw two cards instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::LifeChange(l) = proposal { let amount = l.amount.unsigned_abs(); let subject = Target::Player(l.player); let data = [amount, u32::MAX]; if l.amount > 0 && Some(asker.controller) == Some(l.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::LifeChange(seen) = &seen { let l = seen; let amount = l.amount.unsigned_abs(); let subject = Target::Player(l.player); let data = [amount, u32::MAX]; if let Proposal::LifeChange(l) = proposal { l.amount = l.amount.signum() * ((2i64 * i64::from(data[0]))).min(i32::MAX.into()) as i32; } } }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Draw(d) = proposal { let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; if Some(asker.controller) == Some(d.player) && !cantrip_mechanics::history::first_draw_in_own_draw_step(q, d.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Draw(seen) = &seen { let d = seen; let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; if let Proposal::Draw(d) = proposal { d.replaced = true; } } }
#[allow(unused_variables, unused_mut)] fn ability_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
 Ok(()) }
#[allow(unused_variables)] fn ability_1_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::Draw(d) = before { let q: &dyn Query = g; let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; let _ = ability_1_then_steps(g, asker, subject, data); } }

pub static CARD: CardDef = CardDef { name: "Alhammarret's Archive", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::LifeChange, ability_0_applies, ability_0_apply, "If you would gain life, you gain twice that much life instead."),replacement(ReplacementKind::Draw, ability_1_applies, ability_1_apply, "If you would draw a card except the first one you draw in each of your draw steps, draw two cards instead.").then(ability_1_then)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
