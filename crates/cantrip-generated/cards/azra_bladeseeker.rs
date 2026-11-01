// Azra Bladeseeker
// status: planned
// requires: condition SameTeam
// When this creature enters, each player on your team may discard a card, then each player who discarded a card this way draws a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition SameTeam", false => q.state().same_team(a, b))).collect::<Vec<PlayerId>>() };
let mut guard_0_0 = vec![false; players_0.len()];
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if true { guard_0_0[index_0] = true;
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; gathered_1.extend(cantrip_mechanics::choose_from_hand(g, bound_0, asker.source, amount_2)?);
   }
 } 
let cards_3 = { let q: &dyn Query = g; gathered_1.clone() }; let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (((discarded_3.clone()).into_iter().filter(|&o| Some(q.state().obj(o).owner) == Some(bound_0)).collect::<Vec<ObjectId>>()).len() as i64) > (0i64)).collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Azra Bladeseeker", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Azra,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, each player on your team may discard a card, then each player who discarded a card this way draws a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
