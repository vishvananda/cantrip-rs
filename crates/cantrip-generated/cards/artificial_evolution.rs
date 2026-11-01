// Artificial Evolution
// status: planned
// requires: continuous change ReplaceWord
// requires: modes implied by the target chosen
// requires: operation ChooseValueWith
// Change the text of target spell or permanent by replacing all instances of one creature type with another. The new creature type can't be Wall. (This effect lasts indefinitely.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (true)) }
static ABILITY_0_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let chosen_0 = planned_value!("operation ChooseValueWith", Subtype::Plains => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CreatureType, ChoiceTerms { options: &[], except: &[CreatureType::Wall], count: Some({ let q: &dyn Query = g; (2i64).max(0) as u32 }), secret: false, unchosen: false, random: false, ordered: false, up_to: false })?);
g.end_sentence();
planned!("continuous-change-replace-word", ChangeSpell, "changes" = "[{\"args\":{\"from\":{\"args\":{\"family\":\"ColorWord\",\"form\":\"TheFirstChosenColorWord\"},\"kind\":\"Vocabulary\"},\"to\":{\"args\":{\"family\":\"ColorWord\",\"form\":\"TheSecondChosenColorWord\"},\"kind\":\"Vocabulary\"}},\"kind\":\"ReplaceWord\"}]", "subject" = "{\"One\":{\"args\":{\"index\":0,\"slot\":0},\"kind\":\"Target\"}}");
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
static ABILITY_0_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_1_target_0)];
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let chosen_2 = planned_value!("operation ChooseValueWith", Subtype::Plains => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CreatureType, ChoiceTerms { options: &[], except: &[CreatureType::Wall], count: Some({ let q: &dyn Query = g; (2i64).max(0) as u32 }), secret: false, unchosen: false, random: false, ordered: false, up_to: false })?);
g.end_sentence();
planned!("continuous-change-replace-word", CreateContinuousEffect, "affected" = "{\"Objects\":{\"args\":{\"args\":{\"index\":0,\"slot\":0},\"kind\":\"Target\"},\"kind\":\"One\"}}", "changes" = "[{\"args\":{\"from\":{\"args\":{\"family\":\"ColorWord\",\"form\":\"TheFirstChosenColorWord\"},\"kind\":\"Vocabulary\"},\"to\":{\"args\":{\"family\":\"ColorWord\",\"form\":\"TheSecondChosenColorWord\"},\"kind\":\"Vocabulary\"}},\"kind\":\"ReplaceWord\"}]", "duration" = "\"Indefinite\"", "evaluation" = "\"Snapshot\"", "source" = { let q: &dyn Query = g; Some(asker.source) });
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = planned_value!("modes implied by the target chosen", modes(&[mode(ability_0_mode_0, " ").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, " Change characteristics until end of turn.").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1) => modes(&[mode(ability_0_mode_0, " ").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, " Change characteristics until end of turn.").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1).implied_by_target());
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Artificial Evolution", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
