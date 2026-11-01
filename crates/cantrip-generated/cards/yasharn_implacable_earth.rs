// Yasharn, Implacable Earth
// status: planned
// requires: prohibited action PayLife
// requires: prohibited action Sacrifice
// requires: search with several find clauses
// When Yasharn enters, search your library for a basic Forest card and a basic Plains card, reveal those cards, put them into your hand, then shuffle.
// Players can't pay life or sacrifice nonland permanents to cast spells or activate abilities.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let found_0: Vec<ObjectId> = planned_value!("search with several find clauses", Vec::<ObjectId>::new() => cantrip_mechanics::library::search_clauses(g, searcher_0, owner_0, asker.source, &[(Selection::of({ let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }, &[]), |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).supertypes.basic && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Forest)))), (Selection::of({ let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }, &[]), |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).supertypes.basic && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Plains))))])?);
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_2 = { let q: &dyn Query = g; found_0.clone() }; let moved_2 = g.return_to_hand_revealed(&objects_2);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_1_1_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_1_2_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_1_3_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
planned_item!("prohibited-action-pay-life" => statics: player_rule(ability_1_0_applies, PlayerRule::Cant(Prohibition::PayLife(Payment::Spells(|q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| true)))));
planned_item!("prohibited-action-pay-life" => statics: player_rule(ability_1_1_applies, PlayerRule::Cant(Prohibition::PayLife(Payment::Abilities(|q: &dyn Query, asker: &Asker, permanent: ObjectId, def: Option<&ActivatedDef>| true)))));
planned_item!("prohibited-action-sacrifice" => statics: player_rule(ability_1_2_applies, PlayerRule::Cant(Prohibition::Sacrifice { objects: |q: &dyn Query, asker: &Asker, id: ObjectId| !(q.chars(id).is(CardType::Land)), paying: Some(Payment::Spells(|q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| true)) })));
planned_item!("prohibited-action-sacrifice" => statics: player_rule(ability_1_3_applies, PlayerRule::Cant(Prohibition::Sacrifice { objects: |q: &dyn Query, asker: &Asker, id: ObjectId| !(q.chars(id).is(CardType::Land)), paying: Some(Payment::Abilities(|q: &dyn Query, asker: &Asker, permanent: ObjectId, def: Option<&ActivatedDef>| true)) })));

pub static CARD: CardDef = CardDef { name: "Yasharn, Implacable Earth", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Boar]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Yasharn enters, search your library for a basic Forest card and a basic Plains card, reveal those cards, put them into your hand, then shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
