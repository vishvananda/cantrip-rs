// Haktos the Unscarred
// status: planned
// requires: engine: no protection from this quality
// Haktos attacks each combat if able.
// As Haktos enters, choose 2, 3, or 4 at random.
// Haktos has protection from each mana value other than the chosen number.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.must_attack += 1; }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let chosen_1: u32 = { let pool = [2u32, 3u32, 4u32]; pool[g.choose_at_random(pool.len())] };
entering.chosen_number = Some(chosen_1);
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); planned_value!("engine: no protection from this quality", () => chars.protection_from.push(ProtectionQuality::Matching(|q, asker, id| (i64::from(q.mana_value(id, &q.chars(id)))) == (q.state().chosen_number(q.linked_to(asker.source)).map_or(0i64, i64::from))))); }

pub static CARD: CardDef = CardDef { name: "Haktos the Unscarred", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [2, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_0_apply_0, "Haktos attacks each combat if able."),static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_2_apply_0, "Haktos has protection from each mana value other than the chosen number.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
