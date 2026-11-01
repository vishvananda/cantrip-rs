// Opulent Clomper
// status: planned
// requires: a choice of Color outside its body
// requires: engine: no effect adds colors
// requires: operation ChooseAtRandom
// Vivid — Opulent Clomper's power and toughness are each equal to the number of colors among permanents you control.
// At the beginning of your upkeep, if this creature isn't all colors, it becomes a random color that it isn't in addition to its other colors.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = (({ let mut seen = ColorSet::NONE; for id in q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>() { for color in q.chars(id).colors.iter() { seen = seen.with(color); } } seen.iter().count() as i64 }).clamp(-32768, 32767) as i16, ({ let mut seen = ColorSet::NONE; for id in q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>() { for color in q.chars(id).colors.iter() { seen = seen.with(color); } } seen.iter().count() as i64 }).clamp(-32768, 32767) as i16); chars.power = Some(p); chars.toughness = Some(t); }
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| (q.chars(candidate).colors.iter().count() as i64) < (5i64)) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-choose-at-random" => cantrip_mechanics::choose::at_random(g, asker.source, ChoiceKind::Color, &[ChoosableColor::NotColorAmoungPermanents({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() })])?);
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::Indefinite;
let kind_2_0_0 = { let q: &dyn Query = g; planned_value!("engine: no effect adds colors", EffectKind::Pump(0, 0) => EffectKind::AddColors(ColorSet::of(planned_value!("a choice of Color outside its body", Color::White => q.state().chosen_color(q.linked(item)).expect("it chose one"))))) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Opulent Clomper", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ouphe]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, if this creature isn't all colors, it becomes a random color that it isn't in addition to its other colors.").targets(&[]).condition(ability_1_if)], statics: &[characteristic_defining(Layer::PtDefine, ability_0_apply_0, "Vivid — Opulent Clomper's power and toughness are each equal to the number of colors among permanents you control.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
