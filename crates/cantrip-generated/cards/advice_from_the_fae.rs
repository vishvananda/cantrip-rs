// Advice from the Fae
// status: ready
// ({2/U} can be paid with any two mana or with {U}. This card's mana value is 6.)
// Look at the top five cards of your library. If you control more creatures than each other player, put two of those cards into your hand. Otherwise, put one of them into your hand. Then put the rest on the bottom of your library in any order.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let mut hoisted_1: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) > (({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).into_iter().map(|p: PlayerId| q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == p && q.chars(id).is(CardType::Creature)).count() as i64).max().unwrap_or(0)) } {
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_0 = { let q: &dyn Query = g; (5i64).max(0) as usize };  let taken_0 = cantrip_mechanics::library::look_at_top(g, player_0, asker.source, depth_0, (2, 2), false, |_| true, cantrip_mechanics::library::Rest::BottomChosen)?;
g.end_sentence();
hoisted_1.extend(taken_0.clone());
hoisted_2.extend(taken_0.clone());
hoisted_3.extend({ let q: &dyn Query = g; taken_0.clone() });
} else {
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_4 = { let q: &dyn Query = g; (5i64).max(0) as usize };  let taken_4 = cantrip_mechanics::library::look_at_top(g, player_4, asker.source, depth_4, (1, 1), false, |_| true, cantrip_mechanics::library::Rest::BottomChosen)?;
g.end_sentence();
hoisted_5.extend(taken_4.clone());
hoisted_6.extend(taken_4.clone());
hoisted_7.extend({ let q: &dyn Query = g; taken_4.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Advice from the Fae", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 3, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
