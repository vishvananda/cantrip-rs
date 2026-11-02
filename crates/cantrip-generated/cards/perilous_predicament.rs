// Perilous Predicament
// status: ready
// Each opponent sacrifices an artifact creature and a nonartifact creature of their choice.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_2: PlayerId = bound_0; let mut slots_2: Vec<Vec<ObjectId>> = Vec::new();
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; slots_2.push(slot.into_iter().filter(|&o| g.controller(o) == player_2 && g.can_sacrifice(player_2, o)).collect());
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Artifact)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; slots_2.push(slot.into_iter().filter(|&o| g.controller(o) == player_2 && g.can_sacrifice(player_2, o)).collect());
let mut chosen_2: Vec<ObjectId> = Vec::new(); for at in 0..slots_2.len() { let goal = most_assigned(&slots_2[at..], &mut chosen_2.clone()); let keeps: Vec<ObjectId> = slots_2[at].iter().copied().filter(|o| !chosen_2.contains(o)).filter(|&o| { let mut taken = chosen_2.clone(); taken.push(o); 1 + most_assigned(&slots_2[at + 1..], &mut taken) == goal }).collect(); let picked = match keeps.as_slice() { [] => None, [one] => Some(*one), _ => cantrip_mechanics::choose::card(g, player_2, asker.source, Purpose::Sacrifice, keeps.clone())? }; if let Some(o) = picked { chosen_2.push(o); } }
gathered_1.extend(chosen_2.iter().map(|&o| (bound_0, o)));
    } 
let sacrificed_1 = g.sacrifice_each(&gathered_1);
g.end_sentence();
 Ok(()) }
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

pub static CARD: CardDef = CardDef { name: "Perilous Predicament", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
