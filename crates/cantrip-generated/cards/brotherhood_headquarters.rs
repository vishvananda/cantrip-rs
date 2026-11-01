// Brotherhood Headquarters
// status: planned
// requires: mana spending predicate
// requires: object filter HasKeywordFamily
// {T}: Add {C}.
// {T}: Add one mana of any color. Spend this mana only to cast an Assassin spell or a spell that has freerunning, or to activate an ability of an Assassin source.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
g.add_mana(player, Mana::C, mana_0_0);}
 Ok(()) }
#[allow(dead_code, unused_variables)] fn mana_use_354(q: &dyn Query, asker: &Asker, spending: &Spending) -> bool { (spending.purpose == SpendingFor::Casting && spending.source.is_some_and(|spell| (q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Assassin)) || planned_value!("object filter HasKeywordFamily", false => q.has_keyword_ability(spell, KeywordAbility::Freerunning))))) || (spending.purpose == SpendingFor::Activating && spending.source.is_some_and(|source| q.chars(source).has_subtype(q.words(asker.source).subtype(Subtype::Assassin)))) }

pub static CARD: CardDef = CardDef { name: "Brotherhood Headquarters", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_0_mana) }, "{T}: Add {C}."),mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_any_color(1).only_for(planned_value!("mana spending predicate", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: mana_use_354, spent: None }))), "{T}: Add one mana of any color. Spend this mana only to cast an Assassin spell or a spell that has freerunning, or to activate an ability of an Assassin source.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
