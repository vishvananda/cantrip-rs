// Rankle, Pitiless Trickster
// status: planned
// requires: engine: no perpetual effects
// Flying
// Rankle has lifelink and haste as long as an opponent controls no creatures.
// When Rankle enters, you may pay 1 life. When you do, each player discards a card and sacrifices a creature of their choice.
// Whenever a player discards a card, Rankle perpetually gets +1/+0.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().any(|bound_0: PlayerId| !(q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(bound_0).unwrap_or(PlayerId(u8::MAX)) && q.chars(id).is(CardType::Creature)).count() >= 1))) && (source == affected) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Lifelink);
chars.keywords = chars.keywords.with(Keyword::Haste); }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn reflexive_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_4 = Vec::new(); for player in g.state().apnap().filter(|p| players_4.contains(p)).collect::<Vec<PlayerId>>() { cards_4.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_4)?); } let discarded_4 = g.discard_together(&cards_4);
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_5 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_5 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_5 { let eligible: Vec<ObjectId> = objects_5.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_5.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_5.clone(), &[]))? }; for object in taken { picks_5.push((player, object)); } }
let sacrificed_5 = g.sacrifice_each(&picks_5);
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
'optional_0: {
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_1 = g.may_pay(payer_1, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(life(1)), &PAY_QUESTION)?; if !paid_1 { break 'optional_0; }
if paid_1 && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if true && true && !*undefined { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-perpetual-effects" => let objects_7: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_7 { let kind_7_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (0i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_7_0); });
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Rankle, Pitiless Trickster", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When Rankle enters, you may pay 1 life. When you do, each player discards a card and sacrifices a creature of their choice.").targets(&[]),triggered(when_asked(EventMask(EventMask::DISCARDED.0), ability_3_event), ability_3, "Whenever a player discards a card, Rankle perpetually gets +1/+0.").targets(&[])], statics: &[static_ability(Layer::Ability, ability_1_0_affects, ability_1_0_apply_0, "Rankle has lifelink and haste as long as an opponent controls no creatures.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_3, "When you do, each player discards a card and sacrifices a creature of their choice.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
