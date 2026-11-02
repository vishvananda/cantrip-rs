// Bruvac the Grandiloquent
// status: planned
// requires: replacement of Mill
// If an opponent would mill one or more cards, they mill twice that many cards instead. (To mill a card, a player puts the top card of their library into their graveyard.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; subject.player().into_iter().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<(PlayerId, u32)> = Vec::new();
 for (index_0, bound_9000) in players_0.iter().copied().enumerate() { gathered_1.push((bound_9000, { let q: &dyn Query = g; ((2i64 * i64::from(data[0]))).min(u32::MAX.into()) as u32 }));
    } 
let milled_1 = g.mill_together(&gathered_1);
 Ok(()) }
planned_item!("replacement-of-mill" => callbacks: #[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Mill(p) = proposal { let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Mill(seen) = &seen { let p = seen; let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if let Proposal::Mill(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn ability_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::Mill(p) = before { let q: &dyn Query = g; let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; let _ = ability_0_then_steps(g, asker, subject, data); } }
);
planned_item!("replacement-of-mill" => replacements: replacement(ReplacementKind::Mill, ability_0_applies, ability_0_apply, "If an opponent would mill one or more cards, they mill twice that many cards instead.").then(ability_0_then));

pub static CARD: CardDef = CardDef { name: "Bruvac the Grandiloquent", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Advisor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
