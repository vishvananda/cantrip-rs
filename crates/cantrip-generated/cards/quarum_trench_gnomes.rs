// Quarum Trench Gnomes
// status: ready
// {T}: If target Plains is tapped for mana, it produces colorless mana instead of white mana. (This effect lasts indefinitely.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Plains)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn produces_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { matches!(proposal, Proposal::ManaProduction(m) if m.permanent == affected && m.mana.get(Mana::W) > 0) }
#[allow(unused_variables)] fn produces_1_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { if let Proposal::ManaProduction(m) = proposal { let n = std::mem::take(&mut m.mana.0[Mana::W as usize]); m.mana.0[Mana::C as usize] += n; } }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if let Some(object) = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) } { g.add_effect(item.object, object, EffectKind::Replacement { card: item.ability.card, index: 0 }, Duration::Indefinite); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Quarum Trench Gnomes", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Gnome]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{T}: If target Plains is tapped for mana, it produces colorless mana instead of white mana.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::ManaProduction, produces_1_applies, produces_1_apply, "produces another type of mana instead").text("If target Plains is tapped for mana, it produces colorless mana instead of white mana.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
