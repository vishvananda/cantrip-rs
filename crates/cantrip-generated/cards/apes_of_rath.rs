// Apes of Rath
// status: planned
// requires: continuous rule UntapStep
// requires: duration DuringControllersNextUntapStep
// Whenever this creature attacks, it doesn't untap during its controller's next untap step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = planned_value!("duration DuringControllersNextUntapStep", Duration::Indefinite => Duration::DuringItsControllersNextUntapStep);
let kinds_0 = { let mut all = Vec::new(); for &object in &objects_0 { let kind = { let q: &dyn Query = g; planned_value!("continuous rule UntapStep", EffectKind::CantBlock => EffectKind::DoesntUntap) }; all.push((object, kind));  } all }; if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &(object, kind) in &kinds_0 { g.add_effect(source, object, kind, duration_0); } }); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Apes of Rath", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ape]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever this creature attacks, it doesn't untap during its controller's next untap step.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
