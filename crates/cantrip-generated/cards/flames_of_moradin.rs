// Flames of Moradin
// status: planned
// requires: engine: no perpetual effects
// requires: operation Conjure
// Destroy up to three target artifacts. Conjure a duplicate of each nontoken artifact destroyed this way into your hand. The duplicates perpetually gain "You may pay {R} rather than pay this spell's mana cost" and "At the beginning of your end step, sacrifice this artifact."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Artifact))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, 3)];
static PERPETUAL_4: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ZoneKind::Hand, "You may pay {R} rather than pay this spell's mana cost")], ..CardDef::BLANK };
#[allow(unused_variables)] fn perpetual_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn perpetual_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_6 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_6: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_6 { let controller = g.controller(object); if players_6.contains(&controller) && g.can_sacrifice(controller, object) { picks_6.push((controller, object)); } }
let sacrificed_6 = g.sacrifice_each(&picks_6);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
let duplicated_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).token) && q.chars(id).is(CardType::Artifact))).collect::<Vec<ObjectId>>() };
let count_1: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_1: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_1.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::DuplicateOf(&duplicated_1), count_1, ConjureTo::Hand)?)); }
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_3: Vec<ObjectId> = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (conjured_1.clone()).contains(&id)).collect::<Vec<ObjectId>>() };
for &object in &objects_3 { let kind_3_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_4) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_0); let kind_3_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_1); });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Flames of Moradin", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), perpetual_5_event), perpetual_5, "At the beginning of your end step, sacrifice this artifact.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
