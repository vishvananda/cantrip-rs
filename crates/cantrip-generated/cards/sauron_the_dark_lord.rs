// Sauron, the Dark Lord
// status: planned
// requires: engine: no Ring temptation
// requires: trigger event Player RingTempts
// Ward—Sacrifice a legendary artifact or legendary creature.
// Whenever an opponent casts a spell, amass Orcs 1.
// Whenever an Army you control deals combat damage to a player, the Ring tempts you.
// Whenever the Ring tempts you, you may discard your hand. If you do, draw four cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn paid_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { ((q.chars(id).supertypes.legendary && q.chars(id).is(CardType::Artifact)) || (q.chars(id).supertypes.legendary && q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_0_ward_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); let Target::Object(by) = item.subject else { return Ok(()); }; let payer = PlayerId(item.data[0] as u8); if !g.may_pay(payer, item.object, by, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(1, paid_0)), &cantrip_mechanics::COUNTERED_UNLESS_PAID)? { g.counter_item(by, item.data[1] as usize); } Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&*player) && true { return Some(Fired { subject, data }); } }
 None }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Orc Army).name, token_words: token!(Orc Army).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Orc,Subtype::Army]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let army_2: Vec<ObjectId> = cantrip_mechanics::amass::amass_army(g, player_2, asker.source, &token_3::CARD, Subtype::Orc, amount_2)?.into_iter().collect();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).has_subtype(q.words(asker.source).subtype(Subtype::Army)) && Some(asker.controller) == Some(q.controller(source))) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-ring-temptation" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::ring::tempt(g, player, asker.source)?; });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_5 = cantrip_mechanics::effects::ask_may(g, payer_5, asker.source, Purpose::Optional)?; if paid_5 {
let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_6 = Vec::new(); for player in players_6 { cards_6.extend(g.state().zone(Zone::hand(player)).to_vec()); } let discarded_6 = g.discard_together(&cards_6);
g.end_sentence();
hoisted_7.extend(discarded_6.clone());
hoisted_8.extend({ let q: &dyn Query = g; discarded_6.clone() });
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_5 as i64) != 0) } {
let amount_9 = { let q: &dyn Query = g; (4i64).max(0) as u32 }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_9 { g.draw(player, amount_9); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-ring-tempts" => matcher: #[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::RingTempted { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Sauron, the Dark Lord", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Avatar,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when(EventMask::BECAME_TARGET, cantrip_api::intrinsic::ward_matches), ability_0_ward_1, "Ward").ward(),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Whenever an opponent casts a spell, amass Orcs 1.").targets(&[]),triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever an Army you control deals combat damage to a player, the Ring tempts you.").targets(&[]),triggered(when_asked(planned_value!("trigger event Player RingTempts", EventMask(0) => EventMask(EventMask::RING_TEMPTED.0)), ability_3_event), ability_3, "Whenever the Ring tempts you, you may discard your hand. If you do, draw four cards.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD];
