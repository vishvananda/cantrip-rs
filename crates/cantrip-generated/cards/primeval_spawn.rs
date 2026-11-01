// Primeval Spawn
// status: planned
// requires: as-enters replacement of another event
// requires: object filter ManaSpent
// requires: operation PlayWith
// If this creature would enter and it wasn't cast or no mana was spent to cast it, exile it instead.
// Vigilance, trample, lifelink
// When this creature leaves the battlefield, exile the top ten cards of your library. You may cast any number of spells with total mana value 10 or less from among them without paying their mana costs.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (!(cast_of(q, candidate).is_some_and(|cast| true)) || planned_value!("object filter ManaSpent", false => (q.history().mana_spent(candidate, None, |_: ObjectId| true, |_: PlayerId| true)) == (0i64)))) } {
entering.instead = Some(Zone::exile(g.obj(entering.object).owner));
}
 Ok(()) }
#[allow(unused_variables)] fn ability_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((10i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
planned!("operation-play-with" => cantrip_mechanics::play::with_terms(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, CardPlay::CastSpell, { let q: &dyn Query = g; (moved_1.clone()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>() }, { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }, Some("GroupFilter::TotalManaValueIs(\"Comparison(LessOrEqual, Constant(10))\")"), "{\"limit\":null,\"during\":null,\"cost\":\"Free\",\"spend_as\":null,\"into_exile\":false,\"condition\":null,\"effects\":[],\"flash\":false,\"may_look\":false,\"then\":null,\"face_down\":null}")?);
g.end_sentence();
 Ok(()) }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }

pub static CARD: CardDef = CardDef { name: "Primeval Spawn", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [1, 1, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(10), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance,Keyword::Trample,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_4_event), ability_4, "When this creature leaves the battlefield, exile the top ten cards of your library. You may cast any number of spells with total mana value 10 or less from among them without paying their mana costs.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
