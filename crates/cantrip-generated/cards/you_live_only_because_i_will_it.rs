// You Live Only Because I Will It
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: trigger event Player SetsSchemeInMotion
// When you set this scheme in motion, you may redistribute any number of life totals. (Each affected player or team gets one of those life totals back.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let totals_1: Vec<i32> = players_1.iter().map(|&p| g.state().player(p).life).collect(); let mut open_1 = players_1.clone(); let mut sets_1: Vec<(PlayerId, i32)> = Vec::new(); for &total in &totals_1 { let who = match open_1.as_slice() { [one] => Some(*one), _ => cantrip_mechanics::choose::player(g, asker.controller, asker.source, open_1.clone(), "GetLifeTotal", total.clamp(0, 65535) as u16)? }; if let Some(who) = who { open_1.retain(|&p| p != who); sets_1.push((who, total)); } } cantrip_api::instruction(g, |g| { for &(who, total) in &sets_1 { g.set_life_total(who, total); } });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-sets-scheme-in-motion" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SchemeSetInMotion { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "You Live Only Because I Will It", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player SetsSchemeInMotion", EventMask(0) => EventMask(EventMask::SCHEME_SET.0)), ability_0_event), ability_0, "When you set this scheme in motion, you may redistribute any number of life totals.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
