// Plagiarize
// status: planned
// requires: target reference outside a resolving ability
// Until end of turn, if target player would draw a card, instead that player skips that draw and you draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::Draw(d) = proposal { let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; if planned_value!("target reference outside a resolving ability", None::<PlayerId> => q.effect_about(asker, 0).first().and_then(|t: &Target| t.player())) == Some(d.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_1_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Draw(seen) = &seen { let d = seen; let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; if let Proposal::Draw(d) = proposal { d.replaced = true; } } }
#[allow(unused_variables, unused_mut)] fn made_1_then_steps(g: &mut dyn Game, asker: &Asker, affected: ObjectId, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
 Ok(()) }
#[allow(unused_variables)] fn made_1_then(g: &mut dyn Game, asker: &Asker, affected: ObjectId, before: &Proposal, _: &Proposal) { if let Proposal::Draw(d) = before { let q: &dyn Query = g; let subject = Target::Player(d.player); let data = [1u32, u32::MAX]; let _ = made_1_then_steps(g, asker, affected, subject, data); } }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("target-reference-outside-a-resolving-ability" => {  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); let mut carried: Vec<Target> = Vec::new(); carried.extend({ let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).map(Target::Player).into_iter().collect::<Vec<Target>>() }); carried.push(Target::None);  if let (Some(to), Some(about)) = (to, about) { let _ = (to, about); g.add_player_effect_about(&carried, item.controller, item.object, EffectKind::ReplacementRule { card: item.ability.card, index: 0 }, Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Plagiarize", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::Draw, made_1_applies, made_1_apply, " Draw 1 cards.").then(made_1_then).text("Until end of turn, if target player would draw a card, instead that player skips that draw and you draw a card.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
