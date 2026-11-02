// I Am Duskmourn
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: operation AbandonScheme
// requires: supertype Ongoing
// (An ongoing scheme remains face up until it's abandoned.)
// At the beginning of your end step, you may cast a spell from your hand without paying its mana cost. If you do, abandon this scheme.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let mut paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
let caster_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_1: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_1: u32 = 1; let mut played_1: Vec<ObjectId> = Vec::new();
while left_1 > 0 {
let mut open_1: Vec<ObjectId> = Vec::new(); for &c in &pool_1 { let ways = g.cast_ways_during_resolution_by(c, caster_1); if ways.iter().any(|w| true) { open_1.push(c); } }
let picked_1 = if open_1.is_empty() { None } else if open_1.len() > 1 || (true && (!false || pool_1.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_1, asker.source, Purpose::CastFree, pool_1.clone(), open_1.clone(), 1, false)?.first().copied() } else if !false || cantrip_mechanics::effects::ask_may(g, caster_1, open_1[0], Purpose::CastFree)? { Some(open_1[0]) } else { None };
let Some(card_1) = picked_1 else { break; };
let ways_1: Vec<CastWay> = g.cast_ways_during_resolution_by(card_1, caster_1).into_iter().filter(|w| true).collect();
let done_1 = if ways_1.is_empty() { g.play_land_during_resolution(card_1)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_1, card_1, ways_1)?.map(|w| w.way); g.cast_during_resolution_as(card_1, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_1), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_1 { left_1 -= 1; played_1.push(g.state().obj(card_1).became.unwrap_or(card_1)); } else if true { break; } if done_1 || !(false && true) { pool_1.retain(|&c| c != card_1); }
}
paid_0 = !played_1.is_empty();
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
planned!("operation-abandon-scheme" => cantrip_mechanics::scheme::abandon(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source));
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "I Am Duskmourn", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: planned_value!("supertype Ongoing", Supertypes { basic: false, legendary: false, snow: false } => Supertypes { basic: false, legendary: false, snow: false, ongoing: true }), colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your end step, you may cast a spell from your hand without paying its mana cost. If you do, abandon this scheme.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
