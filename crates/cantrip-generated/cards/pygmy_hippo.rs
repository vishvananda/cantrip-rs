// Pygmy Hippo
// status: planned
// requires: continuous rule CombatDamage NoAssignment
// requires: engine: no effect empties a mana pool
// requires: keyword action ActivateManaAbility
// requires: numeric expression Game
// requires: operation Together
// Whenever this creature attacks and isn't blocked, you may have defending player activate a mana ability of each land they control and lose all unspent mana. If you do, this creature assigns no combat damage this turn and at the beginning of your next main phase this turn, you add an amount of {C} equal to the amount of mana that player lost this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Unblocked { attacker, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn delayed_11_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::PrecombatMain | Step::PostcombatMain) && true { return Some(Fired { subject, data }); } }
 None }
fn delayed_11(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_13 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_13_0 = { let q: &dyn Query = g; (planned_value!("numeric expression Game", 0i64 => q.game_value(item, GameValue::ManaLostThisWay))).max(0) as u32 };
for player in mana_players_13 {
g.add_mana(player, Mana::C, mana_13_0);}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_14: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
if let Some(present_7_0) = { let q: &dyn Query = g; item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) } { if let Some(present_8_1) = { let q: &dyn Query = g; item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) } { planned!("operation-together" => g.begin_simultaneous());
for bound_4294967290 in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() } {
planned!("keyword-action-activate-mana-ability" => { let player = { let q: &dyn Query = g; present_7_0 }; if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967290).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { cantrip_mechanics::mana::activate_mana_ability(g, player, object)?; }; });
}
planned!("engine-no-effect-empties-a-mana-pool" => { let player = { let q: &dyn Query = g; present_8_1 }; g.empty_mana_pool(player); });
planned!("operation-together" => g.end_simultaneous());
 } }
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let objects_9 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_9 = { let q: &dyn Query = g; Some(asker.source) }; let duration_9 = Duration::EndOfTurn;
let kind_9_0_0 = { let q: &dyn Query = g; planned_value!("continuous rule CombatDamage NoAssignment", EffectKind::CantBlock => EffectKind::AssignsNoCombatDamage) };
if let Some(source) = source_9 { cantrip_api::instruction(g, |g| { for &object in &objects_9 { g.add_effect(source, object, kind_9_0_0, duration_9);  } }); }
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_14.extend(objects_9.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Pygmy Hippo", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Hippo]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::UNBLOCKED.0), ability_0_event), ability_0, "Whenever this creature attacks and isn't blocked, you may have defending player activate a mana ability of each land they control and lose all unspent mana. If you do, this creature assigns no combat damage this turn and at the beginning of your next main phase this turn, you add an amount of {C} equal to the amount of mana that player lost this way.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), delayed_11_event), delayed_11, "If you do, this creature assigns no combat damage this turn and at the beginning of your next main phase this turn, you add an amount of {C} equal to the amount of mana that player lost this way.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
