// The Last Agni Kai
// status: planned
// requires: player rule for a while RetainMana
// Target creature you control fights target creature an opponent controls. If the creature the opponent controls is dealt excess damage this way, add that much {R}.
// Until end of turn, you don't lose unspent red mana as steps and phases end.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
#[allow(unused_variables)] fn ability_0_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0),target(ability_0_target_1)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let first_0 = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied(); let second_0 = ({ let q: &dyn Query = g; (t_1_raw.get(0).copied().filter(|t| t_1.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied();
let mut excess_0: Vec<(Target, u32)> = Vec::new(); let recipients_0: Vec<Target> = second_0.map(Target::Object).into_iter().collect(); let since_0 = g.history().head() as usize; if let (Some(first), Some(second)) = (first_0, second_0) { g.fight(first, second); excess_0.extend(g.history().all()[since_0..].iter().filter_map(|r| match r.event { Event::Damage { source: s, target, recipient_excess, .. } if s == first || s == second => Some((target, recipient_excess)), _ => None })); }
let happened_1: bool = ({ let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }).is_some();
g.end_sentence();
if { let q: &dyn Query = g; t_1_raw.get(0).copied().and_then(|t: Target| t.object()).is_some_and(|candidate: ObjectId| excess_0.iter().any(|&(t, n)| t == Target::Object(candidate) && n > 0)) } {
let mana_players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_3_0 = { let q: &dyn Query = g; (excess_0.iter().filter(|(t, _)| recipients_0.contains(t)).map(|(_, n)| i64::from(*n)).sum::<i64>()) as u32 };
for player in mana_players_3 {
g.add_mana(player, Mana::R, mana_3_0);}
g.end_sentence();
}
g.end_sentence();
planned!("player-rule-for-a-while-retain-mana" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::RetainMana(ColorSet(8))), Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Last Agni Kai", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
