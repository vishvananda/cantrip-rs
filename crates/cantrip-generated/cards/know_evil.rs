// Know Evil
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: player rule for a while Prohibit AttackWith
// requires: player rule for a while can't cast some spells
// requires: target reference past a slot of no fixed size
// requires: trigger event Player SetsSchemeInMotion
// When you set this scheme in motion, until your next turn, up to one target opponent can't attack with creatures, up to one target opponent can't cast creature spells, and up to one target opponent can't cast noncreature spells. You can't choose any player as a target more than once.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
#[allow(unused_variables)] fn ability_0_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { (matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p))) && !q.chosen_targets().iter().skip(0).copied().collect::<Vec<Target>>().contains(&t) }
#[allow(unused_variables)] fn ability_0_target_2(q: &dyn Query, asker: &Asker, t: Target) -> bool { (matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p))) && !planned_value!("target reference past a slot of no fixed size", Vec::<Target>::new() => q.chosen_targets_of(0)).contains(&t) && !planned_value!("target reference past a slot of no fixed size", Vec::<Target>::new() => q.chosen_targets_of(1)).contains(&t) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, 1),target(ability_0_target_1).between(0, 1),target(ability_0_target_2).between(0, 1)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
planned_item!("player-rule-for-a-while-can-t-cast-some-spells" => #[allow(unused_variables)] fn lasting_spell_rule_0(q: &dyn Query, asker: &Asker, about: Target, player: PlayerId, spell: ObjectId) -> bool { let source = asker.source; q.chars(spell).is(CardType::Creature) });
planned_item!("player-rule-for-a-while-can-t-cast-some-spells" => #[allow(unused_variables)] fn lasting_spell_rule_1(q: &dyn Query, asker: &Asker, about: Target, player: PlayerId, spell: ObjectId) -> bool { let source = asker.source; !(q.chars(spell).is(CardType::Creature)) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let t_2_raw = announced(g, item, 2, false); let t_2 = announced(g, item, 2, true);
planned!("player-rule-for-a-while-prohibit-attack-with" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Cant(Prohibition::AttackWith(|q: &dyn Query, asker: &Asker, attacker: ObjectId| true))), Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") })); } });
g.end_sentence();
cantrip_api::instruction(g, |g| { planned!("player-rule-for-a-while-can-t-cast-some-spells" => let players_2_0 = { let q: &dyn Query = g; t_1_raw.get(0).copied().filter(|t| t_1.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_2_0 { g.add_player_effect(asker.controller, source, EffectKind::CantCast { player, card: item.ability.card, index: 0, about: Target::None }, Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") })); } });
 });
g.end_sentence();
cantrip_api::instruction(g, |g| { planned!("player-rule-for-a-while-can-t-cast-some-spells" => let players_3_0 = { let q: &dyn Query = g; t_2_raw.get(0).copied().filter(|t| t_2.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_3_0 { g.add_player_effect(asker.controller, source, EffectKind::CantCast { player, card: item.ability.card, index: 1, about: Target::None }, Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") })); } });
 });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-player-sets-scheme-in-motion" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SchemeSetInMotion { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("player-rule-for-a-while-can-t-cast-some-spells" => lasting_spell_rules: lasting_spell_rule_0);
planned_item!("player-rule-for-a-while-can-t-cast-some-spells" => lasting_spell_rules: lasting_spell_rule_1);
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "Know Evil", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player SetsSchemeInMotion", EventMask(0) => EventMask(EventMask::SCHEME_SET.0)), ability_0_event), ability_0, "When you set this scheme in motion, until your next turn, up to one target opponent can't attack with creatures, up to one target opponent can't cast creature spells, and up to one target opponent can't cast noncreature spells. You can't choose any player as a target more than once.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
