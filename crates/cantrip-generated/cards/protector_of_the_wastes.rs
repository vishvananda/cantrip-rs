// Protector of the Wastes
// status: ready
// Flying
// When this creature enters or becomes monstrous, exile up to two target artifacts and/or enchantments controlled by different players.
// {4}{W}: Monstrosity 3. (If this creature isn't monstrous, put three +1/+1 counters on it and it becomes monstrous.)
#[allow(unused_imports)] use cantrip_api::*;
fn different_controllers_key(q: &dyn Query, id: ObjectId) -> PlayerId { q.controller(id) }
fn different_controllers(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let used: Vec<PlayerId> = choice.selected.iter().map(|&c| different_controllers_key(q, c)).collect(); let apart = used.iter().enumerate().all(|(i, k)| !used[..i].contains(k)); Verdict { complete: apart, addable: Bits::of(choice.candidates.len(), |i| !used.contains(&different_controllers_key(q, choice.candidates[i]))) } }
fn largest_different_controllers(q: &dyn Query, choice: &Choice, _: u32) -> Vec<ObjectId> { cantrip_api::select::largest_distinct(choice, |c| different_controllers_key(q, c)) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Enchantment)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 2).together(cantrip_api::select::set_part(&(different_controllers as SelectFn), &(largest_different_controllers as cantrip_api::select::LargestFn), 0, "controlled by different players"))];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::Monstrous { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (3i64).max(0) as i16 }; g.monstrosity(asker.source, amount_1);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Protector of the Wastes", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::MONSTROUS.0), ability_1_event), ability_1, "When this creature enters or becomes monstrous, exile up to two target artifacts and/or enchantments controlled by different players.").targets(ABILITY_1_TARGETS),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{4}{W}: Monstrosity 3.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
