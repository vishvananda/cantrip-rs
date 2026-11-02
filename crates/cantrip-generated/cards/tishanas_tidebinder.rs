// Tishana's Tidebinder
// status: ready
// Flash
// When this creature enters, counter up to one target activated or triggered ability. If an ability of an artifact, creature, or planeswalker is countered this way, that permanent loses all abilities for as long as this creature remains on the battlefield. (Mana abilities can't be targeted.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { q.state().stack.iter().any(|e| matches!(t, Target::Ability(serial) if e.kind != StackKind::Spell && e.serial == serial) && ((e.kind == StackKind::Activated || e.kind == StackKind::Triggered))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 1)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let items_0: Vec<(ObjectId, usize)> = t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).into_iter().collect::<Vec<Target>>().into_iter().filter_map(|t| { let stack = &g.state().stack; stack.iter().position(|e| matches!(t, Target::Ability(serial) if e.kind != StackKind::Spell && e.serial == serial)).map(|at| (stack[at].object, at)) }).collect(); let abilities_0: Vec<ObjectId> = items_0.iter().filter(|&&(_, at)| g.state().stack[at].kind != StackKind::Spell).map(|&(o, _)| o).filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect(); let spells_0: Vec<ObjectId> = items_0.iter().filter(|&&(_, at)| g.state().stack[at].kind == StackKind::Spell).map(|&(o, _)| o).collect(); let countered_0 = g.counter_items(&items_0); let mut moved_0: Vec<ObjectId> = if countered_0 > 0 { abilities_0 } else { Vec::new() }; moved_0.extend(spells_0.into_iter().filter_map(|o| g.state().obj(o).became));
g.end_sentence();
if { let q: &dyn Query = g; (((moved_0.clone()).into_iter().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) } {
let objects_1 = { let q: &dyn Query = g; ((moved_0.clone()).first().copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::WhileSourceOnBattlefield;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::LoseAllAbilities };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
hoisted_2.extend(objects_1.clone());
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Tishana's Tidebinder", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Merfolk,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, counter up to one target activated or triggered ability. If an ability of an artifact, creature, or planeswalker is countered this way, that permanent loses all abilities for as long as this creature remains on the battlefield.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
