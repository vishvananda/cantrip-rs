// Plague Drone
// status: ready
// Flying
// Rot Fly — If an opponent would gain life, that player loses that much life instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::LifeChange(l) = proposal { let amount = l.amount.unsigned_abs(); let subject = Target::Player(l.player); let data = [amount, u32::MAX]; if l.amount > 0 && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&l.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::LifeChange(seen) = &seen { let l = seen; let amount = l.amount.unsigned_abs(); let subject = Target::Player(l.player); let data = [amount, u32::MAX]; if let Proposal::LifeChange(l) = proposal { l.amount = 0; } } }
#[allow(unused_variables, unused_mut)] fn ability_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let amount_0 = { let q: &dyn Query = g; (i64::from(data[0])).min(i32::MAX.into()) as i32 }; let players_0 = { let q: &dyn Query = g; subject.player().into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
 Ok(()) }
#[allow(unused_variables)] fn ability_1_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::LifeChange(l) = before { let q: &dyn Query = g; let amount = l.amount.unsigned_abs(); let subject = Target::Player(l.player); let data = [amount, u32::MAX]; let _ = ability_1_then_steps(g, asker, subject, data); } }

pub static CARD: CardDef = CardDef { name: "Plague Drone", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::LifeChange, ability_1_applies, ability_1_apply, "Rot Fly — If an opponent would gain life, that player loses that much life instead.").then(ability_1_then)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
