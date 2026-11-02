// Curse of Chaos
// status: ready
// Enchant player
// Whenever a player attacks enchanted player with one or more creatures, that attacking player may discard a card. If the player does, they draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if q.chars(id).is(CardType::Creature) && true && match *defender { Target::Player(p) => Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_player) == Some(p), _ => false } { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(PlayerId(item.data[0] as u8)).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(PlayerId(item.data[0] as u8)).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_2 = Vec::new(); for player in players_2 { cards_2.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_2, true)? }); } let discarded_2 = g.discard_together(&cards_2);
let paid_1 = !discarded_2.clone().is_empty();
(paid_1, { let q: &dyn Query = g; (discarded_2).clone() }, { let q: &dyn Query = g; (discarded_2.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(PlayerId(item.data[0] as u8)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Curse of Chaos", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura,Subtype::Curse]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever a player attacks enchanted player with one or more creatures, that attacking player may discard a card. If the player does, they draw a card.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
