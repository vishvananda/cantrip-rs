// Tasha's Hideous Laughter
// status: planned
// requires: operation RevealUntilGroup
// Each opponent exiles cards from the top of their library until that player has exiled cards with total mana value 20 or greater.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { planned!("operation-reveal-until-group" => let revealed = cantrip_mechanics::library::reveal_until_group(g, { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }, asker.source, &[SelectionRule::Total(Property::ManaValue, Comparison::GreaterOrEqual, { let q: &dyn Query = g; 20i64 })])?);
    } 
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_0) }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Tasha's Hideous Laughter", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
