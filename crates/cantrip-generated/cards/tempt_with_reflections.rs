// Tempt with Reflections
// status: ready
// Tempting offer — Choose target creature you control. Create a token that's a copy of that creature. Each opponent may create a token that's a copy of that creature. For each opponent who does, create a token that's a copy of that creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_0 = match original_0 { Some(original) => g.create_token_copies(original, player_0, count_0)?, None => Vec::new() };
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_1.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
let mut each_6 = vec![Default::default(); players_1.len()];
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let payer_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let paid_2 = cantrip_mechanics::effects::ask_may(g, payer_2, asker.source, Purpose::Optional)?; if paid_2 {
let original_3 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let count_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_3 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let created_3 = match original_3 { Some(original) => g.create_token_copies(original, player_3, count_3)?, None => Vec::new() };
g.end_sentence();
hoisted_4.extend(created_3.clone());
}
 each_5[index_1] = hoisted_4.clone(); all_5.extend(each_5[index_1].iter().copied());
each_6[index_1] = paid_2;
   } 
g.end_sentence();
let original_7 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let count_7 = { let q: &dyn Query = g; ((q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).and_then(|p: PlayerId| players_1.iter().position(|&next| next == p)).is_some_and(|at| (each_6[at] as i64) != 0)).collect::<Vec<PlayerId>>()).len() as i64) as u32 }; let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_7 = match original_7 { Some(original) => g.create_token_copies(original, player_7, count_7)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Tempt with Reflections", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
