// Aeon Chronicler
// status: planned
// requires: engine: no event for a counter being removed
// requires: keyword modifier XCantBeZero
// Aeon Chronicler's power and toughness are each equal to the number of cards in your hand.
// Suspend X—{X}{3}{U}. X can't be 0.
// Whenever a time counter is removed from this card while it's exiled, draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = (((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16, ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16); chars.power = Some(p); chars.toughness = Some(t); }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
planned_item!("keyword-modifier-x-cant-be-zero" => abilities: modified(Mechanic, Modifier::XCantBeZero));
planned_item!("engine-no-event-for-a-counter-being-removed" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::CountersRemoved { object, kind, amount, by, .. } = event { let id = *object; let subject = Target::Object(id); let data = [*amount as u32, id.0]; if Some(asker.source) == Some(id) && *kind == CounterKind::Time { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Aeon Chronicler", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("engine: no event for a counter being removed", EventMask(0) => EventMask(EventMask::COUNTERS_REMOVED.0)), ability_2_event), ability_2, "Whenever a time counter is removed from this card while it's exiled, draw a card.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Exile]))], statics: &[characteristic_defining(Layer::PtDefine, ability_0_apply_0, "Aeon Chronicler's power and toughness are each equal to the number of cards in your hand.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
