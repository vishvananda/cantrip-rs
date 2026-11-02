// Catch // Release
// status: ready
// Gain control of target permanent until end of turn. Untap it. It gains haste until end of turn.
// Fuse (You may cast one or both halves of this card from your hand.)
// Each player sacrifices an artifact, a creature, an enchantment, a land, and a planeswalker of their choice.
// Fuse (You may cast one or both halves of this card from your hand.)
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Catch // Release", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [1, 1, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, split: Some((&left::CARD, &right::CARD)), fuse: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&left::CARD,&right::CARD];
pub mod left {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(asker.controller).expect("a reference that always names a player")) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_1: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_1 { g.untap(object); } });
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Catch", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
pub mod right {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_2: PlayerId = bound_0; let mut slots_2: Vec<Vec<ObjectId>> = Vec::new();
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; slots_2.push(slot.into_iter().filter(|&o| g.controller(o) == player_2 && g.can_sacrifice(player_2, o)).collect());
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; slots_2.push(slot.into_iter().filter(|&o| g.controller(o) == player_2 && g.can_sacrifice(player_2, o)).collect());
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Enchantment)).collect::<Vec<ObjectId>>() }; slots_2.push(slot.into_iter().filter(|&o| g.controller(o) == player_2 && g.can_sacrifice(player_2, o)).collect());
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; slots_2.push(slot.into_iter().filter(|&o| g.controller(o) == player_2 && g.can_sacrifice(player_2, o)).collect());
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Planeswalker)).collect::<Vec<ObjectId>>() }; slots_2.push(slot.into_iter().filter(|&o| g.controller(o) == player_2 && g.can_sacrifice(player_2, o)).collect());
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

pub static CARD: CardDef = CardDef { name: "Release", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
