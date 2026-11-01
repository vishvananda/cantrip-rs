// Kassandra, Eagle Bearer
// status: ready
// Haste
// When Kassandra enters, search your graveyard, hand, and library for a card named The Spear of Leonidas, put it onto the battlefield, then shuffle.
// Whenever a creature you control with a legendary Equipment attached to it deals combat damage to a player, draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let mut left_0: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mut found_0: Vec<ObjectId> = Vec::new();
{ let shown = g.state().zone(Zone::graveyard(owner_0)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| q.chars(id).name == "The Spear of Leonidas").collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_0, asker.source, Purpose::PutOntoBattlefield, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::UpTo(left_0), &shown, can))?; left_0 -= taken.len() as u32; found_0.extend(taken); } }
{ let shown = g.state().zone(Zone::hand(owner_0)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| q.chars(id).name == "The Spear of Leonidas").collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_0, asker.source, Purpose::PutOntoBattlefield, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::UpTo(left_0), &shown, can))?; left_0 -= taken.len() as u32; found_0.extend(taken); } }
{ let shown = cantrip_mechanics::library::searched_cards(g, owner_0); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| q.chars(id).name == "The Spear of Leonidas").collect() }; let taken = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Battlefield, cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::UpTo(left_0), &shown, can))?; found_0.extend(taken); }
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; found_0.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, false)).collect(); let came_1 = g.put_all_onto_battlefield_as(&rows_1)?; came_1.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(source)) && q.state().battlefield().iter().any(|&a| q.state().obj(a).attached_to == Some(source) && (q.chars(a).supertypes.legendary && q.chars(a).has_subtype(q.words(asker.source).subtype(Subtype::Equipment))))) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Kassandra, Eagle Bearer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Assassin,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When Kassandra enters, search your graveyard, hand, and library for a card named The Spear of Leonidas, put it onto the battlefield, then shuffle.").targets(&[]),triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever a creature you control with a legendary Equipment attached to it deals combat damage to a player, draw a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
