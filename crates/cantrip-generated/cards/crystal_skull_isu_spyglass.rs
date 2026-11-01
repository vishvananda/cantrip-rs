// Crystal Skull, Isu Spyglass
// status: planned
// requires: play permission for lands from the top of a library with a filter
// You may look at the top card of your library any time.
// You may play historic lands and cast historic spells from the top of your library. (Artifacts, legendaries, and Sagas are historic.)
// {T}: Add {U}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { (q.chars(spell).is(CardType::Artifact) || q.chars(spell).supertypes.legendary || q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) }
fn ability_2_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
g.add_mana(player, Mana::U, mana_0_0);}
 Ok(()) }
planned_item!("play-permission-for-lands-from-the-top-of-a-library-with-a-filter" => play_rules: PlayRule::LibraryTopLandsIf(|q, asker, card| (q.chars(card).is(CardType::Artifact) || q.chars(card).supertypes.legendary || q.chars(card).has_subtype(q.words(asker.source).subtype(Subtype::Saga)))));

pub static CARD: CardDef = CardDef { name: "Crystal Skull, Isu Spyglass", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 1, 0, 0, 0, 0]), ..add_by(ability_2_mana) }, "{T}: Add {U}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopSpells(ability_1_1_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
