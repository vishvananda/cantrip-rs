// Mindleecher
// status: ready
// Mutate {4}{B} (If you cast this spell for its mutate cost, put it over or under target non-Human creature you own. They mutate into the creature on top plus all abilities from under it.)
// Flying
// Whenever this creature mutates, exile the top card of each opponent's library face down. You may look at and play those cards for as long as they remain exiled.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Mutated { permanent, .. } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().filter_map(|p: PlayerId| q.state().zone(Zone::library(p)).first().copied()).collect::<Vec<ObjectId>>() }; let by_0 = None; let moved_0 = g.exile_face_down(&objects_0, by_0);
g.end_sentence();
cantrip_api::instruction(g, |g| { let cards_1_0 = { let q: &dyn Query = g; moved_0.clone() }; let players_1_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for &player in &players_1_0 { g.look_at(player, &cards_1_0); }
let cards_1_1 = { let q: &dyn Query = g; moved_0.clone() }; let players_1_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_1_1 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_1_1 { g.allow_play(card, player, PlayGrant { lands: true, expiry: Expiry::Never, ..PlayGrant::CAST }); } } } });
 });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mindleecher", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Nightmare]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::MUTATED.0), ability_2_event), ability_2, "Whenever this creature mutates, exile the top card of each opponent's library face down. You may look at and play those cards for as long as they remain exiled.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::mutate::way(ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Mutate {4}{B}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
