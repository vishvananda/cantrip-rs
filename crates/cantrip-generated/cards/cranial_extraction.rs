// Cranial Extraction
// status: ready
// Choose a nonland card name. Search target player's graveyard, hand, and library for all cards with that name and exile them. Then that player shuffles.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Name(Some(chosen_0)) = g.ask(player_0, RequestKind::ChooseName { source: asker.source, looked_at: Vec::new(), suggestion: None, kind: NameKind { non_types: TypeSet::of(&[CardType::Land]), ..NameKind::ANY } })? else { return Ok(()) };
g.end_sentence();
let searcher_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") };
let mut found_1: Vec<ObjectId> = Vec::new();
{ let shown = g.state().zone(Zone::graveyard(owner_1)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| Some(chosen_0).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, id, chosen))).collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_1, asker.source, Purpose::Exile, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?;  found_1.extend(taken); } }
{ let shown = g.state().zone(Zone::hand(owner_1)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| Some(chosen_0).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, id, chosen))).collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_1, asker.source, Purpose::Exile, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?;  found_1.extend(taken); } }
{ let shown = cantrip_mechanics::library::searched_cards(g, owner_1); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| Some(chosen_0).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, id, chosen))).collect() }; let taken = cantrip_mechanics::library::search(g, searcher_1, owner_1, asker.source, SearchDestination::Exile, cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?; found_1.extend(taken); }
let objects_2 = { let q: &dyn Query = g; found_1.clone() }; let moved_2 = g.exile_all(&objects_2);
for player in { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Cranial Extraction", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[Subtype::Arcane]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
