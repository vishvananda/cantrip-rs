// Colossus of Akros
// status: ready
// Defender, indestructible
// {10}: Monstrosity 10. (If this creature isn't monstrous, put ten +1/+1 counters on it and it becomes monstrous.)
// As long as this creature is monstrous, it has trample and can attack as though it didn't have defender.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (10i64).max(0) as i16 }; g.monstrosity(asker.source, amount_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).marks.monstrous)) && (source == affected) }
#[allow(unused_variables)] fn ability_3_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Trample); }
#[allow(unused_variables)] fn ability_3_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).marks.monstrous)) && (source == affected) }
#[allow(unused_variables)] fn ability_3_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).marks.monstrous)) && id == asker.source }

pub static CARD: CardDef = CardDef { name: "Colossus of Akros", token_words: &[], oracle: true, cost: ManaCost { generic: 8, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Golem]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(10), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender,Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 10, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{10}: Monstrosity 10.").targets(&[])], statics: &[static_ability(Layer::Ability, ability_3_0_affects, ability_3_0_apply_0, "As long as this creature is monstrous, it has trample and can attack as though it didn't have defender.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, attacks_despite_defender: &[ability_3_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
