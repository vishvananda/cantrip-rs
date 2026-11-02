// Berserker's Frenzy
// status: planned
// requires: keyword action RollKeepHighest(20) with an amount
// requires: player rule for a while Decides
// Cast this spell only before combat or during combat before blockers are declared.
// Roll two d20 and ignore the lower roll.
// 1—14 | Choose any number of creatures. They block this turn if able.
// 15—20 | You choose which creatures block this turn and how those creatures block.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { ((q.state().turn.step() as u8) < (Step::BeginCombat as u8) || (((Step::BeginCombat as u8)..=(Step::EndOfCombat as u8)).contains(&(q.state().turn.step() as u8)) && (q.state().turn.step() as u8) < (Step::DeclareBlockers as u8))) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
planned!("keyword-action-roll-keep-highest-20-with-an-amount" => let result_0 = { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let amount = { let q: &dyn Query = g; (2i64).max(0) as u32 }; cantrip_mechanics::dice::roll_keep_highest(g, player, 20, amount)? };);
g.end_sentence();
if { let q: &dyn Query = g; ((planned_value!("depends on an instruction with a gap", 0i64 => result_0)) >= (1i64) && (planned_value!("depends on an instruction with a gap", 0i64 => result_0)) <= (14i64)) } {
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::ChooseAsEnters, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; (selected_1.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::MustBlock };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
hoisted_3.extend(selected_1.clone());
hoisted_4.extend(objects_2.clone());
}
if { let q: &dyn Query = g; ((planned_value!("depends on an instruction with a gap", 0i64 => result_0)) >= (15i64) && (planned_value!("depends on an instruction with a gap", 0i64 => result_0)) <= (20i64)) } {
planned!("player-rule-for-a-while-decides" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Decides(Decision::HowCreaturesBlock(|q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Creature)))), Duration::EndOfTurn); } });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Berserker's Frenzy", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_only_if: Some(ability_0_if), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
