// Soulfire Grand Master
// status: planned
// requires: operation EffectOn
// Lifelink
// Instant and sorcery spells you control have lifelink.
// {2}{U/R}{U/R}: The next time you cast an instant or sorcery spell from your hand this turn, put that card into your hand instead of into your graveyard as it resolves.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(affected).became.is_none() && q.state().obj(affected).zone.kind() == ZoneKind::Stack) && (((*so_far).is(CardType::Instant) || (*so_far).is(CardType::Sorcery)) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Lifelink); }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-effect-on" => cantrip_mechanics::effects::effect_on(g, asker, EffectOn::NextSpell { spells: |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)), caster: { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, this_turn: true, from: Some("Hand") }, &[SpellEffect::ResolvesIntoHandInsteadOfGraveyard], None)?);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Soulfire Grand Master", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Monk]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::U, Mana::R)),Some((Mana::U, Mana::R)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{2}{U/R}{U/R}: The next time you cast an instant or sorcery spell from your hand this turn, put that card into your hand instead of into your graveyard as it resolves.").targets(&[])], statics: &[static_ability(Layer::Ability, ability_1_affects, ability_1_apply_0, "Instant and sorcery spells you control have lifelink.").everywhere()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
