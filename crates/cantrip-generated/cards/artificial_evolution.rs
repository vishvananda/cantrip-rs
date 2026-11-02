// Artificial Evolution
// status: planned
// requires: continuous change ReplaceWord
// requires: operation ChooseValueWith
// Change the text of target spell or permanent by replacing all instances of one creature type with another. The new creature type can't be Wall. (This effect lasts indefinitely.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (true)) }
#[allow(unused_variables)] fn ability_0_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { ability_0_mode_0_target_0(q, asker, t) || ability_0_mode_1_target_0(q, asker, t) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let chosen_0 = planned_value!("operation ChooseValueWith", Subtype::Plains => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CreatureType, ChoiceTerms { options: &[], except: &[CreatureType::Wall], count: Some({ let q: &dyn Query = g; (2i64).max(0) as u32 }), secret: false, unchosen: false, random: false, ordered: false, up_to: false })?);
g.end_sentence();
planned!("continuous-change-replace-word", ChangeSpell, "changes" = "[{\"args\":{\"from\":{\"args\":{\"family\":\"ColorWord\",\"form\":\"TheFirstChosenColorWord\"},\"kind\":\"Vocabulary\"},\"to\":{\"args\":{\"family\":\"ColorWord\",\"form\":\"TheSecondChosenColorWord\"},\"kind\":\"Vocabulary\"}},\"kind\":\"ReplaceWord\"}]", "subject" = "{\"One\":{\"args\":{\"index\":0,\"slot\":0},\"kind\":\"Target\"}}");
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let chosen_2 = planned_value!("operation ChooseValueWith", Subtype::Plains => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CreatureType, ChoiceTerms { options: &[], except: &[CreatureType::Wall], count: Some({ let q: &dyn Query = g; (2i64).max(0) as u32 }), secret: false, unchosen: false, random: false, ordered: false, up_to: false })?);
g.end_sentence();
planned!("continuous-change-replace-word", CreateContinuousEffect, "affected" = "{\"Objects\":{\"args\":{\"args\":{\"index\":0,\"slot\":0},\"kind\":\"Target\"},\"kind\":\"One\"}}", "changes" = "[{\"args\":{\"from\":{\"args\":{\"family\":\"ColorWord\",\"form\":\"TheFirstChosenColorWord\"},\"kind\":\"Vocabulary\"},\"to\":{\"args\":{\"family\":\"ColorWord\",\"form\":\"TheSecondChosenColorWord\"},\"kind\":\"Vocabulary\"}},\"kind\":\"ReplaceWord\"}]", "duration" = "\"Indefinite\"", "evaluation" = "\"Snapshot\"", "source" = { let q: &dyn Query = g; Some(asker.source) });
g.end_sentence();
 Ok(()) }
fn ability_0_implied(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); let chosen = (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == 0).map(|i| g.state().target(item, i)).next().unwrap_or(Target::None);
if ability_0_mode_0_target_0(g, asker, chosen) { return ability_0_mode_0(g, item); }
ability_0_mode_1(g, item) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Artificial Evolution", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0_implied), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
