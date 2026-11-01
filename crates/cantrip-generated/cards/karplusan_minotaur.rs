// Karplusan Minotaur
// status: planned
// requires: cumulative upkeep paid other than in mana or life
// requires: operation CoinFlips
// requires: targets chosen by another player
// Cumulative upkeep—Flip a coin. (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)
// Whenever you win a coin flip, this creature deals 1 damage to any target.
// Whenever you lose a coin flip, this creature deals 1 damage to any target of an opponent's choice.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_payment(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-coin-flips" => cantrip_mechanics::coins::flips(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, Some({ let q: &dyn Query = g; (1i64).max(0) as u32 }), false, false, true)?);
g.end_sentence();
 Ok(()) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::CoinFlipped { player, result: CoinFlip::Won } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_1 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_1 { let batch: Vec<(ObjectId, Target, u16)> = recipients_1.iter().map(|t| (source, *t, amount_1)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[planned_value!("targets chosen by another player", cantrip_mechanics::targets::ANY => cantrip_mechanics::targets::ANY.chosen_by_one_of(|q: &dyn Query, asker: &Asker| Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()))];
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::CoinFlipped { player, result: CoinFlip::Lost } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_2 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_2 { let batch: Vec<(ObjectId, Target, u16)> = recipients_2.iter().map(|t| (source, *t, amount_2)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("cumulative-upkeep-paid-other-than-in-mana-or-life" => abilities: cantrip_mechanics::cumulative_upkeep!(cantrip_mechanics::cumulative_upkeep::Payment::Instructions(ability_0_payment), "Cumulative upkeep"));

pub static CARD: CardDef = CardDef { name: "Karplusan Minotaur", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Minotaur,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::COIN_FLIPPED.0), ability_1_event), ability_1, "Whenever you win a coin flip, this creature deals 1 damage to any target.").targets(ABILITY_1_TARGETS),triggered(when_asked(EventMask(EventMask::COIN_FLIPPED.0), ability_2_event), ability_2, "Whenever you lose a coin flip, this creature deals 1 damage to any target of an opponent's choice.").targets(ABILITY_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
