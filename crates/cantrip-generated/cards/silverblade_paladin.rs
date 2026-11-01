// Silverblade Paladin
// status: planned
// requires: engine: no Soulbond
// requires: object reference PairedWith
// requires: object state Paired
// Soulbond (You may pair this creature with another unpaired creature when either enters. They remain paired for as long as you control both of them.)
// As long as this creature is paired with another creature, both creatures have double strike.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| planned_value!("object state Paired", false => q.state().obj(candidate).paired_with.is_some()))) && ((Some(asker.source) == Some(affected) || planned_value!("object reference PairedWith", None::<ObjectId> => Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).paired_with)) == Some(affected))) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::DoubleStrike); }
planned_item!("engine-no-soulbond" => abilities: cantrip_mechanics::soulbond::TRIGGER);

pub static CARD: CardDef = CardDef { name: "Silverblade Paladin", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Knight]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_1_0_affects, ability_1_0_apply_0, "As long as this creature is paired with another creature, both creatures have double strike.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
