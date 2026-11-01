// Ichor Aberration
// status: planned
// requires: engine: no perpetual effects
// requires: trigger event Player Proliferates
// Flying, Defender
// As long as this creature's power is 7 or greater, it can attack as though it didn't have defender.
// Whenever you proliferate, if Ichor Aberration is in your graveyard or on the battlefield, it perpetually gets +1/+1. You may cast it from your graveyard this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| q.chars(candidate).power.is_some_and(|v| (i64::from(v)) >= (7i64)))) && (source == affected) }
#[allow(unused_variables)] fn ability_2_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (Some(asker.source).is_some_and(|candidate: ObjectId| q.chars(candidate).power.is_some_and(|v| (i64::from(v)) >= (7i64)))) && id == asker.source }
fn ability_3_if(q: &dyn Query, asker: &Asker) -> bool { (Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().obj(candidate).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(candidate).owner))) || Some(asker.source).is_some_and(|candidate: ObjectId| ((q.state().obj(candidate).became.is_none() && q.state().obj(candidate).zone.kind() == ZoneKind::Battlefield)))) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().obj(candidate).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(candidate).owner))) } {
planned!("engine-no-perpetual-effects" => let objects_1: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let kind_1_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) };
for &object in &objects_1 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_1_0); });
g.end_sentence();
let cards_2_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_2_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_2_0 { if g.obj(card).zone.is(ZoneKind::Graveyard) { for &player in &players_2_0 { g.allow_play(card, player, PlayGrant { lands: false, expiry: Expiry::EndOfTurn, ..PlayGrant::CAST }); } } } });
g.end_sentence();
} else {
planned!("engine-no-perpetual-effects" => let objects_4: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let kind_4_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) };
for &object in &objects_4 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_4_0); });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-proliferates" => matcher: #[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Proliferated { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Ichor Aberration", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Defender]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player Proliferates", EventMask(0) => EventMask(EventMask::PROLIFERATED.0)), ability_3_event), ability_3, "Whenever you proliferate, if Ichor Aberration is in your graveyard or on the battlefield, it perpetually gets +1/+1. You may cast it from your graveyard this turn.").targets(&[]).condition(ability_3_if).functions_in(ZoneKinds::of(&[ZoneKind::Graveyard, ZoneKind::Battlefield]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, attacks_despite_defender: &[ability_2_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
