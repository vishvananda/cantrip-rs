// Mana Clash
// status: ready
// You and target opponent each flip a coin. Mana Clash deals 1 damage to each player whose coin comes up tails. Repeat this process until both players' coins come up heads on the same flip.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
loop { let mut again_0 = false;
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all: Vec<PlayerId> = Vec::new(); for p in Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } for p in t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } all } };
let mut each_3 = vec![Default::default(); players_1.len()];
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let player_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let flip_2 = g.flip_coin_for_heads(player_2);
 each_3[index_1] = flip_2;
   } 
g.end_sentence();
let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_4 = { let q: &dyn Query = g; (q.state().apnap().filter(|&bound_0: &PlayerId| players_1.iter().zip(&each_3).any(|(&flipper, &heads)| Some(flipper) == Some(bound_0) && heads == false)).collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_4 { let batch: Vec<(ObjectId, Target, u16)> = recipients_4.iter().map(|t| (source, *t, amount_4)).collect(); g.damage_together(&batch); }
g.end_sentence();
if { let q: &dyn Query = g; !(((each_3.iter().filter(|&&flip| !flip).count() as i64)) == (0i64)) } {
again_0 = true;
g.end_sentence();
}
g.end_sentence();
if !again_0 { break; } }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Mana Clash", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
