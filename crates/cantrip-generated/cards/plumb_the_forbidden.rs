// Plumb the Forbidden
// status: ready
// As an additional cost to cast this spell, you may sacrifice one or more creatures. When you do, copy this spell for each creature sacrificed this way.
// You draw a card and lose 1 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) }
#[allow(unused_variables)] fn ability_0_sacrificed(q: &dyn Query, asker: &Asker) -> bool { q.obj(asker.source).paid > 0 }
fn ability_0_copies(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { for _ in 0..g.obj(item.object).paid { g.copy_spell(item.object, item.controller, false)?; } Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.lose_life(player, amount_2); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Plumb the Forbidden", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, abilities: &[triggered(cantrip_mechanics::triggers::CAST_THIS_SPELL, ability_0_copies, "When you do, copy this spell for each creature sacrificed this way.").condition(ability_0_sacrificed)], labels: &["Cast without sacrificing", "Sacrifice one or more creatures"], optional_cost: Some(OptionalCost::new(Cost { variable: Some(VariableCost::SacrificeOneOrMore(ability_0_0)), ..Cost::NONE })), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
