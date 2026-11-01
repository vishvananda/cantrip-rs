// Marwyn's Kindred
// status: planned
// requires: operation Conjure
// Conjure a card named Marwyn, the Nurturer and X cards named Llanowar Elves onto the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_0: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_0: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_0.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Marwyn, the Nurturer"), count_0, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
let count_1: u32 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as u32 }; let mut conjured_1: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_1.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Llanowar Elves"), count_1, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Marwyn's Kindred", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 1, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
