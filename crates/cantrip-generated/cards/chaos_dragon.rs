// Chaos Dragon
// status: planned
// requires: continuous rule Prohibit Attack
// requires: player collection Scoped
// requires: player state RolledHighest
// Flying, haste
// This creature attacks each combat if able.
// At the beginning of combat on your turn, each player rolls a d20. If one or more opponents had the highest result, this creature can't attack those players or planeswalkers they control this combat.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.must_attack += 1; }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let value_1 = i64::from(g.roll_die(player_1, 20).result);
 each_2[index_0] = value_1;
   } 
g.end_sentence();
if { let q: &dyn Query = g; (({ let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|player: PlayerId| planned_value!("player state RolledHighest", false => q.rolled_highest(asker, player)))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }).len() as i64) >= (1i64) } {
let objects_3 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfCombat;
let kind_3_0_0 = { let q: &dyn Query = g; planned_value!("continuous rule Prohibit Attack", EffectKind::CantBlock => EffectKind::Rule(Rule::Cant(Action::Attack(|q: &dyn Query, asker: &Asker, defender: Target| (match defender { Target::Player(p) => (q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|p: PlayerId| (planned_value!("player collection Scoped", Vec::<PlayerId>::new() => q.players_in_scope(asker))).contains(&p))).collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => true && q.chars(id).is(CardType::Planeswalker) && (q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|p: PlayerId| (planned_value!("player collection Scoped", Vec::<PlayerId>::new() => q.players_in_scope(asker))).contains(&p))).collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false }))))) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
hoisted_4.extend(objects_3.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Chaos Dragon", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_3_event), ability_3, "At the beginning of combat on your turn, each player rolls a d20. If one or more opponents had the highest result, this creature can't attack those players or planeswalkers they control this combat.").targets(&[])], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_2_apply_0, "This creature attacks each combat if able.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
