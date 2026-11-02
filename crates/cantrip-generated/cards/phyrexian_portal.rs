// Phyrexian Portal
// status: ready
// {3}: If your library has ten or more cards in it, target opponent looks at the top ten cards of your library and separates them into two face-down piles. Exile one of those piles. Search the other pile for a card, put it into your hand, then shuffle the rest of that pile into your library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) >= (10i64) } {
let player_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }; let objects_1 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((10i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_1, &objects_1);
let separator_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }; let mut rest_2: Vec<ObjectId> = { let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let mut piles_2: Vec<Vec<ObjectId>> = Vec::new(); for _ in 1..2 { let (first, second) = cantrip_mechanics::choose::separate(g, Some(separator_2), asker.source, rest_2)?; piles_2.push(first); rest_2 = second; } piles_2.push(rest_2); let choosers_2: Vec<PlayerId> = Vec::<PlayerId>::new();
let mut rows_2: Vec<(PlayerId, Vec<Vec<ObjectId>>, Option<usize>, Vec<PlayerId>)> = vec![(separator_2, piles_2, None, choosers_2)];
let chooser_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let from_3 = None::<Vec<PlayerId>>; let mut chosen_3: Vec<ObjectId> = Vec::new(); for row in rows_2.iter_mut() { if from_3.as_ref().is_some_and(|from| !from.contains(&row.0)) { continue; } let at = usize::from(cantrip_mechanics::choose::one_of(g, chooser_3, asker.source, row.1.iter().map(|pile| Label::FaceDownPile { count: pile.len() as u16 }).collect())?); row.2 = Some(at); chosen_3.extend(row.1[at].iter().copied()); }
let moved_3 = g.exile_all(&chosen_3);
let selected_5_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_5_candidates = { let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let selected_5_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_5 = { cantrip_mechanics::choose::select(g, selected_5_player, asker.source, Purpose::PutIntoHand, selected_5_candidates, cantrip_mechanics::select::Selection::of(selected_5_count, &[]))? };
let objects_6 = { let q: &dyn Query = g; selected_5.clone() }; let moved_6 = g.return_to_hand(&objects_6);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_8.extend(moved_3.clone());
hoisted_9.extend(moved_6.clone());
hoisted_10.extend(objects_1.clone());
hoisted_11.extend(chosen_3.clone());
hoisted_12.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Phyrexian Portal", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{3}: If your library has ten or more cards in it, target opponent looks at the top ten cards of your library and separates them into two face-down piles. Exile one of those piles. Search the other pile for a card, put it into your hand, then shuffle the rest of that pile into your library.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
