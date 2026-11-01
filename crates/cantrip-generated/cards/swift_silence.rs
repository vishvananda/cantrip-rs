// Swift Silence
// status: ready
// Counter all other spells. Draw a card for each spell countered this way.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().zone(Zone::STACK).to_vec()).into_iter().filter(|&id| true && !(Some(asker.source) == Some(id))).collect::<Vec<ObjectId>>() }; cantrip_api::instruction(g, |g| { for spell in objects_0 { g.counter(spell); } });
let countered_2: Vec<ObjectId> = { let q: &dyn Query = g; (q.state().zone(Zone::STACK).to_vec()).into_iter().filter(|&id| true && !(Some(asker.source) == Some(id))).collect::<Vec<ObjectId>>() }.into_iter().filter_map(|o: ObjectId| g.state().obj(o).became).collect();
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; ((countered_2.clone()).len() as i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Swift Silence", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
