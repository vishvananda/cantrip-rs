// Khârn the Betrayer
// status: planned
// requires: prevention with an effect besides
// requires: static rule RequireCombat Block
// requires: trigger event Player LosesControl
// Berzerker — Khârn the Betrayer attacks or blocks each combat if able.
// Sigil of Corruption — When you lose control of Khârn the Betrayer, draw two cards.
// The Betrayer — If damage would be dealt to Khârn the Betrayer, prevent that damage and an opponent of your choice gains control of it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_1_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.must_attack += 1; }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_2_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2], done_prevented: u16) -> Result<(), Suspend> { let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_1) = cantrip_mechanics::choose::player(g, player_1, asker.source, candidates_1, "Choose", 0)? else { return Ok(()) };
let objects_2 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::Indefinite;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(chosen_1).expect("a reference that always names a player")) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
 Ok(()) }
#[allow(unused_variables, dead_code)] fn ability_2_then(g: &mut dyn Game, asker: &Asker, e: &cantrip_api::DamageEvent, prevented: u16) { let q: &dyn Query = g; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; let _ = ability_2_then_steps(g, asker, subject, data, prevented); }
#[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) { return true; } }
 false }
planned_item!("static-rule-require-combat-block" => must_block: ability_0_1_rule);
planned_item!("trigger-event-player-loses-control" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ControlChanged { permanent, previous: player, .. } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("prevention-with-an-effect-besides" => preventions: PreventionDef { hint: "The Betrayer — If damage would be dealt to Khârn the Betrayer, prevent that damage and an opponent of your choice gains control of it.", applies: ability_2_applies, amount: None, then: Some(ability_2_then) });

pub static CARD: CardDef = CardDef { name: "Khârn the Betrayer", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Astartes,Subtype::Berserker]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player LosesControl", EventMask(0) => EventMask(EventMask::CONTROL_CHANGED.0)), ability_1_event), ability_1, "Sigil of Corruption — When you lose control of Khârn the Betrayer, draw two cards.").targets(&[])], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_0_apply_0, "Berzerker — Khârn the Betrayer attacks or blocks each combat if able.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
