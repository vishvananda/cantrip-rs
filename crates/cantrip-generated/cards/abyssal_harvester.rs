// Abyssal Harvester
// status: ready
// {T}: Exile target creature card from a graveyard that was put there this turn. Create a token that's a copy of it, except it's a Nightmare in addition to its other types. Then exile all other Nightmare tokens you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.chars(id).is(CardType::Creature) && cantrip_mechanics::history::arrived_this_turn(q, id)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let original_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let values_1 = { let q: &dyn Query = g; original_1.map(|original: ObjectId| q.copiable_values(original).with_subtype(q.words(asker.source).subtype(Subtype::Nightmare))) }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_1 = match values_1 { Some(values) => g.create_token_copies_tapped(values, player_1, count_1, false)?, None => Vec::new() };
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!((created_1.clone()).contains(&id)) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Nightmare)) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Abyssal Harvester", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon,Subtype::Warlock]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{T}: Exile target creature card from a graveyard that was put there this turn. Create a token that's a copy of it, except it's a Nightmare in addition to its other types. Then exile all other Nightmare tokens you control.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
