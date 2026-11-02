// Rhox Faithmender
// status: ready
// Lifelink (Damage dealt by this creature also causes you to gain that much life.)
// If you would gain life, you gain twice that much life instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::LifeChange(l) = proposal { let amount = l.amount.unsigned_abs(); let subject = Target::Player(l.player); let data = [amount, u32::MAX]; if l.amount > 0 && Some(asker.controller) == Some(l.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::LifeChange(seen) = &seen { let l = seen; let amount = l.amount.unsigned_abs(); let subject = Target::Player(l.player); let data = [amount, u32::MAX]; if let Proposal::LifeChange(l) = proposal { l.amount = l.amount.signum() * ((2i64 * i64::from(data[0]))).min(i32::MAX.into()) as i32; } } }

pub static CARD: CardDef = CardDef { name: "Rhox Faithmender", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rhino,Subtype::Monk]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::LifeChange, ability_1_applies, ability_1_apply, "If you would gain life, you gain twice that much life instead.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
