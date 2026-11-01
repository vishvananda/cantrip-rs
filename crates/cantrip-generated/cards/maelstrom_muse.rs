// Maelstrom Muse
// status: planned
// requires: operation EffectOn
// Flying
// Whenever this creature attacks, the next instant or sorcery spell you cast this turn costs {X} less to cast, where X is this creature's power as this ability resolves.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-effect-on" => cantrip_mechanics::effects::effect_on(g, asker, EffectOn::NextSpell { spells: |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)), caster: { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, this_turn: true, from: None }, &[SpellEffect::DecreaseManaCostX(&[CostReductionSymbolX::CostReduceX], { let q: &dyn Query = g; Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))) })], None)?);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Maelstrom Muse", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [Some((Mana::U, Mana::R)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Djinn,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, the next instant or sorcery spell you cast this turn costs {X} less to cast, where X is this creature's power as this ability resolves.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
