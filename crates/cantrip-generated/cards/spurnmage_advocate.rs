// Spurnmage Advocate
// status: ready
// {T}: Return two target cards from an opponent's graveyard to their hand. Destroy target attacking creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.state().obj(id).zone.kind() == ZoneKind::Graveyard && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.state().obj(id).owner)))) }
#[allow(unused_variables)] fn ability_0_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (((q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(2, 2).together(cantrip_api::select::set_part(&(common_owner as SelectFn), &(largest_common_owner as cantrip_api::select::LargestFn), 0, "with the same owner")),target(ability_0_target_1)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; t_1_raw.get(0).copied().filter(|t| t_1.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_1 = g.destroy_all(&objects_1);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn common_by(q: &dyn Query, choice: &Choice, key: fn(&dyn Query, ObjectId) -> PlayerId) -> Verdict { let first = choice.selected.first().map(|&c| key(q, c)); Verdict { complete: choice.selected.iter().all(|&c| Some(key(q, c)) == first), addable: Bits::of(choice.candidates.len(), |i| first.map_or(true, |k| key(q, choice.candidates[i]) == k)) } }
fn largest_by(q: &dyn Query, choice: &Choice, key: fn(&dyn Query, ObjectId) -> PlayerId) -> Vec<ObjectId> { choice.candidates.iter().map(|&a| choice.candidates.iter().copied().filter(|&b| key(q, b) == key(q, a)).collect::<Vec<_>>()).max_by_key(Vec::len).unwrap_or_default() }
fn owner_of(q: &dyn Query, id: ObjectId) -> PlayerId { q.state().obj(id).owner }
fn common_owner(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { common_by(q, choice, owner_of) }
fn largest_common_owner(q: &dyn Query, choice: &Choice, _: u32) -> Vec<ObjectId> { largest_by(q, choice, owner_of) }

pub static CARD: CardDef = CardDef { name: "Spurnmage Advocate", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Nomad]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{T}: Return two target cards from an opponent's graveyard to their hand. Destroy target attacking creature.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
