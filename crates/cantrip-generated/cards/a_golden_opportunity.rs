// A Golden Opportunity
// status: planned
// requires: operation Conjure
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — Conjure a card named Gilded Goose onto the battlefield.
// II, III — You may tap an untapped Bird you control and sacrifice an artifact. If you do, conjure a card named Golden Egg onto the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_0: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_0: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_0.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Gilded Goose"), count_0, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn paid_2(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(q.state().obj(id).tapped) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Bird)) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) }
#[allow(unused_variables)] fn paid_3(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Artifact) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_1 = g.may_pay(payer_1, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(tap_untapped(1, paid_2)).and(sacrifice(1, paid_3)), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_1 as i64) != 0) } {
let count_4: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_4: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_4.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Golden Egg"), count_4, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
hoisted_5.extend(conjured_4.clone());
hoisted_6.extend(conjured_4.clone());
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn paid_8(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(q.state().obj(id).tapped) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Bird)) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) }
#[allow(unused_variables)] fn paid_9(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Artifact) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_7 = g.may_pay(payer_7, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(tap_untapped(1, paid_8)).and(sacrifice(1, paid_9)), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_7 as i64) != 0) } {
let count_10: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_10: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_10.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Golden Egg"), count_10, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
hoisted_11.extend(conjured_10.clone());
hoisted_12.extend(conjured_10.clone());
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "A Golden Opportunity", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Conjure a card named Gilded Goose onto the battlefield.").targets(&[]),chapter(2, ability_0_1, "II, III — You may tap an untapped Bird you control and sacrifice an artifact. If you do, conjure a card named Golden Egg onto the battlefield.").targets(&[]),chapter(3, ability_0_2, "II, III — You may tap an untapped Bird you control and sacrifice an artifact. If you do, conjure a card named Golden Egg onto the battlefield.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
