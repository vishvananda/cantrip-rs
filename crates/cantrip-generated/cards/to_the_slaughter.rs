// To the Slaughter
// status: ready
// Target player sacrifices a creature or planeswalker of their choice.
// Delirium — If there are four or more card types among cards in your graveyard, instead that player sacrifices a creature and a planeswalker of their choice.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let mut hoisted_1: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if { let q: &dyn Query = g; ({ let mut seen = TypeSet::NONE; for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { for kind in q.chars(id).types.iter() { seen = seen.with(kind); } } seen.iter().count() as i64 }) >= (4i64) } {
let player_0: PlayerId = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }; let mut slots_0: Vec<Vec<ObjectId>> = Vec::new();
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; slots_0.push(slot.into_iter().filter(|&o| g.controller(o) == player_0 && g.can_sacrifice(player_0, o)).collect());
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Planeswalker)).collect::<Vec<ObjectId>>() }; slots_0.push(slot.into_iter().filter(|&o| g.controller(o) == player_0 && g.can_sacrifice(player_0, o)).collect());
let mut chosen_0: Vec<ObjectId> = Vec::new(); for at in 0..slots_0.len() { let goal = most_assigned(&slots_0[at..], &mut chosen_0.clone()); let keeps: Vec<ObjectId> = slots_0[at].iter().copied().filter(|o| !chosen_0.contains(o)).filter(|&o| { let mut taken = chosen_0.clone(); taken.push(o); 1 + most_assigned(&slots_0[at + 1..], &mut taken) == goal }).collect(); let picked = match keeps.as_slice() { [] => None, [one] => Some(*one), _ => cantrip_mechanics::choose::card(g, player_0, asker.source, Purpose::Sacrifice, keeps.clone())? }; if let Some(o) = picked { chosen_0.push(o); } }
let sacrificed_0 = g.sacrifice(player_0, &chosen_0);
g.end_sentence();
hoisted_1.extend(sacrificed_0.clone());
hoisted_2.extend({ let q: &dyn Query = g; chosen_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let objects_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker))).collect::<Vec<ObjectId>>() }; let players_3 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_3 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_3 { let eligible: Vec<ObjectId> = objects_3.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_3.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_3.clone(), &[]))? }; for object in taken { picks_3.push((player, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
hoisted_4.extend(sacrificed_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn most_assigned(slots: &[Vec<ObjectId>], taken: &mut Vec<ObjectId>) -> usize {
    let Some((first, rest)) = slots.split_first() else { return 0 };
    let mut best = most_assigned(rest, taken);
    for &object in first {
        if !taken.contains(&object) {
            taken.push(object);
            best = best.max(1 + most_assigned(rest, taken));
            taken.pop();
        }
    }
    best
}

pub static CARD: CardDef = CardDef { name: "To the Slaughter", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
