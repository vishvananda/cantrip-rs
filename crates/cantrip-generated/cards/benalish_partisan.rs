// Benalish Partisan
// status: planned
// requires: engine: no perpetual effects
// Lifelink
// Whenever you cycle another card, you may pay {1}{W}. If you do, return this card from your graveyard to the battlefield tapped and it perpetually gets +1/+0.
// Cycling {1}{W}
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Cycled { card, player } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && !(Some(asker.source) == Some(id)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, true)).collect(); let came_1 = g.put_all_onto_battlefield_as(&rows_1)?; came_1.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_3: Vec<ObjectId> = { let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_3 { let kind_3_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (0i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_0); });
g.end_sentence();
hoisted_4.extend(moved_1.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Benalish Partisan", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::CYCLED.0), ability_1_event), ability_1, "Whenever you cycle another card, you may pay {1}{W}. If you do, return this card from your graveyard to the battlefield tapped and it perpetually gets +1/+0.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Battlefield, ZoneKind::Graveyard, ZoneKind::Hand, ZoneKind::Library, ZoneKind::Stack, ZoneKind::Exile])),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(DISCARD_THIS), ability_2, "Cycling {1}{W}").targets(&[]).cycling().from(ZoneKind::Hand)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
