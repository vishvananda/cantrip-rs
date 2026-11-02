// Nebuchadnezzar
// status: ready
// {X}, {T}: Choose a card name. Target opponent reveals X cards at random from their hand. Then that player discards all cards with that name revealed this way. Activate only during your turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Name(Some(chosen_0)) = g.ask(player_0, RequestKind::ChooseName { source: asker.source, looked_at: Vec::new(), suggestion: None, kind: NameKind::ANY })? else { return Ok(()) };
g.end_sentence();
let player_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }; let mut pool_2: Vec<ObjectId> = { let q: &dyn Query = g; q.state().zone(Zone::hand(player_2)).iter().copied().filter(|&id| true).collect::<Vec<ObjectId>>() };
let count_2 = { let q: &dyn Query = g; (i64::from(item.x)) as usize }; let mut shown_2: Vec<ObjectId> = Vec::new(); while shown_2.len() < count_2 && !pool_2.is_empty() { let at = g.choose_at_random(pool_2.len()); shown_2.push(pool_2.remove(at)); }
g.reveal(player_2, &shown_2);
let cards_3 = { let q: &dyn Query = g; (shown_2.clone()).into_iter().filter(|&id| Some(chosen_0).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, id, chosen))).collect::<Vec<ObjectId>>() }; let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
 Ok(()) }
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.controller) == Some(q.state().turn.active) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Nebuchadnezzar", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{X}, {T}: Choose a card name. Target opponent reveals X cards at random from their hand. Then that player discards all cards with that name revealed this way. Activate only during your turn.").targets(ABILITY_0_TARGETS).condition(ability_0_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
