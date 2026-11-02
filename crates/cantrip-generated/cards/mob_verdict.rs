// Mob Verdict
// status: planned
// requires: numeric expression Votes
// requires: operation RevealChoices
// requires: operation Vote
// requires: player state ReceivedAVote
// Secret council — Each player secretly votes for another player, then those votes are revealed. For each vote an opponent received, Mob Verdict deals 2 damage to that player and each creature that player controls. For each vote you received, draw a card.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let voters_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
planned!("operation-vote" => cantrip_mechanics::vote::ballot(g, asker.source, &voters_1, cantrip_mechanics::vote::Ballot::Players({ let q: &dyn Query = g; { let other = Some(bound_0); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() } }), true)?);
    } 
g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Player));
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|player: PlayerId| planned_value!("player state ReceivedAVote", false => g.votes().received(player)))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_3.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut gathered_4: Vec<(ObjectId, Target, u16)> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let amount_5 = { let q: &dyn Query = g; ((2i64 * planned_value!("numeric expression Votes", 0i64 => g.votes().count(Player)))).clamp(0, 65535) as u16 }; if let Some(source) = source_5 { for t in { let q: &dyn Query = g; { let mut all: Vec<Target> = Vec::new(); for t in (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() { if !all.contains(&t) { all.push(t); } } for t in (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() { if !all.contains(&t) { all.push(t); } } all } } { gathered_4.push((source, t, amount_5)); } }
    } 
g.damage_together(&gathered_4);
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; (planned_value!("numeric expression Votes", 0i64 => g.votes().count(Player))).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mob Verdict", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
