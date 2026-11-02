// The Master of Keys
// status: ready
// Flying
// When The Master of Keys enters, put X +1/+1 counters on it and mill twice X cards.
// Each enchantment card in your graveyard has escape. The escape cost is equal to the card's mana cost plus exile three other cards from your graveyard. (You may cast cards from your graveyard for their escape cost.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))).clamp(0, 32767) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::PlusOne, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; ((2i64 * item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x)))).max(0) as u32 }; let mills_1: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_1)).collect(); let milled_1 = g.mill_together(&mills_1);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_escape_cost(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> Option<Cost> { (player == asker.controller && q.chars(card).is(CardType::Enchantment) && q.card(q.chars(card).card).has_cost).then(|| cantrip_mechanics::escape::cost(q.card(q.chars(card).card).cost, 3)) }

pub static CARD: CardDef = CardDef { name: "The Master of Keys", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [1, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When The Master of Keys enters, put X +1/+1 counters on it and mill twice X cards.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_ways: &[cantrip_mechanics::escape::granted(ability_2_escape_cost)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
