// Trap the Trespassers
// status: planned
// requires: numeric expression Votes
// requires: object state ReceivedAVote
// requires: operation RevealChoices
// requires: operation Vote
// Secret council — Each player secretly votes for a creature you don't control, then those votes are revealed. For each creature with one or more votes, put that many stun counters on it, then tap it. (If a permanent with a stun counter would become untapped, remove one from it instead.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let voters_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
planned!("operation-vote" => cantrip_mechanics::vote::ballot(g, asker.source, &voters_1, cantrip_mechanics::vote::Ballot::Objects, &({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id)))).collect::<Vec<ObjectId>>() }).into_iter().map(Target::Object).collect::<Vec<Target>>(), true)?);
    } 
g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Object));
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| planned_value!("object state ReceivedAVote", false => q.votes().count(cantrip_mechanics::vote::Vote::Object(id)) > 0)).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new(); for &object in &objects_3 { let amount = { let q: &dyn Query = g; (planned_value!("numeric expression Votes", 0i64 => q.votes().count(cantrip_mechanics::vote::Vote::Object(Some(object).unwrap_or(ObjectId::NONE))))).min(i16::MAX.into()) as i16 }; counters_3.push((object, CounterKind::Stun, amount)); } g.add_counters_together(&counters_3);
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| planned_value!("object state ReceivedAVote", false => q.votes().count(cantrip_mechanics::vote::Vote::Object(id)) > 0)).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_4: Vec<ObjectId> = objects_4.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_4);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Trap the Trespassers", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
