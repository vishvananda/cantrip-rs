// The Ruinous Powers
// status: planned
// requires: play permission with an effect on what is played
// At the beginning of your upkeep, choose an opponent at random. Exile the top card of that player's library. Until end of turn, you may play that card and you may spend mana as though it were mana of any color to cast it. When you cast a spell this way, its owner loses life equal to its mana value.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let pool_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; if pool_1.is_empty() { return Ok(()); } let chosen_1 = pool_1[g.choose_at_random(pool_1.len())];
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; (((Some(chosen_1).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
planned!("play-permission-with-an-effect-on-what-is-played" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Play { zone: ZoneKind::Exile, cards: CardsOfRule::These({ let q: &dyn Query = g; (moved_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }), grant: PlayGrant { lands: true, spend_as: SpendAs::AnyColor, rider: Some(&[|g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { let amount_5 = { let q: &dyn Query = g; ((moved_2.clone()).get(0 as usize).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))) as i32 }; let players_5 = { let q: &dyn Query = g; (moved_2.clone()).get(0 as usize).copied().map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.lose_life(player, amount_5); }
g.end_sentence();
 Ok(()) }]), ..PlayGrant::CAST } }), Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Ruinous Powers", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, choose an opponent at random. Exile the top card of that player's library. Until end of turn, you may play that card and you may spend mana as though it were mana of any color to cast it. When you cast a spell this way, its owner loses life equal to its mana value.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
