// I Know All, I See All
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: operation AbandonScheme
// requires: static rule UntapStep
// requires: supertype Ongoing
// (An ongoing scheme remains face up until it's abandoned.)
// Untap all permanents you control during each opponent's untap step.
// At the beginning of each end step, if three or more cards were put into your graveyard this turn from anywhere, abandon this scheme.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); Some(asker.controller) == Some(q.controller(affected)) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; Some(asker.controller) == Some(q.controller(id)) }
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && (true && Some(asker.controller) == Some(q.state().obj(id).owner)) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (3i64) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-abandon-scheme" => cantrip_mechanics::scheme::abandon(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source));
g.end_sentence();
 Ok(()) }
planned_item!("static-rule-untap-step" => statics: object_rule(ability_0_0_rule, Rule::UntapStep { of: |q: &dyn Query, asker: &Asker, id: ObjectId| (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.state().turn.active), behavior: UntapBehavior::Additional }));
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "I Know All, I See All", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: planned_value!("supertype Ongoing", Supertypes { basic: false, legendary: false, snow: false } => Supertypes { basic: false, legendary: false, snow: false, ongoing: true }), colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of each end step, if three or more cards were put into your graveyard this turn from anywhere, abandon this scheme.").targets(&[]).condition(ability_1_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
