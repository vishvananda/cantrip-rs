// Cut the Tethers
// status: planned
// requires: object collection Unpaid
// For each Spirit, return it to its owner's hand unless that player pays {3}.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for bound_0 in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Spirit))).collect::<Vec<ObjectId>>() } {
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>() };
let mut each_2 = vec![Default::default(); players_0.len()];
 for (index_0, bound_1) in players_0.iter().copied().enumerate() { let payer_1 = { let q: &dyn Query = g; Some(bound_1).expect("a reference that always names a player") }; let paid_1 = g.may_pay(payer_1, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
 each_2[index_0] = paid_1;
   } 
g.end_sentence();
}
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; planned_value!("object collection Unpaid", Vec::<ObjectId>::new() => unpaid_0.clone()) }; let moved_3 = g.return_to_hand(&objects_3);
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Cut the Tethers", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
