// Auntie Ool, Cursewretch
// status: ready
// Ward—Blight 2. (To blight 2, a player puts two -1/-1 counters on a creature they control.)
// Whenever one or more -1/-1 counters are put on a creature, draw a card if you control that creature. If you don't control it, its controller loses 1 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_ward_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); let Target::Object(by) = item.subject else { return Ok(()); }; let payer = PlayerId(item.data[0] as u8); if !g.may_pay(payer, item.object, by, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(cantrip_mechanics::blight::cost(2)), &cantrip_mechanics::COUNTERED_UNLESS_PAID)? { g.counter_item(by, item.data[1] as usize); } Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::CountersPut { object, kind, amount, by, types, .. } = event { let id = *object; let subject = Target::Object(id); let data = [*amount as u32, id.0]; if types.has(CardType::Creature) && *kind == CounterKind::MinusOne { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| Some(asker.controller) == Some(q.controller(candidate))) } {
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
} else {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_2 = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.lose_life(player, amount_2); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Auntie Ool, Cursewretch", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Goblin,Subtype::Warlock]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when(EventMask::BECAME_TARGET, cantrip_api::intrinsic::ward_matches), ability_0_ward_0, "Ward").ward(),triggered(when_asked(EventMask(EventMask::COUNTERS.0), ability_1_event), ability_1, "Whenever one or more -1/-1 counters are put on a creature, draw a card if you control that creature. If you don't control it, its controller loses 1 life.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
