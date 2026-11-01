// Frontier Siege
// status: ready
// As this enchantment enters, choose Khans or Dragons.
// • Khans — At the beginning of each of your main phases, add {G}{G}.
// • Dragons — Whenever a creature you control with flying enters, you may have it fight target creature you don't control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..2)?; let chosen_0: u8 = entering.chosen_option.map_or(0, |chosen| chosen.index);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::PrecombatMain | Step::PostcombatMain) && Some(asker.controller) == Some(q.state().turn.active) && cantrip_mechanics::choose::chosen_option(q, asker, &CARD) == Some(0) { return Some(Fired { subject, data }); } }
 None }
fn ability_1_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::G, mana_2_0);}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id))))) }
static ABILITY_2_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_0_target_0)];
#[allow(unused_variables)] fn ability_2_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && q.chars(id).keywords.has(Keyword::Flying)) && cantrip_mechanics::choose::chosen_option(q, asker, &CARD) == Some(1) { return Some(Fired { subject, data }); } }
 None }
fn ability_2_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let decider_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_3, asker.source, Purpose::Optional)? {
let first_4 = ({ let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied(); let second_4 = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied();
if let (Some(first), Some(second)) = (first_4, second_4) { g.fight(first, second); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Frontier Siege", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_0_event), ability_1_0, "As this enchantment enters, choose Khans or Dragons. Khans — At the beginning of each of your main phases, add {G}{G}; or Dragons — Whenever a creature you control with flying enters, you may have it fight target creature you don't control.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_0_event), ability_2_0, "As this enchantment enters, choose Khans or Dragons. Khans — At the beginning of each of your main phases, add {G}{G}; or Dragons — Whenever a creature you control with flying enters, you may have it fight target creature you don't control.").targets(ABILITY_2_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, labels: &["Khans", "Dragons"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
