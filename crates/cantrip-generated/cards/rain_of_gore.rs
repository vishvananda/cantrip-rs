// Rain of Gore
// status: planned
// requires: replacement of GainLifeByEffect
// If a spell or ability would cause its controller to gain life, that player loses that much life instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let amount_0 = { let q: &dyn Query = g; (i64::from(data[0])).clamp(0, 2147483647) as i32 }; let players_0 = { let q: &dyn Query = g; subject.player().into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
 Ok(()) }
planned_item!("replacement-of-gain-life-by-effect" => callbacks: #[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::EffectLifeGain(p) = proposal { let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if true { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::EffectLifeGain(seen) = &seen { let p = seen; let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if let Proposal::EffectLifeGain(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn ability_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::EffectLifeGain(p) = before { let q: &dyn Query = g; let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; let _ = ability_0_then_steps(g, asker, subject, data); } }
);
planned_item!("replacement-of-gain-life-by-effect" => replacements: replacement(ReplacementKind::EffectLifeGain, ability_0_applies, ability_0_apply, "If a spell or ability would cause its controller to gain life, that player loses that much life instead.").then(ability_0_then));

pub static CARD: CardDef = CardDef { name: "Rain of Gore", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
