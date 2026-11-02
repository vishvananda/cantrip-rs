// Chancellor of the Dross
// status: planned
// requires: a reveal from an opening hand
// You may reveal this card from your opening hand. If you do, at the beginning of the first upkeep, each opponent loses 3 life, then you gain life equal to the life lost this way.
// Flying, lifelink
#[allow(unused_imports)] use cantrip_api::*;
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_5 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut done_5: i64 = 0; for player in players_5 { let before = i64::from(g.state().player(player).life); g.lose_life(player, amount_5); done_5 += (i64::from(g.state().player(player).life) - before).abs(); }
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; ((done_5 as i64)).clamp(0, 2147483647) as i32 }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.gain_life(player, amount_6); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_0 = !({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).is_empty() && cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
let objects_1 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
hoisted_2.extend(objects_1.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
g.delayed_trigger(DelayedWhen::Step { step: Step::Upkeep, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("a-reveal-from-an-opening-hand" => abilities: cantrip_mechanics::opening_hand::reveal(ability_0, " "));

pub static CARD: CardDef = CardDef { name: "Chancellor of the Dross", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 3, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Vampire]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_4, "If you do, at the beginning of the first upkeep, each opponent loses 3 life, then you gain life equal to the life lost this way.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
