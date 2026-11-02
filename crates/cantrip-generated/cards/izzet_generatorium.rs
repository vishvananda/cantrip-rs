// Izzet Generatorium
// status: planned
// requires: condition PlayerDid PaidOrLostEnergy
// If you would get one or more {E} (energy counters), you get that many plus one {E} instead.
// {T}: Draw a card. Activate only if you've paid or lost four or more {E} this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::PlayerCounters(c) = proposal { let subject = Target::Player(c.player); let data = [u32::from(c.energy), u32::MAX]; if c.energy > 0 && Some(asker.controller) == Some(c.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::PlayerCounters(seen) = &seen { let c = seen; let subject = Target::Player(c.player); let data = [u32::from(c.energy), u32::MAX]; if let Proposal::PlayerCounters(c) = proposal { c.energy = 0; } } }
#[allow(unused_variables, unused_mut)] fn ability_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let amount_0 = { let q: &dyn Query = g; ((i64::from(data[0]) + 1i64)).min(i32::MAX.into()) as i32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.change_energy(player_0, amount_0);
 Ok(()) }
#[allow(unused_variables)] fn ability_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::PlayerCounters(c) = before { let q: &dyn Query = g; let subject = Target::Player(c.player); let data = [u32::from(c.energy), u32::MAX]; let _ = ability_0_then_steps(g, asker, subject, data); } }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { planned_value!("condition PlayerDid PaidOrLostEnergy", false => q.player_did(q.history().this_turn().iter(), Some(asker.controller), PlayerDeed::PaidOrLostEnergy(Comparison::GreaterOrEqual, 4i64))) }

pub static CARD: CardDef = CardDef { name: "Izzet Generatorium", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{T}: Draw a card. Activate only if you've paid or lost four or more {E} this turn.").targets(&[]).condition(ability_1_if)], statics: &[], replacements: &[replacement(ReplacementKind::Counters, ability_0_applies, ability_0_apply, "If you would get one or more {E} , you get that many plus one {E} instead.").then(ability_0_then)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
