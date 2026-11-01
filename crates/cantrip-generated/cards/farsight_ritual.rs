// Farsight Ritual
// status: ready
// Bargain (You may sacrifice an artifact, enchantment, or token as you cast this spell.)
// Look at the top four cards of your library. If this spell was bargained, look at the top eight cards of your library instead. Put two of them into your hand and the rest on the bottom of your library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let mut hoisted_1: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).paid > 0) } {
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_0 = { let q: &dyn Query = g; (8i64).max(0) as usize };  let taken_0 = cantrip_mechanics::library::look_at_top(g, player_0, asker.source, depth_0, (2, 2), false, |_| true, cantrip_mechanics::library::Rest::BottomRandom)?;
g.end_sentence();
hoisted_1.extend(taken_0.clone());
hoisted_2.extend(taken_0.clone());
hoisted_3.extend({ let q: &dyn Query = g; taken_0.clone() });
} else {
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_4 = { let q: &dyn Query = g; (4i64).max(0) as usize };  let taken_4 = cantrip_mechanics::library::look_at_top(g, player_4, asker.source, depth_4, (2, 2), false, |_| true, cantrip_mechanics::library::Rest::BottomRandom)?;
g.end_sentence();
hoisted_5.extend(taken_4.clone());
hoisted_6.extend(taken_4.clone());
hoisted_7.extend({ let q: &dyn Query = g; taken_4.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Farsight Ritual", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: cantrip_mechanics::bargain::LABELS, optional_cost: Some(cantrip_mechanics::bargain::OPTIONAL_COST), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
