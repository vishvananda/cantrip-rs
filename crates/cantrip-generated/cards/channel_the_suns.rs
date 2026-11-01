// Channel the Suns
// status: ready
// Add {W}{U}{B}{R}{G}.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mana_1_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mana_1_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mana_1_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mana_1_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_1 {
g.add_mana(player, Mana::W, mana_1_0);g.add_mana(player, Mana::U, mana_1_1);g.add_mana(player, Mana::B, mana_1_2);g.add_mana(player, Mana::R, mana_1_3);g.add_mana(player, Mana::G, mana_1_4);}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Channel the Suns", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
