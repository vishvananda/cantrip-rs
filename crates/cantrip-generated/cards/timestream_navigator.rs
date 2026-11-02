// Timestream Navigator
// status: ready
// Ascend (If you control ten or more permanents, you get the city's blessing for the rest of the game.)
// {2}{U}{U}, {T}, Put this creature on the bottom of its owner's library: Take an extra turn after this one. Activate only if you have the city's blessing.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times = 1u32; g.add_extra_turns(player, times); }
g.end_sentence();
 Ok(()) }
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).citys_blessing) }

pub static CARD: CardDef = CardDef { name: "Timestream Navigator", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Pirate,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(put_self_into_library(LibraryPlace::Bottom)), ability_1, "{2}{U}{U}, {T}, Put this creature on the bottom of its owner's library: Take an extra turn after this one. Activate only if you have the city's blessing.").targets(&[]).condition(ability_1_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ascend: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
