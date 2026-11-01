// Charmed Pendant
// status: planned
// requires: earlier objects Milled
// requires: engine: no mill cost
// requires: mana production SymbolsOf
// {T}, Mill a card: For each colored mana symbol in the milled card's mana cost, add one mana of that color. (Activate only as an instant. For example, if the card's mana cost is {3}{U}{U}{B}, you add {U}{U}{B}.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("mana-production-symbols-of" => for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::mana::add_production(g, player, Production::SymbolsOf({ let q: &dyn Query = g; (planned_value!("earlier objects Milled", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Milled))).first().copied() }), None, None)?; });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Charmed Pendant", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(planned_value!("engine: no mill cost", Cost::NONE => Cost { mill: 1, ..Cost::NONE })), ability_0, "{T}, Mill a card: For each colored mana symbol in the milled card's mana cost, add one mana of that color.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
