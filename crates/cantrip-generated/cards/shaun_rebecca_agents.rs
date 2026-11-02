// Shaun & Rebecca, Agents
// status: ready
// Vigilance
// When Shaun & Rebecca enters, search your graveyard, hand, and library for a card named The Animus and put it onto the battlefield, then shuffle.
// {T}: Add {C}. When you do, mill two cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let mut left_0: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mut found_0: Vec<ObjectId> = Vec::new();
{ let shown = g.state().zone(Zone::graveyard(owner_0)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| q.chars(id).name == "The Animus").collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_0, asker.source, Purpose::PutOntoBattlefield, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::UpTo(left_0), &shown, can))?; left_0 -= taken.len() as u32; found_0.extend(taken); } }
{ let shown = g.state().zone(Zone::hand(owner_0)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| q.chars(id).name == "The Animus").collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_0, asker.source, Purpose::PutOntoBattlefield, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::UpTo(left_0), &shown, can))?; left_0 -= taken.len() as u32; found_0.extend(taken); } }
{ let shown = cantrip_mechanics::library::searched_cards(g, owner_0); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| q.chars(id).name == "The Animus").collect() }; let taken = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Battlefield, cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::UpTo(left_0), &shown, can))?; found_0.extend(taken); }
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; found_0.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, false)).collect(); let came_1 = g.put_all_onto_battlefield_as(&rows_1)?; came_1.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn reflexive_7(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_8 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let mills_8: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_8)).collect(); let milled_8 = g.mill_together(&mills_8);
g.end_sentence();
 Ok(()) }
fn ability_2_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_4_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_4 {
g.add_mana(player, Mana::C, mana_4_0);}
let happened_5: bool = true;
if happened_5 && true { g.reflexive_trigger(asker.source, asker.controller, AbilityRef::delayed({ let q: &dyn Query = g; { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) } }, 0), Target::None); }
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Shaun & Rebecca, Agents", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Assassin,Subtype::Scientist]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When Shaun & Rebecca enters, search your graveyard, hand, and library for a card named The Animus and put it onto the battlefield, then shuffle.").targets(&[]),mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_2_mana), "{T}: Add {C}. When you do, mill two cards.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_7, "When you do, mill two cards.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
