// Mirri
// status: planned
// requires: card type Vanguard
// requires: event change ReplaceMana
// requires: layout Vanguard
// If a basic land you control is tapped for mana, it produces mana of a color of your choice instead of any other type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ManaProduction(m) = proposal { let id = m.permanent; let amount = m.total(); let subject = Target::Object(id); let data = [amount, u32::MAX]; if (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id))) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ManaProduction(seen) = &seen { let m = seen; let id = m.permanent; let amount = m.total(); let subject = Target::Object(id); let data = [amount, u32::MAX]; if let Proposal::ManaProduction(m) = proposal { planned_value!("event change ReplaceMana", () => m.instead_of_choice(&[Mana::W, Mana::U, Mana::B, Mana::R, Mana::G])); } } }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 0, life: 5 }));

pub static CARD: CardDef = CardDef { name: "Mirri", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::ManaProduction, ability_0_applies, ability_0_apply, "If a basic land you control is tapped for mana, it produces mana of a color of your choice instead of any other type.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
