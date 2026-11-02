// One Ring to Rule Them All
// status: planned
// requires: engine: no Ring temptation
// requires: object reference RingBearer
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — The Ring tempts you, then each player mills cards equal to your Ring-bearer's power.
// II — Destroy all nonlegendary creatures.
// III — Each opponent loses 1 life for each creature card in that player's graveyard.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-ring-temptation" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::ring::tempt(g, player, asker.source)?; });
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (planned_value!("object reference RingBearer", None::<ObjectId> => q.ring_bearer(player)).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as u32 }; let mills_1: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_1)).collect(); let milled_1 = g.mill_together(&mills_1);
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).supertypes.legendary) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let destroyed_2 = g.destroy_all(&objects_2);
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_3.sort_by_key(|p| order.iter().position(|o| o == p)); }
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let amount_4 = { let q: &dyn Query = g; ((((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(bound_0) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 2147483647) as i32 }; let players_4 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.lose_life(player, amount_4); }
    } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "One Ring to Rule Them All", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — The Ring tempts you, then each player mills cards equal to your Ring-bearer's power.").targets(&[]),chapter(2, ability_0_1, "II — Destroy all nonlegendary creatures.").targets(&[]),chapter(3, ability_0_2, "III — Each opponent loses 1 life for each creature card in that player's graveyard.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
