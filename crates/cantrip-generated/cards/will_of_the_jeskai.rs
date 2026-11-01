// Will of the Jeskai
// status: planned
// requires: engine: no commander
// Choose one. If you control a commander as you cast this spell, you may choose both instead.
// • Each player may discard their hand and draw five cards.
// • Each instant and sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut guard_0_0 = vec![false; players_0.len()];
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_0_0[index_0] = true;
let amount_2 = { let q: &dyn Query = g; ((((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).max(0) as u32 }; gathered_1.extend(cantrip_mechanics::choose_from_hand(g, bound_0, asker.source, amount_2)?);
   }
 } 
let cards_3 = { let q: &dyn Query = g; gathered_1.clone() }; let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if guard_0_0[index_0] {
let amount_4 = { let q: &dyn Query = g; (5i64).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
   }
 } 
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let cards_5: Vec<ObjectId> = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>() }; for card in cards_5 { let owner = g.obj(card).owner; g.allow_play(card, owner, PlayGrant { expiry: Expiry::EndOfTurn, flashback: true, ..PlayGrant::CAST }); }
g.end_sentence();
 Ok(()) }
fn ability_0_more_modes(q: &dyn Query, asker: &Asker) -> bool { q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && planned_value!("engine: no commander", false => q.is_commander(id))).count() >= 1 }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Each player may discard their hand and draw five cards.").targets(&[]),mode(ability_0_mode_1, "Each instant and sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost.").targets(&[])]).choose(1, 1).up_to_if(ability_0_more_modes, 2);

pub static CARD: CardDef = CardDef { name: "Will of the Jeskai", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
