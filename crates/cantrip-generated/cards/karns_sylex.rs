// Karn's Sylex
// status: ready
// Karn's Sylex enters tapped.
// Players can't pay life to cast spells or to activate abilities that aren't mana abilities.
// {X}, {T}, Exile Karn's Sylex: Destroy each nonland permanent with mana value X or less. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, costing: CostFor, payment: CostPayment) -> bool { true && matches!(costing, CostFor::Spell) && matches!(payment, CostPayment::Life) }
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, player: PlayerId, costing: CostFor, payment: CostPayment) -> bool { true && matches!(costing, CostFor::Ability) && matches!(payment, CostPayment::Life) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && (i64::from(q.mana_value(id))) <= (i64::from(item.x)))).collect::<Vec<ObjectId>>() }; let destroyed_1 = g.destroy_all(&objects_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Karn's Sylex", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(EXILE_THIS), ability_2, "{X}, {T}, Exile Karn's Sylex: Destroy each nonland permanent with mana value X or less. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, cost_bans: &[ability_1_0_rule,ability_1_1_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
