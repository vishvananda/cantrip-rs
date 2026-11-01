// Susan Foreman
// status: planned
// requires: engine: no planar deck
// requires: move to OwnersPlanarDeck
// requires: planar deck
// requires: replacement of Planeswalk
// If you would planeswalk, instead look at the top two cards of your planar deck, put one on the bottom of your planar deck and the other on top, then planeswalk.
// {T}: Add {G}.
// Doctor's companion (You can have two commanders if the other is the Doctor.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_0 = { let q: &dyn Query = g; ((planned_value!("planar deck", Vec::<ObjectId>::new() => (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.planar_deck(p)).collect::<Vec<ObjectId>>())).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((2i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_0, &objects_0);
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; (objects_0.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == planned_value!("planar deck", ZoneKind::Command => ZoneKind::PlanarDeck)).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::PutOnLibrary, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let moved_2 = planned_value!("move to OwnersPlanarDeck", Vec::<ObjectId>::new() => g.put_on_planar_deck(&objects_2, SequenceEnd::Bottom));
let objects_3 = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (objects_0.clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == planned_value!("planar deck", ZoneKind::Command => ZoneKind::PlanarDeck)).collect::<Vec<ObjectId>>() }; let moved_3 = planned_value!("move to OwnersPlanarDeck", Vec::<ObjectId>::new() => g.put_on_planar_deck(&objects_3, SequenceEnd::Top));
planned!("engine-no-planar-deck" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.planeswalk(player); });
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_5_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_5 {
g.add_mana(player, Mana::G, mana_5_0);}
 Ok(()) }
planned_item!("replacement-of-planeswalk" => callbacks: #[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Planeswalk(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if Some(asker.controller) == Some(p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Planeswalk(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::Planeswalk(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn ability_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::Planeswalk(p) = before { let q: &dyn Query = g; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; let _ = ability_0_then_steps(g, asker, subject, data); } }
);
planned_item!("replacement-of-planeswalk" => replacements: replacement(ReplacementKind::Planeswalk, ability_0_applies, ability_0_apply, "If you would planeswalk, instead look at the top two cards of your planar deck, put one on the bottom of your planar deck and the other on top, then planeswalk.").then(ability_0_then));

pub static CARD: CardDef = CardDef { name: "Susan Foreman", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::TimeLord]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 1, 0]), ..add_by(ability_1_mana) }, "{T}: Add {G}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
