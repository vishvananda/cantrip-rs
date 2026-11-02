// Kardur, Doomscourge
// status: planned
// requires: continuous rule RequireCombat Attack
// requires: engine: no resolving rule on an open set: RequireCombat Attack
// requires: engine: no resolving rule on an open set: RequireCombat Attack beyond lasting rules
// When Kardur enters, until your next turn, creatures your opponents control attack each combat if able and attack a player other than you if able.
// Whenever an attacking creature dies, each opponent loses 1 life and you gain 1 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
planned_item!("engine-no-resolving-rule-on-an-open-set-require-combat-attack" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))) });
planned_item!("engine-no-resolving-rule-on-an-open-set-require-combat-attack-beyond-lasting-rules" => #[allow(unused_variables)] fn lasting_rule_1(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") });
planned!("engine-no-resolving-rule-on-an-open-set-require-combat-attack-beyond-lasting-rules" => cantrip_api::instruction(g, |g| { g.add_lasting_rule(item, 0, about_0, until_0); g.add_lasting_rule(item, 1, about_0, until_0); }););
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && ((q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.gain_life(player, amount_2); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-require-combat-attack-beyond-lasting-rules" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "When Kardur enters, until your next turn, creatures your opponents control attack each combat if able and attack a player other than you if able.", rule: LastingRuleKind::MustAttack, applies: lasting_rule_0 },LastingRule { text: "When Kardur enters, until your next turn, creatures your opponents control attack each combat if able and attack a player other than you if able.", rule: LastingRuleKind::ObjectRule(0), applies: lasting_rule_1 }];);
planned_item!("continuous-rule-require-combat-attack" => lasting_object_rules: LastingObjectRule { text: "When Kardur enters, until your next turn, creatures your opponents control attack each combat if able and attack a player other than you if able.", rule: Rule::Must(ObjectAction::Attack(|q: &dyn Query, asker: &Asker, defender: Target| (match defender { Target::Player(p) => ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&p), Target::Object(id) => false && q.chars(id).is(CardType::Planeswalker) && ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id)), _ => false })), Required::AtLeast(1i64)) });

pub static CARD: CardDef = CardDef { name: "Kardur, Doomscourge", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon,Subtype::Berserker]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Kardur enters, until your next turn, creatures your opponents control attack each combat if able and attack a player other than you if able.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever an attacking creature dies, each opponent loses 1 life and you gain 1 life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
