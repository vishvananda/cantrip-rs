// Glittering Wish
// status: ready
// You may reveal a multicolored card you own from outside the game and put it into your hand. Exile Glittering Wish.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let wished_1 = { let q: &dyn Query = g; q.outside_the_game(player_1).into_iter().filter(|&id| (q.chars(id).colors.iter().count() as i64) > (1i64)).collect::<Vec<ObjectId>>() }; let chosen_1 = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::PutIntoHand, wished_1, 1, true)?; let brought_1 = g.bring_into_hand_revealed(&chosen_1);
g.end_sentence();
hoisted_2.extend(brought_1.clone());
hoisted_3.extend({ let q: &dyn Query = g; chosen_1.clone() });
}
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Glittering Wish", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
