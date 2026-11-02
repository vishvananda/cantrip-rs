// City of the Daleks
// status: planned
// requires: card type Plane
// requires: layout Planar
// requires: subtype Skaro
// requires: trigger event Player ChaosEnsues
// Whenever you attack, target opponent loses X life, where X is the number of artifacts you control.
// Whenever chaos ensues, for each opponent, you create a 3/3 black Dalek artifact creature token with menace that attacks that opponent this turn if able. Those tokens gain haste. Sacrifice them at the beginning of the next end step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AttackersDeclared { player, count } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if *count > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_0 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 2147483647) as i32 }; let players_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
pub mod token_4 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Dalek).name, token_words: token!(Dalek).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dalek]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn delayed_7(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_8 = { let q: &dyn Query = g; about(q, item, 0) }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_8: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_8 { let controller = g.controller(object); if players_8.contains(&controller) && g.can_sacrifice(controller, object) { picks_8.push((controller, object)); } }
let sacrificed_8 = g.sacrifice_each(&picks_8);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_1.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut gathered_2: Vec<(CardId, PlayerId, Target)> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let def_3 = g.card_id(&token_4::CARD); let count_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let defender_3 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
gathered_2.extend(std::iter::repeat((def_3, player_3, Target::Player(defender_3))).take(count_3 as usize));
    } 
let words_2 = g.words(asker.source); let mut created_2: Vec<ObjectId> = Vec::new();
let rows_2: Vec<(CardId, PlayerId)> = gathered_2.iter().map(|row| (row.0, row.1)).collect(); created_2.extend(g.create_tokens_worded(&rows_2, false, None, words_2)?); if !gathered_2.is_empty() { let per_2 = created_2.len().div_ceil(gathered_2.len()); g.one_instruction(&mut |g| { for (at, &token) in created_2.iter().enumerate() { let target = gathered_2[(at / per_2).min(gathered_2.len() - 1)].2; g.add_effect(asker.source, token, EffectKind::MustAttackAt(target), Duration::EndOfTurn); } Ok(()) })?; }
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; (created_2.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let duration_5 = Duration::Indefinite;
let kind_5_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_5 { cantrip_api::instruction(g, |g| { for &object in &objects_5 { g.add_effect(source, object, kind_5_0_0, duration_5);  } }); }
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_2.clone() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "City of the Daleks", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Skaro", Subtype::Plains => Subtype::Skaro)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKERS_DECLARED.0), ability_0_event), ability_0, "Whenever you attack, target opponent loses X life, where X is the number of artifacts you control.").targets(ABILITY_0_TARGETS),triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_1_event), ability_1, "Whenever chaos ensues, for each opponent, you create a 3/3 black Dalek artifact creature token with menace that attacks that opponent this turn if able. Those tokens gain haste. Sacrifice them at the beginning of the next end step.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_7, "Sacrifice them at the beginning of the next end step.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_4::CARD];
