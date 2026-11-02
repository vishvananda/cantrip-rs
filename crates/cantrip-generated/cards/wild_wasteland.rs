// Wild Wasteland
// status: planned
// requires: play permission to PlayLand
// requires: skipping a step of the turn
// Skip your draw step.
// At the beginning of your upkeep, exile the top two cards of your library. You may play those cards this turn.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("skipping-a-step-of-the-turn" => #[allow(unused_variables)] fn ability_0_0_skips_0(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller });
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((2i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
planned!("play-permission-to-play-land" => let cards = { let q: &dyn Query = g; moved_0.clone() }; let players = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players { g.allow_play(card, player, PlayGrant { expiry: Expiry::EndOfTurn, lands: true, spells: false, ..PlayGrant::CAST }); } } } }););
cantrip_api::instruction(g, |g| { let cards_3_0 = { let q: &dyn Query = g; moved_0.clone() }; let players_3_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_3_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_3_0 { g.allow_play(card, player, PlayGrant { lands: false, expiry: Expiry::EndOfTurn, ..PlayGrant::CAST }); } } } });
 });
g.end_sentence();
 Ok(()) }
planned_item!("skipping-a-step-of-the-turn" => skip_steps: SkipStep { applies: ability_0_0_skips_0, step: Step::Draw });

pub static CARD: CardDef = CardDef { name: "Wild Wasteland", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, exile the top two cards of your library. You may play those cards this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
