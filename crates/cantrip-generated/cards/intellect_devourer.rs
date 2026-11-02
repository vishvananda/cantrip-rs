// Intellect Devourer
// status: planned
// requires: play permission with mana as though any color
// Devour Intellect — When this creature enters, each opponent exiles a card from their hand until this creature leaves the battlefield.
// Body Thief — You may play lands and cast spells from among cards exiled with this creature. If you cast a spell this way, you may spend mana as though it were mana of any color to cast it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let selected_1_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Exile, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
 each_2[index_0] = selected_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; all_2.clone() }; let by_3 = g.linked(item); let moved_3 = g.exile_with(&objects_3, by_3, true);
g.end_sentence();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_allows(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> bool { player == asker.controller && q.exiled_with(q.linked_to(asker)).contains(&card) && q.chars(card).is(CardType::Land) }
planned_item!("play-permission-with-mana-as-though-any-color" => #[allow(unused_variables)] fn ability_1_1_allows(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> bool { player == asker.controller && q.exiled_with(q.linked_to(asker)).contains(&card) });
planned_item!("play-permission-with-mana-as-though-any-color" => exile_casts: ExileCast { text: "Cast from exile", allows: ability_1_1_allows, additional: Cost::NONE, any_type: false, lands: false, life_for_mana_value: false, any_color: true });

pub static CARD: CardDef = CardDef { name: "Intellect Devourer", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Devour Intellect — When this creature enters, each opponent exiles a card from their hand until this creature leaves the battlefield.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, exile_casts: &[ExileCast { text: "Play from exile", allows: ability_1_0_allows, additional: Cost::NONE, any_type: false, lands: true, life_for_mana_value: false }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
