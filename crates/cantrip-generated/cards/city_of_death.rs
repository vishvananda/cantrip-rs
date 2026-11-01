// City of Death
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after VI.)
// I — Create a Treasure token.
// II, III, IV, V, VI — Create a token that's a copy of target non-Saga token you control.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Treasure).name, token_words: token!(Treasure).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Treasure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = Words::NONE;
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_1_target_0)];
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_2 = match original_2 { Some(original) => g.create_token_copies(original, player_2, count_2)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_2_target_0)];
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_3 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; let count_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_3 = match original_3 { Some(original) => g.create_token_copies(original, player_3, count_3)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_3_target_0)];
fn ability_0_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_4 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; let count_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_4 = match original_4 { Some(original) => g.create_token_copies(original, player_4, count_4)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_4_target_0)];
fn ability_0_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_5 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; let count_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_5 = match original_5 { Some(original) => g.create_token_copies(original, player_5, count_5)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_5_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_5_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_5_target_0)];
fn ability_0_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_6 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; let count_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_6 = match original_6 { Some(original) => g.create_token_copies(original, player_6, count_6)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "City of Death", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Create a Treasure token.").targets(&[]),chapter(2, ability_0_1, "II, III, IV, V, VI — Create a token that's a copy of target non-Saga token you control.").targets(ABILITY_0_1_TARGETS),chapter(3, ability_0_2, "II, III, IV, V, VI — Create a token that's a copy of target non-Saga token you control.").targets(ABILITY_0_2_TARGETS),chapter(4, ability_0_3, "II, III, IV, V, VI — Create a token that's a copy of target non-Saga token you control.").targets(ABILITY_0_3_TARGETS),chapter(5, ability_0_4, "II, III, IV, V, VI — Create a token that's a copy of target non-Saga token you control.").targets(ABILITY_0_4_TARGETS),chapter(6, ability_0_5, "II, III, IV, V, VI — Create a token that's a copy of target non-Saga token you control.").targets(ABILITY_0_5_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
