// Pathrazer of Ulamog
// status: ready
// Annihilator 3 (Whenever this creature attacks, defending player sacrifices three permanents of their choice.)
// This creature can't be blocked except by three or more creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(attacker == asker.source && !(q.chars(blocker).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::CantBeBlockedExceptByThreeOrMore); }

pub static CARD: CardDef = CardDef { name: "Pathrazer of Ulamog", token_words: &[], oracle: true, cost: ManaCost { generic: 11, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(9), toughness: Some(9), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::annihilator::<3>("Annihilator")], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_1_apply_1, "This creature can't be blocked except by three or more creatures.")], replacements: &[], preventions: &[], block_rules: &[ability_1_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
