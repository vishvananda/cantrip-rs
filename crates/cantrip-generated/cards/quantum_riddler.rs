// Quantum Riddler
// status: ready
// Flying
// When this creature enters, draw a card.
// As long as you have one or fewer cards in hand, if you would draw one or more cards, you draw that many cards plus one instead.
// Warp {1}{U}
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::DrawInstruction(d) = proposal { let amount = d.count; let subject = Target::Player(d.player); let data = [d.count, u32::MAX]; if Some(asker.controller) == Some(d.player) && ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) <= (1i64) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_2_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::DrawInstruction(seen) = &seen { let d = seen; let amount = d.count; let subject = Target::Player(d.player); let data = [d.count, u32::MAX]; if let Proposal::DrawInstruction(d) = proposal { d.count = 0; } } }
#[allow(unused_variables, unused_mut)] fn ability_2_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let amount_1 = { let q: &dyn Query = g; ((i64::from(data[0]) + 1i64)).min(u32::MAX.into()) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::DrawInstruction(d) = before { let q: &dyn Query = g; let amount = d.count; let subject = Target::Player(d.player); let data = [d.count, u32::MAX]; let _ = ability_2_0_then_steps(g, asker, subject, data); } }

pub static CARD: CardDef = CardDef { name: "Quantum Riddler", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Sphinx]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, draw a card.").targets(&[])], statics: &[], replacements: &[replacement(ReplacementKind::DrawInstruction, ability_2_0_applies, ability_2_0_apply, "As long as you have one or fewer cards in hand, if you would draw one or more cards, you draw that many cards plus one instead.").then(ability_2_0_then)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[cantrip_mechanics::warp::EXILE], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::warp::way(ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Warp {1}{U}")], end_step_riders: cantrip_mechanics::warp::RIDERS, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
