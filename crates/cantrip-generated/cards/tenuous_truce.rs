// Tenuous Truce
// status: ready
// Enchant opponent
// At the beginning of enchanted opponent's end step, you and that player each draw a card.
// When you attack enchanted opponent or a planeswalker they control or when they attack you or a planeswalker you control, sacrifice this Aura.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_player) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; { let mut all: Vec<PlayerId> = Vec::new(); for p in Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } for p in item.subject.player().into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } all } };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if true && Some(asker.controller) == Some(q.controller(id)) && match *defender { Target::Player(p) => Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_player) == Some(p), Target::Object(o) => q.chars(o).is(CardType::Planeswalker) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_player) == Some(q.controller(o)), _ => false } { return Some(Fired { subject, data }); } }
if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if true && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_player) == Some(q.controller(id)) && match *defender { Target::Player(p) => Some(asker.controller) == Some(p), Target::Object(o) => q.chars(o).is(CardType::Planeswalker) && Some(asker.controller) == Some(q.controller(o)), _ => false } { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_1: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_1 { let controller = g.controller(object); if players_1.contains(&controller) && g.can_sacrifice(controller, object) { picks_1.push((controller, object)); } }
let sacrificed_1 = g.sacrifice_each(&picks_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Tenuous Truce", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of enchanted opponent's end step, you and that player each draw a card.").targets(&[]),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "When you attack enchanted opponent or a planeswalker they control or when they attack you or a planeswalker you control, sacrifice this Aura.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
