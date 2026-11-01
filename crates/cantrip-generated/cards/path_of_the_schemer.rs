// Path of the Schemer
// status: planned
// requires: condition MostVotes
// requires: engine: no planar deck
// requires: keyword action ChaosEnsues
// requires: operation Vote
// Each player mills two cards. Then you put a creature card from a graveyard onto the battlefield under your control. It's an artifact in addition to its other types.
// Will of the Planeswalkers — Starting with you, each player votes for planeswalk or chaos. If planeswalk gets more votes, planeswalk. If chaos gets more votes or the vote is tied, chaos ensues.
#[allow(unused_imports)] use cantrip_api::*;
static ENTERS_2: EntersAs = EntersAs { adds_types: TypeSet::of(&[CardType::Artifact]), ..EntersAs::NONE };
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::PutOntoBattlefield, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_2 = ({ let q: &dyn Query = g; selected_1.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_2 = { let rows: Vec<(ObjectId, PlayerId)> = objects_2.iter().map(|&o| (o, player_2)).collect(); let counters: Vec<(CounterKind, i16)> = vec![]; g.put_all_onto_battlefield_becoming(&rows, item, &ENTERS_2, &counters)? };
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_3 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_3.iter().position(|&p| Some(p) == first_3) { players_3.rotate_left(at); }
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let voters_4: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
planned!("operation-vote" => cantrip_mechanics::vote::ballot(g, asker.source, &voters_4, cantrip_mechanics::vote::Ballot::Words(&["Planeswalk", "Chaos"]), false)?);
    } 
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition MostVotes", false => g.votes().most(option)) } {
planned!("engine-no-planar-deck" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.planeswalk(player); });
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition MostVotes", false => g.votes().most(option)) } {
planned!("keyword-action-chaos-ensues" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::planechase::chaos_ensues(g, player)?; });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Path of the Schemer", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
