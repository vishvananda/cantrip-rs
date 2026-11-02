// Dragonlord's Prerogative
// status: planned
// requires: engine: no reveal cost
// As an additional cost to cast this spell, you may reveal a Dragon card from your hand.
// If you revealed a Dragon card or controlled a Dragon as you cast this spell, this spell can't be countered.
// Draw four cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Dragon)) }
#[allow(unused_variables)] fn ability_1_apply(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if (q.history().casts().any(|c| (c.spell == asker.source || q.state().obj(c.spell).became == Some(asker.source)) && c.fact != 0) || planned_value!("engine: no reveal cost", false => q.as_cast(asker, "AsCast { player: AbilityController, action: Revealed(Subtype(Dragon)) }"))) { chars.keywords = chars.keywords.with(Keyword::CantBeCountered); } }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (4i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn kept_as_cast(q: &dyn Query, asker: &Asker) -> u32 { (i64::from((q.state().battlefield().iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Dragon))).collect::<Vec<ObjectId>>()).into_iter().any(|o: ObjectId| q.controller(o) == asker.controller))).max(0) as u32 }

pub static CARD: CardDef = CardDef { name: "Dragonlord's Prerogative", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_2), abilities: &[], statics: &[characteristic_defining(Layer::Ability, ability_1_apply, "This spell can't be countered.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_fact: Some(kept_as_cast), labels: &["Cast without revealing", "Reveal a Dragon card from your hand"], optional_cost: Some(OptionalCost::new(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no reveal cost", Cost::NONE => Cost { reveal: Some((1, ability_0_0)), ..Cost::NONE })))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
