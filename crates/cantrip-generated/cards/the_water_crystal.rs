// The Water Crystal
// status: planned
// requires: replacement of Mill
// Blue spells you cast cost {1} less to cast.
// If an opponent would mill one or more cards, they mill that many cards plus four instead.
// {4}{U}{U}, {T}: Each opponent mills cards equal to the number of cards in your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && q.chars(spell).colors.has(q.words(asker.source).color(Color::Blue)) { CostDelta::less((1i64) as u8) } else { CostDelta::NONE } }
#[allow(unused_variables, unused_mut)] fn ability_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; subject.player().into_iter().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<(PlayerId, u32)> = Vec::new();
 for (index_0, bound_9000) in players_0.iter().copied().enumerate() { gathered_1.push((bound_9000, { let q: &dyn Query = g; ((i64::from(data[0]) + 4i64)).clamp(0, 4294967295) as u32 }));
    } 
let milled_1 = g.mill_together(&gathered_1);
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_3 = { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).max(0) as u32 }; let mills_3: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_3)).collect(); let milled_3 = g.mill_together(&mills_3);
g.end_sentence();
 Ok(()) }
planned_item!("replacement-of-mill" => callbacks: #[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Mill(p) = proposal { let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Mill(seen) = &seen { let p = seen; let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if let Proposal::Mill(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn ability_1_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::Mill(p) = before { let q: &dyn Query = g; let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; let _ = ability_1_then_steps(g, asker, subject, data); } }
);
planned_item!("replacement-of-mill" => replacements: replacement(ReplacementKind::Mill, ability_1_applies, ability_1_apply, "If an opponent would mill one or more cards, they mill that many cards plus four instead.").then(ability_1_then));

pub static CARD: CardDef = CardDef { name: "The Water Crystal", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_2, "{4}{U}{U}, {T}: Each opponent mills cards equal to the number of cards in your hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
