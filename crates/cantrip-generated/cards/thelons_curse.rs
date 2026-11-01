// Thelon's Curse
// status: planned
// requires: earlier objects Selected
// requires: operation ChooseAndPay
// Blue creatures don't untap during their controllers' untap steps.
// At the beginning of each player's upkeep, that player may choose any number of tapped blue creatures they control and pay {U} for each creature chosen this way. If the player does, untap those creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).colors.has(q.words(asker.source).color(Color::Blue)) && (*so_far).is(CardType::Creature)) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).colors.has(q.words(asker.source).color(Color::Blue)) && q.chars(id).is(CardType::Creature)) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let chosen_0: Vec<ObjectId> = planned_value!("operation ChooseAndPay", Vec::<ObjectId>::new() => cantrip_mechanics::players::choose_and_pay(g, { let q: &dyn Query = g; item.subject.player().expect("a reference that always names a player") }, asker.source, { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.state().obj(id).tapped && q.chars(id).colors.has(q.words(asker.source).color(Color::Blue)) && q.chars(id).is(CardType::Creature)) && item.subject.player() == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }, "Cost { parts: [Mana([Color(Blue)])] }")?);
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (planned_value!("earlier objects Selected", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Selected))).contains(&id)).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_1: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_1 { g.untap(object); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Thelon's Curse", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of each player's upkeep, that player may choose any number of tapped blue creatures they control and pay {U} for each creature chosen this way. If the player does, untap those creatures.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[ability_0_0_rule], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
