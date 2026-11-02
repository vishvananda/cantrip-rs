// Arcanis, the Omnipotent Avatar
// status: planned
// requires: a cost's filter reading the X announced
// requires: card type Vanguard
// requires: layout Vanguard
// {X}, Return a creature you control with mana value X to its owner's hand: Draw a number of cards chosen at random between 0 and X. X can't be 0.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let chosen_1: u32 = { let least = { let q: &dyn Query = g; (0i64).max(0) as u32 }; let most = { let q: &dyn Query = g; (i64::from(item.x)) as u32 }; least + g.choose_at_random((most.saturating_sub(least) + 1) as usize) as u32 };
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (i64::from(chosen_1)) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
 Ok(()) }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 1, life: -3 }));

pub static CARD: CardDef = CardDef { name: "Arcanis, the Omnipotent Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("a cost's filter reading the X announced", Cost::NONE => Cost::moves(&[CostPick { objects: Picks::Matching(|q: &dyn Query, asker: &Asker, id: ObjectId| ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) && (i64::from(q.mana_value(id))) == (i64::from(q.announced_x().unwrap_or(0))))), count: Count::Exactly(1) }], ZoneDestination::OwnersHand, Arrival { tapped: false, counters: &[], states: &[] }))), ability_0, "{X}, Return a creature you control with mana value X to its owner's hand: Draw a number of cards chosen at random between 0 and X. X can't be 0.").targets(&[]).x_cant_be_zero()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
