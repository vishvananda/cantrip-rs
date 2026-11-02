// Peacekeeper Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// requires: per-player tokens entering attached
// requires: tokens entering in a special state
// {3}: For each opponent who controls a creature, create a token that's a copy of a card named Arrest and attach it to a creature that player controls chosen at random.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(bound_0).unwrap_or(PlayerId(u8::MAX)) && q.chars(id).is(CardType::Creature)).count() >= 1).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let mut pool_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_1: Vec<ObjectId> = Vec::new(); while picked_1.len() < count_1 && !pool_1.is_empty() { let at = g.choose_at_random(pool_1.len()); picked_1.push(pool_1.remove(at)); }
 each_2[index_0] = picked_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
planned!("per-player-tokens-entering-attached" => g.begin_batch(););
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
 for (index_0, _) in players_0.iter().copied().enumerate() { let created_3: Vec<ObjectId> = planned_value!("tokens entering in a special state", Vec::<ObjectId>::new() => cantrip_mechanics::tokens::create_from_recipes(g, asker.source, &{ let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, &[({ let q: &dyn Query = g; (1i64).max(0) as u32 }, TokenRecipe::CopyNamed("Arrest", &[TokenCopyEffects::NoTokenCopyEffects]))], Arrival { tapped: false, counters: &[], states: &[ArrivalState::AttachedTo({ let q: &dyn Query = g; (each_2[index_0].clone()).get(0 as usize).copied() })] })?);
 each_4[index_0] = created_3.clone(); all_4.extend(each_4[index_0].iter().copied());
   } 
planned!("per-player-tokens-entering-attached" => g.end_batch(););
g.end_sentence();
 Ok(()) }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 0, life: 9 }));

pub static CARD: CardDef = CardDef { name: "Peacekeeper Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{3}: For each opponent who controls a creature, create a token that's a copy of a card named Arrest and attach it to a creature that player controls chosen at random.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
