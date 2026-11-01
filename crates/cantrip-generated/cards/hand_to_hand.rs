// Hand to Hand
// status: ready
// During combat, players can't cast instant spells or activate abilities that aren't mana abilities.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !((((Step::BeginCombat as u8)..=(Step::EndOfCombat as u8)).contains(&(q.state().turn.step() as u8))) && true && q.chars(spell).is(CardType::Instant)) }
#[allow(unused_variables)] #[allow(unused_variables)] fn ability_0_0_1_rule(q: &dyn Query, asker: &Asker, object: ObjectId, ability: &ActivatedDef) -> bool { let player = q.controller(object); !((((Step::BeginCombat as u8)..=(Step::EndOfCombat as u8)).contains(&(q.state().turn.step() as u8))) && true && !(ability.is_mana_ability())) }

pub static CARD: CardDef = CardDef { name: "Hand to Hand", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, activation_rules: &[ability_0_0_1_rule], spell_rules: &[ability_0_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
