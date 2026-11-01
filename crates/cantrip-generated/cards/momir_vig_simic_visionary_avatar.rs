// Momir Vig, Simic Visionary Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// requires: token recipe CopyAtRandom
// {X}, Discard a card: Create a token that's a copy of a creature card with mana value X chosen at random. Activate only as a sorcery and only once each turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let created_1: Vec<ObjectId> = planned_value!("token recipe CopyAtRandom", Vec::<ObjectId>::new() => cantrip_mechanics::tokens::create(g, asker, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, &[({ let q: &dyn Query = g; (1i64).max(0) as u32 }, TokenRecipe::CopyAtRandom(CardsInOracle::And(CardsInOracle::IsCardtype(CardType::Creature), CardsInOracle::ManaValueIs("Comparison(Equal, AnnouncedX)"))))], Arrival { tapped: false, counters: &[], states: &[] })?);
g.end_sentence();
 Ok(()) }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 0, life: 4 }));

pub static CARD: CardDef = CardDef { name: "Momir Vig, Simic Visionary Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(discard_cards(1)), ability_0, "{X}, Discard a card: Create a token that's a copy of a creature card with mana value X chosen at random. Activate only as a sorcery and only once each turn.").targets(&[]).sorcery_speed().once_each_turn()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
