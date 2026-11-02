// Karlach, Tiefling Spellrager
// status: planned
// requires: engine: no perpetual effects
// requires: engine: no specialize event
// First strike, Haste
// When this card specializes from your graveyard, return it from your graveyard to the battlefield. It perpetually gains "This creature can't block."
// When this card specializes from any zone, seek an instant or sorcery card with mana value 3 or less. Until end of turn, you may cast that card without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
static PERPETUAL_3: CardDef = CardDef { keywords: Keywords::of(&[Keyword::CantBlock]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, ..CardDef::BLANK };
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_0 = ({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_0 = { let rows_0: Vec<(ObjectId, PlayerId, bool)> = objects_0.iter().map(|&o| (o, player_0, false)).collect(); let came_0 = g.put_all_onto_battlefield_as(&rows_0)?; came_0.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_2 { let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_3) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_4 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (i64::from(q.mana_value(id))) <= (3i64))).collect::<Vec<ObjectId>>() }; let count_4 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_4: Vec<ObjectId> = Vec::new(); while picked_4.len() < count_4 && !pool_4.is_empty() { let at = g.choose_at_random(pool_4.len()); picked_4.push(pool_4.remove(at)); }
let objects_5 = { let q: &dyn Query = g; picked_4.clone() }; let moved_5 = g.return_to_hand(&objects_5);
g.end_sentence();
cantrip_api::instruction(g, |g| { let cards_6_0 = { let q: &dyn Query = g; (moved_5.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let players_6_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_6_0 { if g.obj(card).zone.is(ZoneKind::Hand) { for &player in &players_6_0 { g.allow_play(card, player, PlayGrant { lands: false, expiry: Expiry::EndOfTurn, instead: Some(ManaCost::ZERO), ..PlayGrant::CAST }); } } } });
 });
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-specialize-event" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Specialized { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("engine-no-specialize-event" => matcher: #[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Specialized { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Karlach, Tiefling Spellrager", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Tiefling,Subtype::Barbarian]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("engine: no specialize event", EventMask(0) => EventMask(EventMask::SPECIALIZED.0)), ability_2_event), ability_2, "When this card specializes from your graveyard, return it from your graveyard to the battlefield. It perpetually gains \"This creature can't block.\"").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Graveyard])),triggered(when_asked(planned_value!("engine: no specialize event", EventMask(0) => EventMask(EventMask::SPECIALIZED.0)), ability_3_event), ability_3, "When this card specializes from any zone, seek an instant or sorcery card with mana value 3 or less. Until end of turn, you may cast that card without paying its mana cost.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Battlefield, ZoneKind::Graveyard, ZoneKind::Hand, ZoneKind::Library, ZoneKind::Stack, ZoneKind::Exile]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
