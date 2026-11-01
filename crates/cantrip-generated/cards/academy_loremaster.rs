// Academy Loremaster
// status: ready
// At the beginning of each player's draw step, that player may draw an additional card. If they do, spells they cast this turn cost {2} more to cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Draw) && true { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn lasting_cost_4(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, player); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if true { CostDelta::more(2) } else { CostDelta::NONE } }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; item.subject.player().expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if !cant_draw(g, payer_0, 1) && cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)? {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; item.subject.player().into_iter().collect::<Vec<PlayerId>>() };
let mut done_2: i64 = 0; let mut drawn_2: Vec<ObjectId> = Vec::new(); for player in players_2 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_2); done_2 += g.history().drawn(player) as i64 - before; drawn_2.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
let paid_1 = done_2 > 0;
(paid_1, { let q: &dyn Query = g; (drawn_2).clone() }, Vec::new(), Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let players_3_0 = { let q: &dyn Query = g; item.subject.player().into_iter().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_3_0 { g.add_player_effect(asker.controller, source, EffectKind::SpellCostChange { card: item.ability.card, index: 0, player }, Duration::EndOfTurn); } }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Academy Loremaster", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of each player's draw step, that player may draw an additional card. If they do, spells they cast this turn cost {2} more to cast.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, lasting_cost_changes: &[LastingCostChange { text: "These spells cost more to cast.", change: lasting_cost_4, next: None }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
