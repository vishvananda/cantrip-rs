// Omnath, Locus of All
// status: planned
// requires: player rule UnspentManaBecomes
// If you would lose unspent mana, that mana becomes black instead.
// At the beginning of your first main phase, look at the top card of your library. You may reveal that card if it has three or more colored mana symbols in its mana cost. If you do, add three mana in any combination of its colors and put it into your hand. If you don't reveal it, put it into your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::PrecombatMain) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_0, &objects_0);
g.end_sentence();
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_1 = { let mut revealed = false; if let Some(card) = { let q: &dyn Query = g; q.state().zone(Zone::library(payer_1)).first().copied() } { if { let q: &dyn Query = g; ({ let cost = q.mana_cost(card); i64::from(cost.symbols[..5].iter().map(|&n| u32::from(n)).sum::<u32>() + cost.twobrid.iter().map(|&n| u32::from(n)).sum::<u32>() + cost.phyrexian.iter().map(|&n| u32::from(n)).sum::<u32>()) + cost.hybrid.iter().flatten().count() as i64 }) >= (3i64) } && cantrip_mechanics::effects::ask_may(g, payer_1, asker.source, Purpose::Optional)? { g.reveal(payer_1, &[card]); revealed = true; } } revealed };
let revealed_1: Vec<ObjectId> = match paid_1 { true => g.state().zone(Zone::library(payer_1)).first().copied().into_iter().collect(), false => Vec::new() };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_1 as i64) != 0) } {
let colors_2 = { let q: &dyn Query = g; ((revealed_1.clone()).first().copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().fold(ColorSet::NONE, |all: ColorSet, o: ObjectId| ColorSet(all.0 | q.chars(o).colors.0)) }; for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } {
if !colors_2.is_empty() { for _ in 0..({ let q: &dyn Query = g; 3i64 }).max(0) { let colors = cantrip_mechanics::choose_colors(g, player, asker.source, colors_2, 1)?; g.add_mana(player, colors[0].mana(), 1); } }
}
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_3 = g.return_to_hand(&objects_3);
g.end_sentence();
hoisted_4.extend(moved_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let objects_6 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_6 = g.return_to_hand(&objects_6);
g.end_sentence();
hoisted_7.extend(moved_6.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");
planned_item!("player-rule-unspent-mana-becomes" => statics: player_rule(ability_0_0_applies, PlayerRule::UnspentManaBecomes(Some(Mana::Black))));

pub static CARD: CardDef = CardDef { name: "Omnath, Locus of All", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 1, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 1, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your first main phase, look at the top card of your library. You may reveal that card if it has three or more colored mana symbols in its mana cost. If you do, add three mana in any combination of its colors and put it into your hand. If you don't reveal it, put it into your hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
