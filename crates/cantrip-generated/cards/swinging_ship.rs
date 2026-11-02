// Swinging Ship
// status: planned
// requires: ability Visit
// requires: operation AddPhases
// Visit — After the first combat phase this turn, there's an additional combat phase. At the beginning of that combat, untap all creatures that attacked this turn.
#[allow(unused_imports)] use cantrip_api::*;
fn delayed_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && ({ let mut attackers: Vec<ObjectId> = q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Attacks { attacker, defending, known, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(id) == Some(id) { return Some(Fired { subject, data }); } }  None }).filter_map(|f: Fired| f.subject.object()).collect(); attackers.sort(); attackers.dedup(); attackers.len() as i64 }) >= (1i64))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_2: Vec<ObjectId> = objects_2.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_2 { g.untap(object); } });
g.end_sentence();
 Ok(()) }
fn ability_0_visit(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; !q.history().this_turn().iter().any(|r| matches!(r.event, Event::StepBegan { step: Step::BeginCombat })) } {
planned!("operation-add-phases" => g.add_phases({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, AddedPhases::Combat, { let q: &dyn Query = g; (1i64).max(0) as u32 }, Some(AbilityRef::delayed(item.ability.card, 0))));
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("ability-visit" => abilities: cantrip_mechanics::attractions::visit_ability(spell(&[], ability_0_visit), None, ""));

pub static CARD: CardDef = CardDef { name: "Swinging Ship", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Attraction]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_1, "At the beginning of that combat, untap all creatures that attacked this turn.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
