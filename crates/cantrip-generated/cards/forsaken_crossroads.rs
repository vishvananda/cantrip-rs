// Forsaken Crossroads
// status: planned
// requires: engine: no untap symbol cost
// This land enters tapped. As it enters, choose a color.
// When this land enters, scry 1. If you weren't the starting player, you may untap this land instead.
// {T}: Add one mana of the chosen color.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
let player_1 = entering.controller; let Answer::Colors(colors_1) = g.ask(player_1, RequestKind::ChooseColors { source: entering.object, choices: ColorSet(31), count: 1, purpose: ColorUse::Any })? else { return Ok(()) }; let chosen_1 = colors_1[0];
entering.chosen_color = Some(chosen_1);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; !(Some(asker.controller).is_some_and(|player: PlayerId| q.state().turn.starting == player)) } {
let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_2 = g.may_pay(payer_2, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no untap symbol cost", Cost::NONE => Cost { untap: true, ..Cost::NONE })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_2 as i64) != 0)) } {
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_3, asker.source, amount_3)?;
g.end_sentence();
}
g.end_sentence();
} else {
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_4, asker.source, amount_4)?;
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn ability_2_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_5 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_5);}
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Forsaken Crossroads", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this land enters, scry 1. If you weren't the starting player, you may untap this land instead.").targets(&[]),mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_2_mana).choose_color().colors(ColorSet(31)).yields_any(1), "{T}: Add one mana of the chosen color.").colors_from(cantrip_mechanics::lands::chosen_color)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
