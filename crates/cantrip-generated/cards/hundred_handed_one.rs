// Hundred-Handed One
// status: planned
// requires: static rule BlockCapacity
// Vigilance
// {3}{W}{W}{W}: Monstrosity 3. (If this creature isn't monstrous, put three +1/+1 counters on it and it becomes monstrous.)
// As long as this creature is monstrous, it has reach and can block an additional ninety-nine creatures each combat.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as i16 }; g.monstrosity(asker.source, amount_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).marks.monstrous)) && (source == affected) }
#[allow(unused_variables)] fn ability_2_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Reach); }
#[allow(unused_variables)] fn ability_2_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).marks.monstrous)) && (source == affected) }
#[allow(unused_variables)] fn ability_2_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> Option<u32> { ((Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).marks.monstrous)) && id == asker.source).then(|| (99i64).max(0) as u32) }
planned_item!("static-rule-block-capacity" => additional_blocks: ability_2_1_0_rule);

pub static CARD: CardDef = CardDef { name: "Hundred-Handed One", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Giant]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [3, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{3}{W}{W}{W}: Monstrosity 3.").targets(&[])], statics: &[static_ability(Layer::Ability, ability_2_0_affects, ability_2_0_apply_0, "As long as this creature is monstrous, it has reach and can block an additional ninety-nine creatures each combat.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
