// Harmonized Crescendo
// status: ready
// Convoke (Your creatures can help cast this spell. Each creature you tap while casting this spell pays for {1} or one mana of that creature's color.)
// Choose a creature type. Draw a card for each permanent you control of that type.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let chosen_0 = cantrip_mechanics::chosen_type::creature_type_now(g, player_0, asker.source)?;
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (Some(asker.controller) == Some(q.controller(id)) && Some(chosen_0).is_some_and(|chosen| q.chars(id).has_subtype(chosen)))).collect::<Vec<ObjectId>>()).len() as i64) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Harmonized Crescendo", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, pay_with: PayWith { convoke: true, ..PayWith::NONE }, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
