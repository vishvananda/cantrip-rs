// Swinging Ship
// status: planned
// requires: ability Visit
// requires: operation AddPhases
// requires: turn window BeforeFirstCombat
// Visit — After the first combat phase this turn, there's an additional combat phase. At the beginning of that combat, untap all creatures that attacked this turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_visit(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; planned_value!("turn window BeforeFirstCombat", false => q.state().turn.is_in(TurnWindow::BeforeFirstCombat)) } {
planned!("operation-add-phases" => g.add_phases({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, AddedPhases::Combat, { let q: &dyn Query = g; (1i64).max(0) as u32 }, Some("|g: &mut dyn Game| -> Result<(), Suspend> { let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(id) == Some(id) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_1: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_1 { g.untap(object); } });\ng.end_sentence();\nhoisted_2.extend(turned_1.clone());\nhoisted_3.extend(objects_1.clone());\nhoisted_4.extend({ let q: &dyn Query = g; turned_1.clone() });\n Ok(()) }")));
g.end_sentence();
hoisted_5.extend(hoisted_2.clone());
hoisted_6.extend(hoisted_3.clone());
hoisted_7.extend({ let q: &dyn Query = g; hoisted_4.clone() });
}
g.end_sentence();
 Ok(()) }
planned_item!("ability-visit" => abilities: cantrip_mechanics::attractions::visit(spell(&[], ability_0_visit), None, ""));

pub static CARD: CardDef = CardDef { name: "Swinging Ship", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Attraction]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
