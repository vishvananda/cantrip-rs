// Spare from Evil
// status: planned
// requires: engine: no protection from this quality
// Creatures you control gain protection from non-Human creatures until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-protection-from-this-quality" => #[allow(unused_variables)] fn protection_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Human))) && q.chars(id).is(CardType::Creature)) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; planned_value!("engine: no protection from this quality", EffectKind::Pump(0, 0) => EffectKind::ProtectionFrom(ProtectionQuality::Rule { card: { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, index: 0, about: Target::None })) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-protection-from-this-quality" => protection_rules: protection_rule_0);

pub static CARD: CardDef = CardDef { name: "Spare from Evil", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
