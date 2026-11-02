// Spectral Bears
// status: planned
// requires: continuous rule UntapStep
// requires: duration DuringControllersNextUntapStep
// Whenever this creature attacks, if defending player controls no black nontoken permanents, it doesn't untap during your next untap step.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { !(q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.source).and_then(|o: ObjectId| q.defending_player(o)).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).colors.has(q.words(asker.source).color(Color::Black)) && !(q.state().obj(id).token))).count() >= 1) }
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = planned_value!("duration DuringControllersNextUntapStep", Duration::Indefinite => Duration::DuringItsControllersNextUntapStep);
let kinds_0 = { let mut all = Vec::new(); for &object in &objects_0 { let kind = { let q: &dyn Query = g; planned_value!("continuous rule UntapStep", EffectKind::CantBlock => EffectKind::DoesntUntap) }; all.push((object, kind));  } all }; if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &(object, kind) in &kinds_0 { g.add_effect(source, object, kind, duration_0); } }); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Spectral Bears", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bear,Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever this creature attacks, if defending player controls no black nontoken permanents, it doesn't untap during your next untap step.").targets(&[]).condition(ability_0_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
