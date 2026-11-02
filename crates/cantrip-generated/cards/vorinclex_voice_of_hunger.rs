// Vorinclex, Voice of Hunger
// status: planned
// requires: continuous rule UntapStep
// requires: duration DuringControllersNextUntapStep
// requires: engine: only a mana ability triggers on tapping for mana
// Trample
// Whenever you tap a land for mana, add one mana of any type that land produced.
// Whenever an opponent taps a land for mana, that land doesn't untap during its controller's next untap step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TappedForMana { permanent, player, produced } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), Mana::ALL.iter().filter(|&&m| produced.get(m) > 0).fold(0u32, |types, &m| types | 1 << m as u32)]; if Some(asker.controller) == Some(*player) && q.chars(id).is(CardType::Land) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if item.data[1] != 0 { cantrip_mechanics::mana::add_one_mana_of_any_type_that_land_produced(g, item)?; }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = planned_value!("duration DuringControllersNextUntapStep", Duration::Indefinite => Duration::DuringItsControllersNextUntapStep);
let kinds_1 = { let mut all = Vec::new(); for &object in &objects_1 { let kind = { let q: &dyn Query = g; planned_value!("continuous rule UntapStep", EffectKind::CantBlock => EffectKind::DoesntUntap) }; all.push((object, kind));  } all }; if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &(object, kind) in &kinds_1 { g.add_effect(source, object, kind, duration_1); } }); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-only-a-mana-ability-triggers-on-tapping-for-mana" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TappedForMana { permanent, player, produced } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), Mana::ALL.iter().filter(|&&m| produced.get(m) > 0).fold(0u32, |types, &m| types | 1 << m as u32)]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&*player) && q.chars(id).is(CardType::Land) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Vorinclex, Voice of Hunger", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Praetor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::TAPPED.0), ability_1_event), ability_1, "Whenever you tap a land for mana, add one mana of any type that land produced.").targets(&[]).mana_ability(TriggeredMana::OneOfProduced),triggered(when_asked(planned_value!("engine: only a mana ability triggers on tapping for mana", EventMask(0) => EventMask(EventMask::TAPPED.0)), ability_2_event), ability_2, "Whenever an opponent taps a land for mana, that land doesn't untap during its controller's next untap step.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
